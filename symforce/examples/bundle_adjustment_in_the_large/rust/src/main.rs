mod dataset;
mod generated;
mod metis_ordering;

use std::{env, mem::size_of};

use dataset::Problem;
use generated::snavely_reprojection_factor::sym::snavely_reprojection_factor;
use stack_algebra::{
    Cholesky, Matrix, StaticCscCholeskyPattern, StaticCscLdlt, StaticCscOrdering, StaticCscPattern,
    Vector,
};
use symforce_rust::Pose3;
use symforce_rust::{
    DynamicFactor, DynamicLinearization, DynamicOptimizationError, DynamicOptimizer, FactorIndex,
    OptimizerParams, StaticSparseCallbackWorkspace, StaticSparseLinearization,
    StaticSparseOptimizationError, StaticSparseOptimizer, Value, Values,
};

const EPSILON: f64 = 1e-10;
const BAL_CAMERAS: usize = 49;
const BAL_POINTS: usize = 7776;
const CAMERA_STATE_DIM: usize = 9;
const POINT_STATE_DIM: usize = 3;
const BAL_STATE_DIM: usize = BAL_CAMERAS * CAMERA_STATE_DIM + BAL_POINTS * POINT_STATE_DIM;
const BAL_CAMERA_STATE_DIM: usize = BAL_CAMERAS * CAMERA_STATE_DIM;
const BAL_HESSIAN_NNZ: usize = 908_622;
const BAL_FACTOR_NNZ: usize = 997_075;
const BAL_LOCAL_HESSIAN_NNZ: usize = 78;

fn pose_key(camera: usize) -> String {
    format!("cameras[{camera}].pose")
}

fn intrinsics_key(camera: usize) -> String {
    format!("cameras[{camera}].intrinsics")
}

fn point_key(point: usize) -> String {
    format!("points[{point}]")
}

struct ReprojectionFactor {
    pixel: Vector<2, f64>,
    pose_key: String,
    intrinsics_key: String,
    point_key: String,
}

impl ReprojectionFactor {
    fn new(camera: usize, point: usize, pixel: Vector<2, f64>) -> Self {
        Self {
            pixel,
            pose_key: pose_key(camera),
            intrinsics_key: intrinsics_key(camera),
            point_key: point_key(point),
        }
    }
}

#[derive(Clone)]
struct FastBalState {
    poses: Vec<Pose3<f64>>,
    intrinsics: Vec<Vector<3, f64>>,
    points: Vec<Vector<3, f64>>,
}

fn fast_bal_retract(state: &FastBalState, step: &Matrix<BAL_STATE_DIM, 1, f64>) -> FastBalState {
    let mut next = state.clone();
    for camera in 0..BAL_CAMERAS {
        let pose_offset = camera * 6;
        let delta = Matrix::from_rows([
            [step[pose_offset]],
            [step[pose_offset + 1]],
            [step[pose_offset + 2]],
            [step[pose_offset + 3]],
            [step[pose_offset + 4]],
            [step[pose_offset + 5]],
        ]);
        next.poses[camera] = next.poses[camera].retract(&delta, EPSILON);

        let intrinsics_offset = BAL_CAMERAS * 6 + camera * POINT_STATE_DIM;
        next.intrinsics[camera] = next.intrinsics[camera]
            + Matrix::from_rows([
                [step[intrinsics_offset]],
                [step[intrinsics_offset + 1]],
                [step[intrinsics_offset + 2]],
            ]);
    }
    for point in 0..BAL_POINTS {
        let offset = BAL_CAMERAS * CAMERA_STATE_DIM + point * POINT_STATE_DIM;
        next.points[point] = next.points[point]
            + Matrix::from_rows([[step[offset]], [step[offset + 1]], [step[offset + 2]]]);
    }
    next
}

fn static_state_columns(camera: usize, point: usize) -> [usize; 12] {
    let pose_offset = camera * 6;
    let intrinsics_offset = BAL_CAMERAS * 6 + camera * POINT_STATE_DIM;
    let point_offset = BAL_CAMERA_STATE_DIM + point * POINT_STATE_DIM;
    [
        pose_offset,
        pose_offset + 1,
        pose_offset + 2,
        pose_offset + 3,
        pose_offset + 4,
        pose_offset + 5,
        intrinsics_offset,
        intrinsics_offset + 1,
        intrinsics_offset + 2,
        point_offset,
        point_offset + 1,
        point_offset + 2,
    ]
}

fn static_hessian_indices(
    pattern: &StaticCscPattern<BAL_STATE_DIM, BAL_STATE_DIM, BAL_HESSIAN_NNZ>,
    columns: &[usize; 12],
) -> [u32; BAL_LOCAL_HESSIAN_NNZ] {
    let mut indices = [0; BAL_LOCAL_HESSIAN_NNZ];
    let mut index = 0;
    for column in 0..columns.len() {
        for row in column..columns.len() {
            let global_column = columns[column];
            let global_row = columns[row];
            indices[index] = pattern
                .entry_index(global_row.max(global_column), global_row.min(global_column))
                .expect("BAL Hessian pattern should contain every factor entry")
                .try_into()
                .expect("BAL Hessian index should fit in u32");
            index += 1;
        }
    }
    indices
}

fn static_hessian_pattern(
    problem: &Problem,
) -> Box<StaticCscPattern<BAL_STATE_DIM, BAL_STATE_DIM, BAL_HESSIAN_NNZ>> {
    let mut rows_by_column = vec![Vec::<usize>::new(); BAL_STATE_DIM];
    for observation in &problem.observations {
        let columns = static_state_columns(observation.camera, observation.point);
        for local_column in 0..columns.len() {
            let column = columns[local_column];
            for local_row in local_column..columns.len() {
                let row = columns[local_row];
                rows_by_column[column].push(row.max(column));
            }
        }
    }
    let mut nnz = 0;
    for rows in &mut rows_by_column {
        rows.sort_unstable();
        rows.dedup();
        nnz += rows.len();
    }
    let mut row_indices = Vec::with_capacity(nnz);
    let mut column_pointers = vec![0usize; BAL_STATE_DIM + 1];
    for column in 0..BAL_STATE_DIM {
        row_indices.extend_from_slice(&rows_by_column[column]);
        column_pointers[column + 1] = rows_by_column[column].len();
    }
    for column in 0..BAL_STATE_DIM {
        column_pointers[column + 1] += column_pointers[column];
    }
    assert_eq!(nnz, BAL_HESSIAN_NNZ);
    let mut output =
        Box::<StaticCscPattern<BAL_STATE_DIM, BAL_STATE_DIM, BAL_HESSIAN_NNZ>>::new_uninit();
    StaticCscPattern::from_arrays_into(&row_indices, &column_pointers, output.as_mut())
        .expect("BAL Hessian pattern should be canonical");
    // SAFETY: `from_arrays_into` initialized the complete pattern.
    unsafe { output.assume_init() }
}

struct FastBalFactor {
    camera: u32,
    point: u32,
    pixel: Vector<2, f64>,
    hessian_indices: [u32; BAL_LOCAL_HESSIAN_NNZ],
}

fn print_memory_profile(problem: &Problem) {
    let linearization_bytes =
        size_of::<StaticSparseLinearization<BAL_STATE_DIM, BAL_HESSIAN_NNZ, f64>>();
    let ordered_matrix_bytes = size_of::<
        stack_algebra::StaticCscMatrix<BAL_STATE_DIM, BAL_STATE_DIM, BAL_HESSIAN_NNZ, f64>,
    >();
    let factor_bytes = size_of::<StaticCscLdlt<BAL_STATE_DIM, BAL_FACTOR_NNZ, f64>>();
    let factor_pattern_bytes = size_of::<StaticCscCholeskyPattern<BAL_STATE_DIM, BAL_FACTOR_NNZ>>();
    let optimizer_bytes =
        size_of::<StaticSparseOptimizer<f64, BAL_STATE_DIM, BAL_HESSIAN_NNZ, BAL_FACTOR_NNZ>>();
    let factor_record_bytes = size_of::<FastBalFactor>();
    let factor_records = problem.observations.len() * factor_record_bytes;
    let optimizer_linearizations = 2 * linearization_bytes;
    let major_workspaces = optimizer_bytes
        + optimizer_linearizations
        + ordered_matrix_bytes
        + factor_bytes
        + factor_pattern_bytes;
    let mib = |bytes: usize| bytes as f64 / (1024.0 * 1024.0);

    eprintln!("BAL Rust memory profile (capacity-sized objects):");
    eprintln!("  optimizer inline: {:.1} MiB", mib(optimizer_bytes));
    eprintln!(
        "  two linearizations: {:.1} MiB",
        mib(optimizer_linearizations)
    );
    eprintln!("  ordered Hessian: {:.1} MiB", mib(ordered_matrix_bytes));
    eprintln!("  numeric factor: {:.1} MiB", mib(factor_bytes));
    eprintln!("  factor pattern: {:.1} MiB", mib(factor_pattern_bytes));
    eprintln!(
        "  factor records: {:.1} MiB ({} observations × {} bytes)",
        mib(factor_records),
        problem.observations.len(),
        factor_record_bytes
    );
    eprintln!(
        "  listed major workspaces: {:.1} MiB",
        mib(major_workspaces)
    );
}

/// Accumulates one prevalidated BAL factor without repeating generic sparse
/// dimension and pattern checks for every observation.
fn fast_bal_add_hessian_factor(
    residual: &Matrix<2, 1, f64>,
    hessian: &Matrix<12, 12, f64>,
    rhs: &Matrix<12, 1, f64>,
    factor: &FastBalFactor,
    linearization: &mut StaticSparseLinearization<BAL_STATE_DIM, BAL_HESSIAN_NNZ, f64>,
) {
    linearization.error += residual.squared_norm() / 2.0;
    let rhs_output = linearization.rhs.as_mut_slice().as_mut_ptr();
    let hessian_output = linearization.hessian.values_mut().as_mut_ptr();
    let rhs_input = rhs.as_slice().as_ptr();
    let hessian_input = hessian.as_slice().as_ptr();
    let hessian_nnz = linearization.hessian.nnz();
    let columns = static_state_columns(factor.camera as usize, factor.point as usize);

    // SAFETY: `factor.columns` and `factor.hessian_indices` are constructed
    // from the validated BAL pattern before optimization. Matrix slices are
    // fixed-size, and `hessian_indices` contains exactly the lower-triangular
    // entries in the same column-major order as this loop.
    unsafe {
        let mut local_index = 0;
        for local_column in 0..12 {
            *rhs_output.add(columns[local_column]) += *rhs_input.add(local_column);
            for local_row in local_column..12 {
                let target = factor.hessian_indices[local_index] as usize;
                debug_assert!(target < hessian_nnz);
                let source = local_row + local_column * 12;
                *hessian_output.add(target) += *hessian_input.add(source);
                local_index += 1;
            }
        }
    }
}

fn fast_bal_linearize(
    state: &FastBalState,
    factors: &[FastBalFactor],
    linearization: &mut StaticSparseLinearization<BAL_STATE_DIM, BAL_HESSIAN_NNZ, f64>,
) -> Result<(), StaticSparseOptimizationError> {
    // The generated factor overwrites every requested output on each call.
    // Keep these fixed-size temporaries outside the observation loop so their
    // stack storage and initialization are paid for once per linearization.
    let mut residual = Matrix::<2, 1, f64>::zeros();
    let mut hessian = Matrix::<12, 12, f64>::zeros();
    let mut rhs = Matrix::<12, 1, f64>::zeros();
    for factor in factors {
        snavely_reprojection_factor(
            &state.poses[factor.camera as usize],
            &state.intrinsics[factor.camera as usize],
            &state.points[factor.point as usize],
            &factor.pixel,
            EPSILON,
            Some(&mut residual),
            None,
            Some(&mut hessian),
            Some(&mut rhs),
        );
        fast_bal_add_hessian_factor(&residual, &hessian, &rhs, factor, linearization);
    }
    Ok(())
}

struct SchurFactor {
    camera: usize,
    point: usize,
    pixel: Vector<2, f64>,
}

struct SchurLinearization {
    error: f64,
    camera_hessian: Matrix<BAL_CAMERA_STATE_DIM, BAL_CAMERA_STATE_DIM, f64>,
    camera_rhs: Matrix<BAL_CAMERA_STATE_DIM, 1, f64>,
    point_hessian: Vec<Matrix<3, 3, f64>>,
    point_rhs: Vec<Matrix<3, 1, f64>>,
    camera_point: Vec<Matrix<9, 3, f64>>,
}

struct SchurBlock {
    camera: usize,
    a: Matrix<9, 3, f64>,
    w: Matrix<9, 3, f64>,
}

fn schur_factors(problem: &Problem) -> Vec<SchurFactor> {
    let mut factors = problem
        .observations
        .iter()
        .map(|observation| SchurFactor {
            camera: observation.camera,
            point: observation.point,
            pixel: observation.pixel,
        })
        .collect::<Vec<_>>();
    factors.sort_unstable_by_key(|factor| (factor.point, factor.camera));
    factors
}

fn schur_linearize(state: &FastBalState, factors: &[SchurFactor]) -> SchurLinearization {
    let mut linearization = SchurLinearization {
        error: 0.0,
        camera_hessian: Matrix::zeros(),
        camera_rhs: Matrix::zeros(),
        point_hessian: vec![Matrix::zeros(); BAL_POINTS],
        point_rhs: vec![Matrix::zeros(); BAL_POINTS],
        camera_point: vec![Matrix::zeros(); factors.len()],
    };

    for (factor_index, factor) in factors.iter().enumerate() {
        let mut residual = Matrix::<2, 1, f64>::zeros();
        let mut hessian = Matrix::<12, 12, f64>::zeros();
        let mut rhs = Matrix::<12, 1, f64>::zeros();
        snavely_reprojection_factor(
            &state.poses[factor.camera],
            &state.intrinsics[factor.camera],
            &state.points[factor.point],
            &factor.pixel,
            EPSILON,
            Some(&mut residual),
            None,
            Some(&mut hessian),
            Some(&mut rhs),
        );
        linearization.error +=
            0.5 * (residual[(0, 0)] * residual[(0, 0)] + residual[(1, 0)] * residual[(1, 0)]);

        let camera_offset = factor.camera * CAMERA_STATE_DIM;
        for row in 0..9 {
            linearization.camera_rhs[(camera_offset + row, 0)] += rhs[(row, 0)];
            for column in 0..=row {
                linearization.camera_hessian[(camera_offset + row, camera_offset + column)] +=
                    hessian[(row, column)];
            }
        }

        for row in 0..3 {
            linearization.point_rhs[factor.point][(row, 0)] += rhs[(9 + row, 0)];
            for column in 0..=row {
                linearization.point_hessian[factor.point][(row, column)] +=
                    hessian[(9 + row, 9 + column)];
            }
        }
        for row in 0..9 {
            for column in 0..3 {
                linearization.camera_point[factor_index][(row, column)] =
                    hessian[(9 + column, row)];
            }
        }
    }

    linearization
}

fn schur_add_block_product(
    matrix: &mut Matrix<BAL_CAMERA_STATE_DIM, BAL_CAMERA_STATE_DIM, f64>,
    row_camera: usize,
    column_camera: usize,
    row: &Matrix<9, 3, f64>,
    column: &Matrix<9, 3, f64>,
) {
    let row_offset = row_camera * CAMERA_STATE_DIM;
    let column_offset = column_camera * CAMERA_STATE_DIM;
    for local_row in 0..9 {
        for local_column in 0..9 {
            let global_row = row_offset + local_row;
            let global_column = column_offset + local_column;
            if global_row < global_column {
                continue;
            }
            let mut value = 0.0;
            for inner in 0..3 {
                value += row[(local_row, inner)] * column[(local_column, inner)];
            }
            matrix[(global_row, global_column)] -= value;
        }
    }
}

fn schur_solve(
    linearization: &SchurLinearization,
    factors: &[SchurFactor],
    lambda: f64,
) -> Option<Matrix<BAL_STATE_DIM, 1, f64>> {
    let mut schur = linearization.camera_hessian;
    let mut schur_rhs = linearization.camera_rhs;
    for index in 0..BAL_CAMERA_STATE_DIM {
        schur[(index, index)] += lambda;
    }

    let mut point_ranges = vec![0usize; BAL_POINTS + 1];
    for factor in factors {
        point_ranges[factor.point + 1] += 1;
    }
    for point in 0..BAL_POINTS {
        point_ranges[point + 1] += point_ranges[point];
    }

    let mut point_blocks = Vec::with_capacity(BAL_POINTS);
    for point in 0..BAL_POINTS {
        let start = point_ranges[point];
        let end = point_ranges[point + 1];
        let mut point_matrix = linearization.point_hessian[point];
        for index in 0..3 {
            point_matrix[(index, index)] += lambda;
        }
        let point_factor = Cholesky::<3, f64>::try_decompose(&point_matrix).ok()?;

        let mut blocks: Vec<SchurBlock> = Vec::with_capacity(end - start);
        for factor_index in start..end {
            let factor = &factors[factor_index];
            if let Some(block) = blocks.last_mut() {
                if block.camera == factor.camera {
                    for row in 0..9 {
                        for column in 0..3 {
                            block.a[(row, column)] +=
                                linearization.camera_point[factor_index][(row, column)];
                        }
                    }
                    continue;
                }
            }
            blocks.push(SchurBlock {
                camera: factor.camera,
                a: linearization.camera_point[factor_index],
                w: Matrix::zeros(),
            });
        }

        for block in &mut blocks {
            let solved = point_factor.solve(&block.a.transpose());
            block.w = solved.transpose();
            let camera_offset = block.camera * CAMERA_STATE_DIM;
            for row in 0..9 {
                let mut correction = 0.0;
                for inner in 0..3 {
                    correction +=
                        block.w[(row, inner)] * linearization.point_rhs[point][(inner, 0)];
                }
                schur_rhs[(camera_offset + row, 0)] -= correction;
            }
        }

        for row_block in 0..blocks.len() {
            for column_block in 0..=row_block {
                let (row, column) = (&blocks[row_block], &blocks[column_block]);
                if row.camera >= column.camera {
                    schur_add_block_product(
                        &mut schur,
                        row.camera,
                        column.camera,
                        &row.w,
                        &column.a,
                    );
                } else {
                    schur_add_block_product(
                        &mut schur,
                        column.camera,
                        row.camera,
                        &column.w,
                        &row.a,
                    );
                }
            }
        }
        point_blocks.push((point_factor, blocks));
    }

    let camera_factor = Cholesky::<BAL_CAMERA_STATE_DIM, f64>::try_decompose(&schur).ok()?;
    let camera_solution = camera_factor.solve(&schur_rhs);
    let mut step = Matrix::<BAL_STATE_DIM, 1, f64>::zeros();
    for camera in 0..BAL_CAMERAS {
        let packed_offset = camera * CAMERA_STATE_DIM;
        let pose_offset = camera * 6;
        for index in 0..6 {
            step[(pose_offset + index, 0)] = -camera_solution[(packed_offset + index, 0)];
        }
        let intrinsics_offset = BAL_CAMERAS * 6 + camera * POINT_STATE_DIM;
        for index in 0..3 {
            step[(intrinsics_offset + index, 0)] = -camera_solution[(packed_offset + 6 + index, 0)];
        }
    }
    for point in 0..BAL_POINTS {
        let mut rhs = linearization.point_rhs[point];
        for block in &point_blocks[point].1 {
            let camera_offset = block.camera * CAMERA_STATE_DIM;
            for column in 0..3 {
                let mut correction = 0.0;
                for row in 0..9 {
                    correction +=
                        block.a[(row, column)] * camera_solution[(camera_offset + row, 0)];
                }
                rhs[(column, 0)] -= correction;
            }
        }
        let solution = point_blocks[point].0.solve(&rhs);
        let point_offset = BAL_CAMERA_STATE_DIM + point * POINT_STATE_DIM;
        for index in 0..3 {
            step[(point_offset + index, 0)] = -solution[(index, 0)];
        }
    }
    Some(step)
}

fn schur_error(state: &FastBalState, factors: &[SchurFactor]) -> f64 {
    let mut error = 0.0;
    for factor in factors {
        let mut residual = Matrix::<2, 1, f64>::zeros();
        snavely_reprojection_factor(
            &state.poses[factor.camera],
            &state.intrinsics[factor.camera],
            &state.points[factor.point],
            &factor.pixel,
            EPSILON,
            Some(&mut residual),
            None,
            None,
            None,
        );
        error += 0.5 * (residual[(0, 0)] * residual[(0, 0)] + residual[(1, 0)] * residual[(1, 0)]);
    }
    error
}

fn run_bal_schur(problem: &Problem) -> Result<(f64, usize), String> {
    let factors = schur_factors(problem);
    let mut state = FastBalState {
        poses: problem.cameras.iter().map(|camera| camera.pose()).collect(),
        intrinsics: problem
            .cameras
            .iter()
            .map(|camera| camera.intrinsics)
            .collect(),
        points: problem.points.clone(),
    };
    let mut current = schur_linearize(&state, &factors);
    let mut lambda = 1.0;
    let mut nu = 2.0;
    let mut iterations = 0;
    for iteration in 0..50 {
        iterations = iteration + 1;
        let step = match schur_solve(&current, &factors, lambda) {
            Some(step) => step,
            None => {
                lambda *= 10.0;
                continue;
            }
        };
        let mut linear_term = 0.0;
        for camera in 0..BAL_CAMERAS {
            let packed_offset = camera * CAMERA_STATE_DIM;
            let pose_offset = camera * 6;
            for index in 0..6 {
                let value = step[(pose_offset + index, 0)];
                linear_term +=
                    value * (current.camera_rhs[(packed_offset + index, 0)] - lambda * value);
            }
            let intrinsics_offset = BAL_CAMERAS * 6 + camera * POINT_STATE_DIM;
            for index in 0..3 {
                let value = step[(intrinsics_offset + index, 0)];
                linear_term +=
                    value * (current.camera_rhs[(packed_offset + 6 + index, 0)] - lambda * value);
            }
        }
        for point in 0..BAL_POINTS {
            let offset = BAL_CAMERA_STATE_DIM + point * POINT_STATE_DIM;
            for index in 0..POINT_STATE_DIM {
                let value = step[(offset + index, 0)];
                linear_term += value * (current.point_rhs[point][(index, 0)] - lambda * value);
            }
        }
        let linear_error = current.error + 0.5 * linear_term;
        let candidate_state = fast_bal_retract(&state, &step);
        let candidate_error = schur_error(&candidate_state, &factors);
        let relative_reduction = (current.error - candidate_error) / (current.error + EPSILON);
        let gain_ratio = (current.error - candidate_error) / (current.error - linear_error);
        if candidate_error < current.error {
            state = candidate_state;
            current = schur_linearize(&state, &factors);
            let multiplier =
                (1.0_f64 / 3.0).max(1.0 - (2.0 - 1.0) * (2.0 * gain_ratio - 1.0).powi(3));
            lambda *= multiplier;
            nu = 2.0;
            if step.squared_norm() < 1e-12 {
                break;
            }
        } else {
            lambda *= nu;
            nu *= 2.0;
        }
        if relative_reduction > -1e-7 && relative_reduction < 1e-6 {
            break;
        }
    }
    Ok((current.error, iterations))
}

fn run_bal_sparse(problem: &Problem) -> Result<(f64, usize), String> {
    let ordering = StaticCscOrdering::from_permutation(&metis_ordering::BAL_METIS_ORDERING)
        .map_err(|error| format!("invalid BAL ordering: {error:?}"))?;
    let pattern = static_hessian_pattern(problem);
    let mut factors = Vec::with_capacity(problem.observations.len());
    for observation in &problem.observations {
        let columns = static_state_columns(observation.camera, observation.point);
        factors.push(FastBalFactor {
            camera: observation
                .camera
                .try_into()
                .expect("BAL camera index should fit in u32"),
            point: observation
                .point
                .try_into()
                .expect("BAL point index should fit in u32"),
            pixel: observation.pixel,
            hessian_indices: static_hessian_indices(&pattern, &columns),
        });
    }
    let mut optimizer = Box::<
        StaticSparseOptimizer<f64, BAL_STATE_DIM, BAL_HESSIAN_NNZ, BAL_FACTOR_NNZ>,
    >::new_uninit();
    StaticSparseOptimizer::new_with_ordering_ref_into(
        OptimizerParams {
            initial_lambda: 1.0,
            lambda_down_factor: 0.1,
            lambda_up_factor: 10.0,
            max_iterations: 50,
            early_exit_min_reduction: 1e-6,
            step_tolerance: 1e-12,
        },
        &pattern,
        ordering,
        optimizer.as_mut(),
    );
    // SAFETY: `new_with_ordering_into` initializes the optimizer before it is used.
    let optimizer = unsafe { optimizer.assume_init() };
    drop(pattern);
    let initial_state = FastBalState {
        poses: problem.cameras.iter().map(|camera| camera.pose()).collect(),
        intrinsics: problem
            .cameras
            .iter()
            .map(|camera| camera.intrinsics)
            .collect(),
        points: problem.points.clone(),
    };
    let mut workspace = StaticSparseCallbackWorkspace::new(&optimizer);
    let result = optimizer
        .optimize_with_callbacks_with_workspace(
            initial_state,
            EPSILON,
            &mut workspace,
            |state, linearization| fast_bal_linearize(state, &factors, linearization),
            |state, step| Ok(fast_bal_retract(state, step)),
        )
        .map_err(|error| format!("sparse optimization failed: {error:?}"))?;
    Ok((result.error, result.iterations))
}

impl DynamicFactor<f64> for ReprojectionFactor {
    fn linearize(
        &self,
        values: &Values<f64>,
        state_index: &symforce_rust::StateIndex,
        linearization: &mut DynamicLinearization<f64>,
    ) -> Result<(), DynamicOptimizationError> {
        let pose = values.pose3(&self.pose_key)?;
        let intrinsics = values.vector3(&self.intrinsics_key)?;
        let point = values.vector3(&self.point_key)?;
        let mut residual = Matrix::<2, 1, f64>::zeros();
        let mut jacobian = Matrix::<2, 12, f64>::zeros();
        snavely_reprojection_factor(
            pose,
            intrinsics,
            point,
            &self.pixel,
            EPSILON,
            Some(&mut residual),
            Some(&mut jacobian),
            None,
            None,
        );
        let columns = FactorIndex::from_state_index(
            state_index,
            &[
                self.pose_key.clone(),
                self.intrinsics_key.clone(),
                self.point_key.clone(),
            ],
        )?;
        linearization.add_factor(&residual, &jacobian, columns.columns())?;
        Ok(())
    }
}

fn initial_values(problem: &Problem) -> Values<f64> {
    let mut values = Values::new();
    for (index, camera) in problem.cameras.iter().enumerate() {
        values.insert(pose_key(index), Value::Pose3(camera.pose()));
        values.insert(intrinsics_key(index), Value::Vector3(camera.intrinsics));
    }
    for (index, point) in problem.points.iter().enumerate() {
        values.insert(point_key(index), Value::Vector3(*point));
    }
    values
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let path = args
        .next()
        .ok_or_else(|| "usage: ... <bal-file> [max-cameras] [max-points]".to_owned())?;
    let max_cameras = args
        .next()
        .map(|value| value.parse::<usize>().map_err(|error| error.to_string()))
        .transpose()?
        .unwrap_or(BAL_CAMERAS);
    let max_points = args
        .next()
        .map(|value| value.parse::<usize>().map_err(|error| error.to_string()))
        .transpose()?
        .unwrap_or(BAL_POINTS);
    let problem = Problem::read(path)?.subset(max_cameras, max_points);
    if problem.observations.is_empty() {
        return Err("selected BAL subset contains no observations".to_owned());
    }
    if env::var_os("BAL_MEMORY_PROFILE").is_some() {
        print_memory_profile(&problem);
    }

    if problem.cameras.len() == BAL_CAMERAS && problem.points.len() == BAL_POINTS {
        let (error, iterations) = if env::var("BAL_SOLVER").as_deref() == Ok("sparse") {
            run_bal_sparse(&problem)?
        } else {
            run_bal_schur(&problem)?
        };
        println!(
            "cameras: {}, points: {}, observations: {}",
            problem.cameras.len(),
            problem.points.len(),
            problem.observations.len()
        );
        println!("iterations: {}", iterations);
        println!("final error: {:.12}", error);
        return Ok(());
    }

    let mut factors: Vec<Box<dyn DynamicFactor<f64>>> = Vec::new();
    for observation in &problem.observations {
        factors.push(Box::new(ReprojectionFactor::new(
            observation.camera,
            observation.point,
            observation.pixel,
        )));
    }
    let mut optimized_keys = Vec::new();
    for camera in 0..problem.cameras.len() {
        optimized_keys.push(pose_key(camera));
        optimized_keys.push(intrinsics_key(camera));
    }
    for point in 0..problem.points.len() {
        optimized_keys.push(point_key(point));
    }

    let optimizer = DynamicOptimizer::new_symforce(OptimizerParams {
        initial_lambda: 1.0,
        lambda_down_factor: 0.1,
        lambda_up_factor: 10.0,
        max_iterations: 50,
        early_exit_min_reduction: 1e-6,
        step_tolerance: 1e-12,
    });
    let result = optimizer
        .optimize(initial_values(&problem), &optimized_keys, &factors, EPSILON)
        .map_err(|error| format!("optimization failed: {error:?}"))?;
    println!(
        "cameras: {}, points: {}, observations: {}",
        problem.cameras.len(),
        problem.points.len(),
        problem.observations.len()
    );
    println!("iterations: {}", result.iterations);
    println!("final error: {:.12}", result.error);
    Ok(())
}

fn main() {
    std::thread::Builder::new()
        .name("bal-static".to_owned())
        .stack_size(768 * 1024 * 1024)
        .spawn(run)
        .expect("BAL worker thread should start")
        .join()
        .expect("BAL worker thread should not panic")
        .expect("BAL optimization should succeed");
}
