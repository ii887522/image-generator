// Copyright ii887522

#ifndef TEST

#include <Any/allocator.h>
#include <Functions/control_flow.h>
#include <SDL.h>
#include <Any/App.h>
#include <Struct/Size.h>
#include <Struct/Color.h>
#include <iostream>
#include <string>
#include "../ViewGroupFactory/ImageGeneratorViewGroupFactory.h"
#include "cli_ext.h"  // NOLINT(build/include_subdir)

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
  constexpr auto viewCount{ 1u };  // See also Any/View.h for more details
  ImageGeneratorViewGroupFactory<viewCount> imageGeneratorViewGroupFactory{ string{ argv[1u] } };  // See also View/ViewGroup.h for more details
  eventLoop(App<viewCount>{ "Image Generator", Size{ 512, 512 }, Color{ 0u, 0u, 0u }, &imageGeneratorViewGroupFactory, SDL_WINDOW_MINIMIZED });
  return EXIT_SUCCESS;
}

}  // namespace ii887522::imageGenerator

int main(int argc, char** argv) {
  return ii887522::imageGenerator::main(argc, argv);
}

#endif
