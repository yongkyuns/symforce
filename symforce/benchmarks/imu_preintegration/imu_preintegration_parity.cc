/* ----------------------------------------------------------------------------
 * SymForce - Copyright 2022, Skydio, Inc.
 * This source code is under the Apache 2.0 license found in the LICENSE file.
 * ---------------------------------------------------------------------------- */

#include <cmath>
#include <cstdint>
#include <iomanip>
#include <iostream>

#include <Eigen/Core>

#include <sym/pose3.h>
#include <symforce/slam/imu_preintegration/imu_factor.h>
#include <symforce/slam/imu_preintegration/imu_preintegrator.h>

namespace {

class DeterministicRng {
 public:
  explicit DeterministicRng(uint64_t state) : state_{state} {}

  double Signed() {
    state_ = state_ * 6364136223846793005ULL + 1;
    const double unit = static_cast<double>(state_ >> 11) * (1.0 / 9007199254740992.0);
    return 2.0 * unit - 1.0;
  }

 private:
  uint64_t state_;
};

Eigen::Vector3d Vector(DeterministicRng& rng, double scale) {
  return Eigen::Vector3d{scale * rng.Signed(), scale * rng.Signed(), scale * rng.Signed()};
}

}  // namespace

int main() {
  constexpr int kCases = 12;
  constexpr int kMeasurements = 5;
  std::cout << std::setprecision(17);

  for (int case_index = 0; case_index < kCases; ++case_index) {
    DeterministicRng rng{0x4d595df4d0f33173ULL + static_cast<uint64_t>(case_index)};
    const Eigen::Vector3d accel_bias = Vector(rng, 0.5);
    const Eigen::Vector3d gyro_bias = Vector(rng, 0.2);
    const Eigen::Vector3d gravity{0.3 * rng.Signed(), 0.3 * rng.Signed(), -9.81};
    const sym::Vector3d zero = sym::Vector3d::Zero();
    const sym::Pose3d pose_i;
    sym::ImuPreintegrator<double> integrator(accel_bias, gyro_bias);

    for (int measurement = 0; measurement < kMeasurements; ++measurement) {
      const Eigen::Vector3d accel = Vector(rng, 4.0);
      const Eigen::Vector3d gyro = Vector(rng, 1.5);
      const Eigen::Vector3d accel_cov =
          Eigen::Vector3d::Constant(1.0e-4 + 2.0e-4 * std::abs(rng.Signed()));
      const Eigen::Vector3d gyro_cov =
          Eigen::Vector3d::Constant(1.0e-4 + 2.0e-4 * std::abs(rng.Signed()));
      const double dt = 5.0e-4 + 2.0e-3 * std::abs(rng.Signed());
      integrator.IntegrateMeasurement(accel, gyro, accel_cov, gyro_cov, dt, 1.0e-9);
    }

    const auto& delta = integrator.PreintegratedMeasurements().delta;
    const auto [pose_j, vel_j] = delta.RollForwardState(pose_i, zero, gravity);
    const auto factor = sym::ImuFactor<double>{integrator};
    Eigen::Matrix<double, 9, 1> residual;
    Eigen::Matrix<double, 9, 24> jacobian;
    Eigen::Matrix<double, 24, 24> hessian;
    Eigen::Matrix<double, 24, 1> rhs;
    factor(pose_i, zero, pose_j, vel_j, accel_bias, gyro_bias, gravity, 1.0e-9, &residual,
           &jacobian, &hessian, &rhs);

    std::cout << "case=" << case_index << " dp0=" << delta.Dp(0) << " dv1=" << delta.Dv(1)
              << " cov00=" << integrator.Covariance()(0, 0)
              << " dr_db00=" << integrator.PreintegratedMeasurements().DR_D_gyro_bias(0, 0)
              << " hessian00=" << hessian(0, 0) << " rhs0=" << rhs(0) << '\n';
  }
}
