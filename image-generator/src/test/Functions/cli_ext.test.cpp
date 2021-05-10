// Copyright ii887522

#ifdef TEST

#include "cli_ext.test.h"  // NOLINT(build/include_subdir)
#include <cassert>
#include <cstring>
#include "../../main/Functions/cli_ext.h"

namespace ii887522::imageGenerator {

static void testIsValid() {
  {
    char* args[]{ new char[16u] };
    strcpy_s(args[0u], 16u, "image-generator");
    assert(!isValid(sizeof args / sizeof(char*), args));
  }
  {
    char* args[]{ new char[16u], new char[2u] };
    strcpy_s(args[0u], 16u, "image-generator");
    strcpy_s(args[1u], 2u, "a");
    assert(!isValid(sizeof args / sizeof(char*), args));
  }
  {
    char* args[]{ new char[16u], new char[2u] };
    strcpy_s(args[0u], 16u, "image-generator");
    strcpy_s(args[1u], 2u, "b");
    assert(!isValid(sizeof args / sizeof(char*), args));
  }
  {
    char* args[]{ new char[16u], new char[3u] };
    strcpy_s(args[0u], 16u, "image-generator");
    strcpy_s(args[1u], 3u, "b/");
    assert(isValid(sizeof args / sizeof(char*), args));
  }
  {
    char* args[]{ new char[16u], new char[3u] };
    strcpy_s(args[0u], 16u, "image-generator");
    strcpy_s(args[1u], 3u, "b\\");
    assert(isValid(sizeof args / sizeof(char*), args));
  }
}

void testCliExt() {
  testIsValid();
}

}  // namespace ii887522::imageGenerator

#endif
