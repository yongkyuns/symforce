//! Bundle-adjustment dataset exported from the C++ example's `BuildValues`.
//!
//! The values below were dumped with seed 42 from the C++ implementation. Keeping this as a
//! literal dataset makes the Rust path exercise the same observations, initialization, priors,
//! weights, and calibration as the reference implementation.

use stack_algebra::{Matrix, Vector};
use symforce_rust::{LinearCameraCal, Pose3};

pub const NUM_VIEWS: usize = 2;
pub const NUM_LANDMARKS: usize = 20;
pub const EPSILON: f64 = 1e-10;
pub const GNC_SCALE: f64 = 10.0;
pub const GNC_MU: f64 = 0.0;

const VIEWS: [[f64; 7]; NUM_VIEWS] = [
    [
        0.41218259669455154,
        0.18320077154392883,
        -0.87700711132923204,
        0.1655340175486667,
        2.1416584899523161,
        -1.1978063100887117,
        -0.92882812166838824,
    ],
    [
        0.45007163753320473,
        0.23606887433371249,
        -0.85016859680711288,
        0.13755131646777718,
        0.4337764347175006,
        -1.3736855917023418,
        -0.61771491881623741,
    ],
];

const CALIBRATIONS: [[f64; 4]; NUM_VIEWS] = [[740.0, 740.0, 639.5, 359.5]; NUM_VIEWS];

const RELATIVE_PRIOR: [f64; 7] = [
    0.033006088511260349,
    -0.28353690741099746,
    -0.054699977687069565,
    0.95683087988318416,
    0.88950579268957208,
    -0.058944199609359427,
    0.95245202177499122,
];

const LANDMARKS: [f64; NUM_LANDMARKS] = [
    0.021177625421905495,
    0.019346004428788196,
    0.068274967316610174,
    0.12459132193618576,
    0.15178338444993225,
    0.20298952278162863,
    0.04586721099946528,
    0.15295777634885521,
    0.085162455755469227,
    0.077230374003418284,
    0.030993523992566767,
    0.082475102727713481,
    0.05342332876972896,
    0.023508408478815742,
    0.11054236753499799,
    0.066050864713661922,
    0.10996634243955603,
    0.091017282806495695,
    0.022719633958520613,
    0.072027202196005488,
];

const SOURCE_PIXELS: [[f64; 2]; NUM_LANDMARKS] = [
    [750.0, 550.0],
    [950.0, 150.0],
    [50.0, 450.0],
    [750.0, 450.0],
    [150.0, 350.0],
    [350.0, 450.0],
    [1150.0, 150.0],
    [650.0, 350.0],
    [50.0, 550.0],
    [150.0, 550.0],
    [950.0, 650.0],
    [350.0, 350.0],
    [250.0, 650.0],
    [950.0, 250.0],
    [250.0, 150.0],
    [1050.0, 450.0],
    [1250.0, 250.0],
    [850.0, 450.0],
    [150.0, 150.0],
    [150.0, 650.0],
];

const TARGET_PIXELS: [[f64; 2]; NUM_LANDMARKS] = [
    [592.77812786187246, 513.45258467326585],
    [830.5486056511977, 111.63092431014454],
    [-320.99441325621825, 363.52962278921854],
    [535.10591623904384, 450.52676084215454],
    [-337.62818456383235, 269.44427765246559],
    [-577.84213180463598, 618.44478224344368],
    [1014.1259270335029, 144.41682331285205],
    [340.51780367392098, 352.03270486785874],
    [-408.17694403104326, 517.73912133611361],
    [-420.80821392572688, 593.90177759511141],
    [769.45960730261061, 613.65510172823383],
    [93.770112555798775, 272.80777460462963],
    [-39.139162983471145, 616.72681452613131],
    [812.63899339801696, 224.55897756862564],
    [-347.45410357329399, -16.642776323219213],
    [885.59764989646067, 435.82713765124629],
    [1090.7628143507732, 261.18830855866787],
    [645.9785330540933, 469.08725019800465],
    [-125.70594006063254, -5.533322864161013],
    [-179.84967533865424, 612.87823667017153],
];

const MATCH_WEIGHTS: [f64; NUM_LANDMARKS] = [
    1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0,
    0.0,
];

const LANDMARK_PRIORS: [f64; NUM_LANDMARKS] = [
    0.034220695862150881,
    0.038692008857576392,
    0.044747611005534219,
    0.1112398368666222,
    0.10480116278612629,
    0.27686077069677628,
    0.045365403525909183,
    0.18019648780239755,
    0.068578855690143631,
    0.12426561856218379,
    0.036854811901256619,
    0.064215017855032031,
    0.055586194955135268,
    0.046239156230660293,
    0.15793758838946287,
    0.052297212901547131,
    0.057651408624708897,
    0.12369832534511561,
    0.035176000874370117,
    0.05271814683923131,
];

pub fn pose(index: usize) -> Pose3<f64> {
    Pose3::from_storage(Vector::from_rows([
        [VIEWS[index][0]],
        [VIEWS[index][1]],
        [VIEWS[index][2]],
        [VIEWS[index][3]],
        [VIEWS[index][4]],
        [VIEWS[index][5]],
        [VIEWS[index][6]],
    ]))
}

pub fn calibration(index: usize) -> LinearCameraCal<f64> {
    LinearCameraCal::from_storage(Matrix::from_rows([
        [CALIBRATIONS[index][0]],
        [CALIBRATIONS[index][1]],
        [CALIBRATIONS[index][2]],
        [CALIBRATIONS[index][3]],
    ]))
}

pub fn prior_pose(i: usize, j: usize) -> Pose3<f64> {
    if i == 0 && j == 1 {
        Pose3::from_storage(Vector::from_rows([
            [RELATIVE_PRIOR[0]],
            [RELATIVE_PRIOR[1]],
            [RELATIVE_PRIOR[2]],
            [RELATIVE_PRIOR[3]],
            [RELATIVE_PRIOR[4]],
            [RELATIVE_PRIOR[5]],
            [RELATIVE_PRIOR[6]],
        ]))
    } else {
        Pose3::identity()
    }
}

pub fn prior_info(i: usize, j: usize) -> Matrix<6, 6, f64> {
    if i == 0 && j == 1 {
        Matrix::from_rows([
            [10.0 / 3.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 10.0 / 3.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 10.0 / 3.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 10.0 / 3.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 10.0 / 3.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 10.0 / 3.0],
        ])
    } else {
        Matrix::zeros()
    }
}

pub fn landmark(index: usize) -> f64 {
    LANDMARKS[index]
}

pub fn source_pixel(index: usize) -> Matrix<2, 1, f64> {
    Matrix::from_rows([[SOURCE_PIXELS[index][0]], [SOURCE_PIXELS[index][1]]])
}

pub fn target_pixel(index: usize) -> Matrix<2, 1, f64> {
    Matrix::from_rows([[TARGET_PIXELS[index][0]], [TARGET_PIXELS[index][1]]])
}

pub fn match_weight(index: usize) -> f64 {
    MATCH_WEIGHTS[index]
}

pub fn landmark_prior(index: usize) -> f64 {
    LANDMARK_PRIORS[index]
}
