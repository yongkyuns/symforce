#include <fstream>
#include <iostream>
#include <vector>

#include <Eigen/MetisSupport>
#include <Eigen/SparseCore>

int main(int argc, char** argv) {
  if (argc != 3) {
    std::cerr << "usage: generate_bal_metis_ordering <bal-file> <output>\n";
    return 2;
  }

  std::ifstream input(argv[1]);
  int cameras = 0;
  int points = 0;
  int observations = 0;
  input >> cameras >> points >> observations;
  const int state_dim = cameras * 9 + points * 3;
  std::vector<Eigen::Triplet<double>> triplets;
  triplets.reserve(state_dim + observations * 144);
  for (int index = 0; index < state_dim; ++index) {
    triplets.emplace_back(index, index, 1.0);
  }
  for (int observation = 0; observation < observations; ++observation) {
    int camera = 0;
    int point = 0;
    double x = 0.0;
    double y = 0.0;
    input >> camera >> point >> x >> y;
    const int columns[12] = {
        camera * 6,
        camera * 6 + 1,
        camera * 6 + 2,
        camera * 6 + 3,
        camera * 6 + 4,
        camera * 6 + 5,
        cameras * 6 + camera * 3,
        cameras * 6 + camera * 3 + 1,
        cameras * 6 + camera * 3 + 2,
        cameras * 9 + point * 3,
        cameras * 9 + point * 3 + 1,
        cameras * 9 + point * 3 + 2,
    };
    for (int column = 0; column < 12; ++column) {
      for (int row = 0; row < 12; ++row) {
        triplets.emplace_back(columns[row], columns[column], 1.0);
      }
    }
  }
  Eigen::SparseMatrix<double> pattern(state_dim, state_dim);
  pattern.setFromTriplets(triplets.begin(), triplets.end());
  pattern.makeCompressed();

  Eigen::PermutationMatrix<Eigen::Dynamic, Eigen::Dynamic, int> inverse;
  Eigen::MetisOrdering<int> metis;
  metis(pattern, inverse);

  std::ofstream output(argv[2]);
  output << "// Generated from Eigen MetisOrdering for the BAL problem.\n\n";
  output << "pub const BAL_METIS_ORDERING: [usize; " << state_dim << "] = [\n";
  for (int index = 0; index < state_dim; ++index) {
    output << "    " << inverse.indices()[index] << ",\n";
  }
  output << "];\n";
}
