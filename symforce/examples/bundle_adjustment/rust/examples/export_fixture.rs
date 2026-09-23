//! Export the actual checked-in BA inputs for cross-language qualification.
//! Both Rust BA examples share this dataset; the C++ reference consumes this stream.

#[path = "../src/dataset.rs"]
mod dataset;

use stack_algebra::Matrix;
use std::io::{self, Write};

fn matrix<const R: usize, const C: usize>(
    output: &mut impl Write,
    value: &Matrix<R, C, f64>,
) -> io::Result<()> {
    for row in 0..R {
        for column in 0..C {
            write!(output, "{:.17e} ", value[(row, column)])?;
        }
    }
    writeln!(output)
}

fn main() -> io::Result<()> {
    let mut output = io::BufWriter::new(io::stdout().lock());
    writeln!(
        output,
        "SYMFORCE_BA_V1 {} {}",
        dataset::NUM_VIEWS,
        dataset::NUM_LANDMARKS
    )?;
    writeln!(
        output,
        "{:.17e} {:.17e} {:.17e}",
        dataset::EPSILON,
        dataset::GNC_MU,
        dataset::GNC_SCALE
    )?;
    for view in 0..dataset::NUM_VIEWS {
        matrix(&mut output, dataset::pose(view).data())?;
        matrix(&mut output, dataset::calibration(view).data())?;
    }
    for source in 0..dataset::NUM_VIEWS {
        for target in 0..dataset::NUM_VIEWS {
            matrix(&mut output, dataset::prior_pose(source, target).data())?;
            matrix(&mut output, &dataset::prior_info(source, target))?;
        }
    }
    for landmark in 0..dataset::NUM_LANDMARKS {
        writeln!(
            output,
            "{:.17e} {:.17e} {:.17e} {:.17e}",
            dataset::landmark(landmark),
            dataset::landmark_prior(landmark),
            dataset::match_weight(landmark),
            100.0_f64
        )?;
        matrix(&mut output, &dataset::source_pixel(landmark))?;
        matrix(&mut output, &dataset::target_pixel(landmark))?;
    }
    output.flush()
}
