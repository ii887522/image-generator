// Copyright ii887522

#ifndef TEST

#define ALLOCATOR_IMPLEMENTATIONS
#include <nitro/Any/allocator.h>

#include <viewify/Functions/control_flow.h>
#include <viewify/Any/Subsystems.h>
#include <SDL.h>
#include <viewify/Any/App.h>
#include <viewify/Struct/Size.h>
#include <viewify/Struct/Color.h>
#include <iostream>
#include <string>
#include "../ViewGroupFactory/ImageGeneratorViewGroupFactory.h"
#include "cli_ext.h"  // NOLINT(build/include_subdir)
#include "../Any/constants.h"

using ii887522::viewify::Subsystems;
using ii887522::viewify::App;
using ii887522::viewify::Size;
using ii887522::viewify::Color;
using ii887522::viewify::eventLoop;
using std::cerr;
using std::string;

namespace ii887522::imageGenerator {

static int main(int argc, char** argv) {
  if (!isValid(argc, argv)) {
    cerr << "Command Line: image-generator <output-directory-path>\n";
    cerr << "Param <output-directory-path>: it must ends with either '/' or '\\'\n";
    return EXIT_FAILURE;
  }
  const Subsystems subsystems;
  ImageGeneratorViewGroupFactory imageGeneratorViewGroupFactory{ string{ argv[OUTPUT_DIRECTORY_PATH_I] } };  // See also viewify/View/ViewGroup.h for more details
  eventLoop(App{ "Image Generator", Size{ 512, 512 }, Color{ 0u, 0u, 0u, 255u }, &imageGeneratorViewGroupFactory, SDL_WINDOW_MINIMIZED });
  return EXIT_SUCCESS;
}

}  // namespace ii887522::imageGenerator

int main(int argc, char** argv) {
  return ii887522::imageGenerator::main(argc, argv);
}

#endif
