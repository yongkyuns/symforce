/* ----------------------------------------------------------------------------
 * SymForce - Copyright 2022, Skydio, Inc.
 * This source code is under the Apache 2.0 license found in the LICENSE file.
 * ---------------------------------------------------------------------------- */

#include <chrono>
#include <iomanip>
#include <iostream>
#include <type_traits>

#include <Eigen/Core>

#include <sym/pose3.h>
#include <symforce/slam/imu_preintegration/imu_factor.h>
#include <symforce/slam/imu_preintegration/imu_preintegrator.h>

namespace {

constexpr int kMeasurements = 100;
constexpr int kRounds = 2000;

template <typename Scalar>
struct Result {
  double seconds;
  Scalar checksum;
};

template <typename Scalar>
Result<Scalar> Run() {
  using Vector3 = Eigen::Matrix<Scalar, 3, 1>;
  const Vector3 accel_bias{Scalar{3.4}, Scalar{1.6}, Scalar{-5.9}};
  const Vector3 gyro_bias{Scalar{1.2}, Scalar{-2.4}, Scalar{0.5}};
  const Vector3 accel = accel_bias + Vector3::Constant(Scalar{4.3});
  const Vector3 gyro = gyro_bias + Vector3::Constant(Scalar{10.2});
  const Vector3 accel_cov = Vector3::Constant(Scalar{7.0e-5});
  const Vector3 gyro_cov = Vector3::Constant(Scalar{1.0e-3});
  const Vector3 gravity = Vector3::Zero();
  const sym::Pose3<Scalar> pose_i;
  const Vector3 vel_i = Vector3::Zero();
  const Scalar epsilon = std::is_same_v<Scalar, float> ? Scalar{1.0e-6} : Scalar{1.0e-9};
  Scalar checksum = Scalar{0};
  double integration_seconds = 0.0;
  double factor_seconds = 0.0;

  const auto start = std::chrono::steady_clock::now();
  for (int round = 0; round < kRounds; ++round) {
    sym::ImuPreintegrator<Scalar> integrator(accel_bias, gyro_bias);
    const auto integration_start = std::chrono::steady_clock::now();
    for (int sample = 0; sample < kMeasurements; ++sample) {
      integrator.IntegrateMeasurement(accel, gyro, accel_cov, gyro_cov, Scalar{1.0e-3}, epsilon);
    }
    integration_seconds +=
        std::chrono::duration<double>(std::chrono::steady_clock::now() - integration_start).count();
    const auto& delta = integrator.PreintegratedMeasurements().delta;
    const auto [pose_j, vel_j] = delta.RollForwardState(pose_i, vel_i, gravity);
    const auto factor_start = std::chrono::steady_clock::now();
    Eigen::Matrix<Scalar, 9, 1> residual;
    Eigen::Matrix<Scalar, 9, 24> jacobian;
    Eigen::Matrix<Scalar, 24, 24> hessian;
    Eigen::Matrix<Scalar, 24, 1> rhs;
    sym::ImuFactor<Scalar>{integrator}(pose_i, vel_i, pose_j, vel_j, accel_bias, gyro_bias, gravity,
                                       epsilon, &residual, &jacobian, &hessian, &rhs);
    factor_seconds +=
        std::chrono::duration<double>(std::chrono::steady_clock::now() - factor_start).count();
    if (round == 0) {
      std::cout << (std::is_same_v<Scalar, float> ? "cpp_f32" : "cpp_f64")
                << "_first_residual=" << residual(0) << " "
                << (std::is_same_v<Scalar, float> ? "cpp_f32" : "cpp_f64")
                << "_first_hessian=" << hessian(0, 0) << " "
                << (std::is_same_v<Scalar, float> ? "cpp_f32" : "cpp_f64")
                << "_cov00=" << integrator.Covariance()(0, 0) << " "
                << (std::is_same_v<Scalar, float> ? "cpp_f32" : "cpp_f64")
                << "_delta_dp0=" << delta.Dp(0) << '\n';
    }
    checksum += residual(0) + jacobian(0, 0) + hessian(0, 0) + rhs(0);
  }
  const auto elapsed = std::chrono::duration<double>(std::chrono::steady_clock::now() - start);
  std::cout << (std::is_same_v<Scalar, float> ? "cpp_f32" : "cpp_f64")
            << "_integration_seconds=" << integration_seconds << '\n'
            << (std::is_same_v<Scalar, float> ? "cpp_f32" : "cpp_f64")
            << "_factor_seconds=" << factor_seconds << '\n';
  return {elapsed.count(), checksum};
}

}  // namespace

int main() {
  std::cout << std::setprecision(17);
  const auto f64 = Run<double>();
  const auto f32 = Run<float>();
  std::cout << "cpp_f64_seconds=" << f64.seconds << '\n';
  std::cout << "cpp_f32_seconds=" << f32.seconds << '\n';
  std::cout << "cpp_f64_samples=" << static_cast<double>(kMeasurements) * kRounds << '\n';
  std::cout << "cpp_f64_checksum=" << f64.checksum << '\n';
  std::cout << "cpp_f32_checksum=" << f32.checksum << '\n';
}
