// Copyright ii887522

#ifdef TEST

#define ALLOCATOR_IMPLEMENTATIONS
#include <nitro/Any/allocator.h>

#include "cli_ext.test.h"  // NOLINT(build/include_subdir)

namespace ii887522::imageGenerator {

static int main() {
  testCliExt();
  return 0;
}

}  // namespace ii887522::imageGenerator

int main() {
  return ii887522::imageGenerator::main();
}

#endif
