// Copyright ii887522

#ifdef TEST

#include <catch.hpp>
#include <cstring>
#include "../../main/Functions/cli_ext.h"
#include "../../main/Any/constants.h"

namespace ii887522::imageGenerator {

TEST_CASE("test isValid() function") {
  {
    char* args[]{ new char[16u] };
    strcpy_s(args[PROGRAM_NAME_I], 16u, "image-generator");
    REQUIRE_FALSE(isValid(sizeof args / sizeof(char*), args));
  }
  {
    char* args[]{ new char[16u], new char[2u] };
    strcpy_s(args[PROGRAM_NAME_I], 16u, "image-generator");
    strcpy_s(args[OUTPUT_DIRECTORY_PATH_I], 2u, "a");
    REQUIRE_FALSE(isValid(sizeof args / sizeof(char*), args));
  }
  {
    char* args[]{ new char[16u], new char[2u] };
    strcpy_s(args[PROGRAM_NAME_I], 16u, "image-generator");
    strcpy_s(args[OUTPUT_DIRECTORY_PATH_I], 2u, "b");
    REQUIRE_FALSE(isValid(sizeof args / sizeof(char*), args));
  }
  {
    char* args[]{ new char[16u], new char[3u] };
    strcpy_s(args[PROGRAM_NAME_I], 16u, "image-generator");
    strcpy_s(args[OUTPUT_DIRECTORY_PATH_I], 3u, "b/");
    REQUIRE(isValid(sizeof args / sizeof(char*), args));
  }
  {
    char* args[]{ new char[16u], new char[3u] };
    strcpy_s(args[PROGRAM_NAME_I], 16u, "image-generator");
    strcpy_s(args[OUTPUT_DIRECTORY_PATH_I], 3u, "b\\");
    REQUIRE(isValid(sizeof args / sizeof(char*), args));
  }
}

}  // namespace ii887522::imageGenerator

#endif
