/* ----------------------------------------------------------------------------
 * SymForce - Copyright 2022, Skydio, Inc.
 * This source code is under the Apache 2.0 license found in the LICENSE file.
 * ---------------------------------------------------------------------------- */

#include <cmath>
#include <cstdint>
#include <iomanip>
#include <iostream>
#include <string>

#include <Eigen/Core>

#include <sym/pose3.h>
#include <sym/unit3.h>
#include <symforce/slam/imu_preintegration/imu_factor.h>
#include <symforce/slam/imu_preintegration/imu_preintegrator.h>

// Protocol: scalar case field rows columns values-in-row-major-order.
// Keep inputs and protocol synchronized with rust/symforce/examples/imu_parity.rs.
namespace {

struct Rng {
  std::uint64_t state;
  double Signed() {
    state = state * 6364136223846793005ULL + 1;
    return 2.0 * (static_cast<double>(state >> 11) / 9007199254740992.0) - 1.0;
  }
};

template <typename Scalar>
Eigen::Matrix<Scalar, 3, 1> Vector(Rng& rng, double scale) {
  // Separate statements make RNG consumption independent of argument evaluation order.
  const Scalar x = static_cast<Scalar>(scale * rng.Signed());
  const Scalar y = static_cast<Scalar>(scale * rng.Signed());
  const Scalar z = static_cast<Scalar>(scale * rng.Signed());
  return {x, y, z};
}

template <typename Derived>
void Emit(const char* scalar, int test_case, const std::string& field,
          const Eigen::MatrixBase<Derived>& matrix) {
  std::cout << scalar << " " << test_case << " " << field << " " << matrix.rows() << " "
            << matrix.cols();
  for (int row = 0; row < matrix.rows(); ++row) {
    for (int col = 0; col < matrix.cols(); ++col) {
      std::cout << " " << static_cast<double>(matrix(row, col));
    }
  }
  std::cout << "\n";
}

template <int Dim, typename Scalar, typename Factor, typename... Args>
void Evaluate(const char* scalar, int test_case, const std::string& prefix, const Factor& factor,
              const Args&... args) {
  Eigen::Matrix<Scalar, 9, 1> residual = Eigen::Matrix<Scalar, 9, 1>::Zero();
  Eigen::Matrix<Scalar, 9, Dim> jacobian = Eigen::Matrix<Scalar, 9, Dim>::Zero();
  Eigen::Matrix<Scalar, Dim, Dim> hessian = Eigen::Matrix<Scalar, Dim, Dim>::Zero();
  Eigen::Matrix<Scalar, Dim, 1> rhs = Eigen::Matrix<Scalar, Dim, 1>::Zero();
  factor(args..., &residual, &jacobian, &hessian, &rhs);
  // SymForce's generated Hessian contract defines its lower triangle. Canonicalize
  // the symmetric matrix rather than reading an unspecified upper triangle.
  for (int row = 0; row < Dim; ++row) {
    for (int col = row + 1; col < Dim; ++col) {
      hessian(row, col) = hessian(col, row);
    }
  }
  Emit(scalar, test_case, prefix + ".residual", residual);
  Emit(scalar, test_case, prefix + ".jacobian", jacobian);
  Emit(scalar, test_case, prefix + ".hessian", hessian);
  Emit(scalar, test_case, prefix + ".rhs", rhs);
}

template <typename Scalar>
void Run(const char* scalar, Scalar epsilon) {
  using Vector3 = Eigen::Matrix<Scalar, 3, 1>;
  for (int test_case = 0; test_case < 12; ++test_case) {
    Rng rng{0x4d595df4d0f33173ULL + static_cast<std::uint64_t>(test_case)};
    const Vector3 accel_bias = Vector<Scalar>(rng, 0.5);
    const Vector3 gyro_bias = Vector<Scalar>(rng, 0.2);
    Vector3 gravity = Vector<Scalar>(rng, 0.3);
    gravity[2] = static_cast<Scalar>(-9.81);
    sym::ImuPreintegrator<Scalar> integrator(accel_bias, gyro_bias);
    for (int sample = 0; sample < 20 + test_case; ++sample) {
      const Vector3 accel = Vector<Scalar>(rng, 4.0);
      const Vector3 gyro = Vector<Scalar>(rng, 1.5);
      Vector3 accel_cov;
      Vector3 gyro_cov;
      for (int axis = 0; axis < 3; ++axis) {
        accel_cov[axis] = static_cast<Scalar>(1e-3 + 2e-3 * std::abs(rng.Signed()));
        gyro_cov[axis] = static_cast<Scalar>(1e-3 + 2e-3 * std::abs(rng.Signed()));
      }
      const Scalar dt = static_cast<Scalar>(0.005 + 0.01 * std::abs(rng.Signed()));
      integrator.IntegrateMeasurement(accel, gyro, accel_cov, gyro_cov, dt, epsilon);
    }
    const auto& measurement = integrator.PreintegratedMeasurements();
    Eigen::Matrix<Scalar, 62, 1> storage;
    measurement.ToStorage(storage.data());
    Emit(scalar, test_case, "measurement", storage);
    Emit(scalar, test_case, "covariance", integrator.Covariance());

    const sym::Pose3<Scalar> pose_i = sym::Pose3<Scalar>::Identity();
    const Vector3 vel_i{Scalar(0.4), Scalar(-0.2), Scalar(0.1)};
    const auto prediction = measurement.delta.RollForwardState(pose_i, vel_i, gravity);
    Emit(scalar, test_case, "pose", prediction.first.Data());
    Emit(scalar, test_case, "velocity", prediction.second);

    // Nonzero state and bias discrepancies exercise residuals and RHS, not just
    // the near-zero residual obtained by evaluating the rolled-forward state.
    auto pose_j = prediction.first;
    pose_j.Data()[4] += Scalar(0.02);
    pose_j.Data()[5] -= Scalar(0.03);
    pose_j.Data()[6] += Scalar(0.01);
    const Vector3 vel_j = prediction.second + Vector3{Scalar(0.1), Scalar(-0.05), Scalar(0.02)};
    const Vector3 eval_accel_bias = accel_bias + Vector3{Scalar(0.01), Scalar(-0.02), Scalar(0.03)};
    const Vector3 eval_gyro_bias =
        gyro_bias + Vector3{Scalar(0.001), Scalar(0.002), Scalar(-0.003)};

    // A deliberately non-diagonal square-root information matrix broadens factor coverage beyond
    // the covariance-derived factor path. This is also the matrix used by the fresh Rust generator
    // contract, so a disagreement there can be attributed to generator/runtime implementation.
    Eigen::Matrix<Scalar, 9, 9> manual_sqrt_info = Eigen::Matrix<Scalar, 9, 9>::Zero();
    for (int row = 0; row < 9; ++row) {
      manual_sqrt_info(row, row) = Scalar(1) + Scalar(0.2) * Scalar(row);
      for (int col = 0; col < row; ++col) {
        manual_sqrt_info(row, col) = Scalar(0.01) * Scalar(row + col + 1);
      }
    }

    Evaluate<24, Scalar>(scalar, test_case, "imu", sym::ImuFactor<Scalar>(integrator), pose_i,
                         vel_i, pose_j, vel_j, eval_accel_bias, eval_gyro_bias, gravity, epsilon);
    Evaluate<27, Scalar>(scalar, test_case, "gravity",
                         sym::ImuWithGravityFactor<Scalar>(integrator), pose_i, vel_i, pose_j,
                         vel_j, eval_accel_bias, eval_gyro_bias, gravity, epsilon);
    const Scalar gravity_norm = gravity.norm();
    const sym::Unit3<Scalar> direction = sym::Unit3<Scalar>::FromUnitVector(gravity / gravity_norm);
    Evaluate<26, Scalar>(scalar, test_case, "direction",
                         sym::ImuWithGravityDirectionFactor<Scalar>(integrator), pose_i, vel_i,
                         pose_j, vel_j, eval_accel_bias, eval_gyro_bias, direction, gravity_norm,
                         epsilon);
    Evaluate<24, Scalar>(scalar, test_case, "manual_imu",
                         sym::ImuFactor<Scalar>(measurement, manual_sqrt_info), pose_i, vel_i,
                         pose_j, vel_j, eval_accel_bias, eval_gyro_bias, gravity, epsilon);
    Evaluate<27, Scalar>(scalar, test_case, "manual_gravity",
                         sym::ImuWithGravityFactor<Scalar>(measurement, manual_sqrt_info), pose_i,
                         vel_i, pose_j, vel_j, eval_accel_bias, eval_gyro_bias, gravity, epsilon);
    Evaluate<26, Scalar>(scalar, test_case, "manual_direction",
                         sym::ImuWithGravityDirectionFactor<Scalar>(measurement, manual_sqrt_info),
                         pose_i, vel_i, pose_j, vel_j, eval_accel_bias, eval_gyro_bias, direction,
                         gravity_norm, epsilon);
  }
}
}  // namespace

int main() {
  std::cout << std::scientific << std::setprecision(17);
  Run<float>("f32", 1e-6F);
  Run<double>("f64", 1e-9);
}
