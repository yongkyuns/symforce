/* ----------------------------------------------------------------------------
 * SymForce - Copyright 2022, Skydio, Inc.
 * This source code is under the Apache 2.0 license found in the LICENSE file.
 * ---------------------------------------------------------------------------- */

#include <algorithm>
#include <fstream>
#include <string>
#include <vector>

#include <Eigen/Core>
#include <spdlog/spdlog.h>

#include <sym/pose3.h>
#include <symforce/opt/factor.h>
#include <symforce/opt/optimizer.h>
#include <symforce/opt/values.h>

#include "./gen/keys.h"
#include "./gen/snavely_reprojection_factor.h"

using namespace sym::Keys;

/**
 * Create a `sym::Factor` for the reprojection residual, attached to the given camera and point
 * variables.  It's also attached to fixed entries in the Values for the pixel measurement and the
 * constant EPSILON.
 */
sym::Factord MakeFactor(int camera, int point, int pixel) {
  return sym::Factord::Hessian(sym::SnavelyReprojectionFactor<double>,
                               /* all_keys = */
                               {
                                   CAM_T_WORLD.WithSuper(camera),
                                   INTRINSICS.WithSuper(camera),
                                   POINT.WithSuper(point),
                                   PIXEL.WithSuper(pixel),
                                   EPSILON,
                               },
                               /* optimized_keys = */
                               {
                                   CAM_T_WORLD.WithSuper(camera),
                                   INTRINSICS.WithSuper(camera),
                                   POINT.WithSuper(point),
                               });
}

/**
 * A struct to represent the problem definition
 */
struct Problem {
  std::vector<sym::Factord> factors;
  sym::Valuesd values;
  int num_cameras;
  int num_points;
  int num_observations;
};

/**
 * Read the problem description from the given path
 *
 * See https://grail.cs.washington.edu/projects/bal/ for file format description
 */
Problem ReadProblem(const std::string& filename, const int max_cameras = -1,
                    const int max_points = -1) {
  std::ifstream file(filename);

  int num_cameras, num_points, num_observations;
  file >> num_cameras;
  file >> num_points;
  file >> num_observations;

  struct Observation {
    int camera;
    int point;
    Eigen::Vector2d pixel;
  };
  std::vector<Observation> observations;
  observations.reserve(num_observations);

  for (int i = 0; i < num_observations; i++) {
    int camera, point;
    file >> camera;
    file >> point;

    double px, py;
    file >> px;
    file >> py;

    observations.push_back({camera, point, Eigen::Vector2d(px, py)});
  }

  std::vector<sym::Pose3d> camera_poses;
  std::vector<Eigen::Vector3d> camera_intrinsics;
  camera_poses.reserve(num_cameras);
  camera_intrinsics.reserve(num_cameras);
  for (int i = 0; i < num_cameras; i++) {
    double rx, ry, rz, tx, ty, tz, f, k1, k2;
    file >> rx;
    file >> ry;
    file >> rz;
    file >> tx;
    file >> ty;
    file >> tz;
    file >> f;
    file >> k1;
    file >> k2;

    camera_poses.emplace_back(sym::Rot3d::FromTangent(Eigen::Vector3d(rx, ry, rz)),
                              Eigen::Vector3d(tx, ty, tz));
    camera_intrinsics.emplace_back(f, k1, k2);
  }

  std::vector<Eigen::Vector3d> points;
  points.reserve(num_points);
  for (int i = 0; i < num_points; i++) {
    double x, y, z;
    file >> x;
    file >> y;
    file >> z;

    points.emplace_back(x, y, z);
  }

  const int camera_count = max_cameras < 0 ? num_cameras : std::min(max_cameras, num_cameras);
  const int point_count = max_points < 0 ? num_points : std::min(max_points, num_points);
  std::vector<sym::Factord> factors;
  sym::Valuesd values;
  int selected_observations = 0;
  for (const auto& observation : observations) {
    if (observation.camera >= camera_count || observation.point >= point_count) {
      continue;
    }
    factors.push_back(MakeFactor(observation.camera, observation.point, selected_observations));
    values.Set(PIXEL.WithSuper(selected_observations), observation.pixel);
    ++selected_observations;
  }
  for (int i = 0; i < camera_count; i++) {
    values.Set(CAM_T_WORLD.WithSuper(i), camera_poses[i]);
    values.Set(INTRINSICS.WithSuper(i), camera_intrinsics[i]);
  }
  for (int i = 0; i < point_count; i++) {
    values.Set(POINT.WithSuper(i), points[i]);
  }

  values.Set(EPSILON, sym::kDefaultEpsilond);

  spdlog::info("Created problem with {} cameras, {} points, {} observations", camera_count,
               point_count, selected_observations);

  return {std::move(factors), std::move(values), camera_count, point_count, selected_observations};
}

/**
 * Example usage: `bundle_adjustment_in_the_large_example data/trafalgar/problem-21-11315-pre.txt`
 */
int main(int argc, char** argv) {
  SYM_ASSERT(argc == 2 || argc == 4);

  const int max_cameras = argc == 4 ? std::stoi(argv[2]) : -1;
  const int max_points = argc == 4 ? std::stoi(argv[3]) : -1;

  // Read the problem from disk, and create the Values and factors
  const auto problem = ReadProblem(argv[1], max_cameras, max_points);

  // Create a copy of the Values - we'll optimize this one in place
  sym::Valuesd optimized_values = problem.values;

  // Optimize
  auto params = sym::DefaultOptimizerParams();
  params.verbose = true;
  params.lambda_update_type = sym::lambda_update_type_t::DYNAMIC;
  sym::Optimizerd optimizer{params, std::move(problem.factors)};
  const auto stats = optimizer.Optimize(optimized_values);

  const auto& best_iter = stats.iterations[stats.best_index];
  // The first C++ stats entry is the initial linearization, matching the Rust
  // examples' convention of reporting attempted optimization steps only.
  spdlog::info("Iterations: {}", stats.iterations.size() - 1);
  spdlog::info("Final error: {:.12f}", best_iter.new_error);
}
