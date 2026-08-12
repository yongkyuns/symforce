//! Runtime parser for Bundle-Adjustment-in-the-Large problem files.

use std::fs;
use std::path::Path;

use stack_algebra::Vector;
use symforce_rust::{Pose3, Rot3};

fn next_token<'a>(tokens: &mut std::str::SplitWhitespace<'a>) -> Result<&'a str, String> {
    tokens
        .next()
        .ok_or_else(|| "unexpected end of BAL file".to_owned())
}

fn parse_usize(tokens: &mut std::str::SplitWhitespace<'_>) -> Result<usize, String> {
    next_token(tokens)?
        .parse::<usize>()
        .map_err(|error| error.to_string())
}

fn parse_f64(tokens: &mut std::str::SplitWhitespace<'_>) -> Result<f64, String> {
    next_token(tokens)?
        .parse::<f64>()
        .map_err(|error| error.to_string())
}

/// One observed feature correspondence.
#[derive(Clone, Copy, Debug)]
pub struct Observation {
    /// Camera index.
    pub camera: usize,
    /// Point index.
    pub point: usize,
    /// Measured pixel coordinates.
    pub pixel: Vector<2, f64>,
}

/// One BAL camera, including its initial pose and intrinsics.
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    rotation: Vector<3, f64>,
    translation: Vector<3, f64>,
    /// BAL intrinsics `(f, k1, k2)`.
    pub intrinsics: Vector<3, f64>,
}

impl Camera {
    /// Returns the BAL world-to-camera pose.
    pub fn pose(&self) -> Pose3<f64> {
        Pose3::new(Rot3::from_tangent(&self.rotation, 1e-10), self.translation)
    }
}

/// A parsed BAL problem.
#[derive(Clone, Debug)]
pub struct Problem {
    /// All feature observations.
    pub observations: Vec<Observation>,
    /// Initial camera values.
    pub cameras: Vec<Camera>,
    /// Initial 3D points.
    pub points: Vec<Vector<3, f64>>,
}

impl Problem {
    /// Reads the standard BAL text format.
    pub fn read(path: impl AsRef<Path>) -> Result<Self, String> {
        let text = fs::read_to_string(path.as_ref())
            .map_err(|error| format!("failed to read {}: {error}", path.as_ref().display()))?;
        let mut tokens = text.split_whitespace();

        let num_cameras = parse_usize(&mut tokens)?;
        let num_points = parse_usize(&mut tokens)?;
        let num_observations = parse_usize(&mut tokens)?;
        let mut observations = Vec::with_capacity(num_observations);
        for _ in 0..num_observations {
            let camera = parse_usize(&mut tokens)?;
            let point = parse_usize(&mut tokens)?;
            observations.push(Observation {
                camera,
                point,
                pixel: Vector::from_rows([[parse_f64(&mut tokens)?], [parse_f64(&mut tokens)?]]),
            });
        }

        let mut cameras = Vec::with_capacity(num_cameras);
        for _ in 0..num_cameras {
            let rotation = Vector::from_rows([
                [parse_f64(&mut tokens)?],
                [parse_f64(&mut tokens)?],
                [parse_f64(&mut tokens)?],
            ]);
            let translation = Vector::from_rows([
                [parse_f64(&mut tokens)?],
                [parse_f64(&mut tokens)?],
                [parse_f64(&mut tokens)?],
            ]);
            cameras.push(Camera {
                rotation,
                translation,
                intrinsics: Vector::from_rows([
                    [parse_f64(&mut tokens)?],
                    [parse_f64(&mut tokens)?],
                    [parse_f64(&mut tokens)?],
                ]),
            });
        }

        let mut points = Vec::with_capacity(num_points);
        for _ in 0..num_points {
            points.push(Vector::from_rows([
                [parse_f64(&mut tokens)?],
                [parse_f64(&mut tokens)?],
                [parse_f64(&mut tokens)?],
            ]));
        }

        Ok(Self {
            observations,
            cameras,
            points,
        })
    }

    /// Returns an induced subset containing the first cameras and points.
    pub fn subset(&self, max_cameras: usize, max_points: usize) -> Self {
        let camera_count = max_cameras.min(self.cameras.len());
        let point_count = max_points.min(self.points.len());
        Self {
            observations: self
                .observations
                .iter()
                .copied()
                .filter(|observation| {
                    observation.camera < camera_count && observation.point < point_count
                })
                .collect(),
            cameras: self.cameras[..camera_count].to_vec(),
            points: self.points[..point_count].to_vec(),
        }
    }
}
