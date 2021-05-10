// Copyright ii887522

#include "cli_ext.h"  // NOLINT(build/include_subdir)
#include <string>

using std::string;

namespace ii887522::imageGenerator {

bool isValid(int argc, char** argv) {
  return argc == 2 && (string{ argv[1u] }.ends_with('/') || string{ argv[1u] }.ends_with('\\'));
}

}  // namespace ii887522::imageGenerator
