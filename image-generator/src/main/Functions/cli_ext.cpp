// Copyright ii887522

#include "cli_ext.h"  // NOLINT(build/include_subdir)
#include <string>
#include "../Any/constants.h"

using std::string;

namespace ii887522::imageGenerator {

bool isValid(int argc, char** argv) {
  return argc == 2 && (string{ argv[OUTPUT_DIRECTORY_PATH_INDEX] }.ends_with('/') || string{ argv[OUTPUT_DIRECTORY_PATH_INDEX] }.ends_with('\\'));
}

}  // namespace ii887522::imageGenerator
