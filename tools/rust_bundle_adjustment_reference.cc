/* ----------------------------------------------------------------------------
 * SymForce - Copyright 2022, Skydio, Inc.
 * This source code is under the Apache 2.0 license found in the LICENSE file.
 * ---------------------------------------------------------------------------- */

// Evaluate the existing C++ BA implementations on the exact Rust example inputs.
// A random seed does not establish identical fixtures across C++ platforms/builds.
#include <cmath>
#include <iomanip>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

#include <sym/linear_camera_cal.h>
#include <sym/pose3.h>
#include <symforce/examples/bundle_adjustment/build_example_state.h>
#include <symforce/examples/bundle_adjustment_fixed_size/build_example_state.h>
#include <symforce/examples/example_utils/bundle_adjustment_util.h>
#include <symforce/opt/factor.h>
#include <symforce/opt/optimizer.h>

// Reuse the example factor construction; do not implement a competing C++ solver.
namespace bundle_adjustment {
std::vector<sym::Factord> BuildFactors(const BundleAdjustmentProblemParams& params);
std::vector<sym::Key> ComputeKeysToOptimizeWithoutView0(const std::vector<sym::Factord>& factors);
}  // namespace bundle_adjustment
namespace bundle_adjustment_fixed_size {
sym::Factord BuildFactor();
}  // namespace bundle_adjustment_fixed_size

namespace {
using Var = bundle_adjustment::Var;
using FixedVar = bundle_adjustment_fixed_size::Var;

// Both example graphs address the same fixture schema. Fail compilation on key drift.
#define SAME_KEY(name) \
  static_assert(static_cast<char>(Var::name) == static_cast<char>(FixedVar::name))
SAME_KEY(VIEW);
SAME_KEY(CALIBRATION);
SAME_KEY(POSE_PRIOR_T);
SAME_KEY(POSE_PRIOR_SQRT_INFO);
SAME_KEY(LANDMARK);
SAME_KEY(LANDMARK_PRIOR);
SAME_KEY(LANDMARK_PRIOR_SIGMA);
SAME_KEY(MATCH_SOURCE_COORDS);
SAME_KEY(MATCH_TARGET_COORDS);
SAME_KEY(MATCH_WEIGHT);
SAME_KEY(GNC_MU);
SAME_KEY(GNC_SCALE);
SAME_KEY(EPSILON);
#undef SAME_KEY

double ReadScalar() {
  double value = 0;
  if (!(std::cin >> value) || !std::isfinite(value)) {
    throw std::runtime_error("truncated or nonfinite BA fixture");
  }
  return value;
}

template <int Rows, int Columns>
Eigen::Matrix<double, Rows, Columns> ReadMatrix() {
  Eigen::Matrix<double, Rows, Columns> matrix;
  for (int row = 0; row < Rows; ++row) {
    for (int column = 0; column < Columns; ++column) {
      matrix(row, column) = ReadScalar();
    }
  }
  return matrix;
}

sym::Pose3d ReadPose() {
  sym::Pose3d pose;
  pose.Data() = ReadMatrix<7, 1>();
  return pose;
}

sym::Valuesd ReadFixture(bundle_adjustment::BundleAdjustmentProblemParams& params) {
  std::string version;
  int views = 0;
  int landmarks = 0;
  if (!(std::cin >> version >> views >> landmarks) || version != "SYMFORCE_BA_V1" ||
      views != params.num_views || landmarks != params.num_landmarks) {
    throw std::runtime_error("unsupported BA fixture version or dimensions");
  }
  sym::Valuesd values;
  params.epsilon = ReadScalar();
  values.Set(Var::EPSILON, params.epsilon);
  values.Set(Var::GNC_MU, ReadScalar());
  params.reprojection_error_gnc_scale = ReadScalar();
  values.Set(Var::GNC_SCALE, params.reprojection_error_gnc_scale);
  for (int view = 0; view < views; ++view) {
    values.Set({Var::VIEW, view}, ReadPose());
    values.Set({Var::CALIBRATION, view}, sym::LinearCameraCald(ReadMatrix<4, 1>()));
  }
  for (int source = 0; source < views; ++source) {
    for (int target = 0; target < views; ++target) {
      values.Set({Var::POSE_PRIOR_T, source, target}, ReadPose());
      values.Set({Var::POSE_PRIOR_SQRT_INFO, source, target}, ReadMatrix<6, 6>());
    }
  }
  for (int landmark = 0; landmark < landmarks; ++landmark) {
    values.Set({Var::LANDMARK, landmark}, ReadScalar());
    values.Set({Var::LANDMARK_PRIOR, 1, landmark}, ReadScalar());
    values.Set({Var::MATCH_WEIGHT, 1, landmark}, ReadScalar());
    values.Set({Var::LANDMARK_PRIOR_SIGMA, 1, landmark}, ReadScalar());
    values.Set({Var::MATCH_SOURCE_COORDS, 1, landmark}, ReadMatrix<2, 1>());
    values.Set({Var::MATCH_TARGET_COORDS, 1, landmark}, ReadMatrix<2, 1>());
  }
  std::string extra;
  if (std::cin >> extra) {
    throw std::runtime_error("unexpected trailing BA fixture data");
  }
  return values;
}
}  // namespace

int main(int argc, char** argv) {
  try {
    if (argc != 2 || (std::string(argv[1]) != "runtime" && std::string(argv[1]) != "fixed")) {
      throw std::runtime_error("usage: rust_bundle_adjustment_reference <runtime|fixed> < fixture");
    }
    bundle_adjustment::BundleAdjustmentProblemParams params;
    auto values = ReadFixture(params);
    const auto factors =
        std::string(argv[1]) == "fixed"
            ? std::vector<sym::Factord>{bundle_adjustment_fixed_size::BuildFactor()}
            : bundle_adjustment::BuildFactors(params);
    const auto keys = bundle_adjustment::ComputeKeysToOptimizeWithoutView0(factors);
    sym::Optimizerd optimizer(sym::example_utils::OptimizerParams(), factors, "RustParityReference",
                              keys, params.epsilon);
    const auto stats = optimizer.Optimize(values);
    const auto& best = stats.iterations.at(stats.best_index);
    if (stats.status != sym::optimization_status_t::SUCCESS || !std::isfinite(best.new_error)) {
      throw std::runtime_error("C++ BA reference failed to converge");
    }
    std::cout << std::setprecision(17);
    std::cout << "iterations: " << stats.iterations.back().iteration << "\n";
    std::cout << "final error: " << best.new_error << "\n";
    const auto pose = values.At<sym::Pose3d>({Var::VIEW, 1});
    std::cout << "pose 1: (";
    for (int index = 0; index < 7; ++index) {
      if (index != 0) {
        std::cout << ", ";
      }
      std::cout << pose.Data()[index];
    }
    std::cout << ")\n";
    return 0;
  } catch (const std::exception& error) {
    std::cerr << error.what() << "\n";
    return 1;
  }
}
