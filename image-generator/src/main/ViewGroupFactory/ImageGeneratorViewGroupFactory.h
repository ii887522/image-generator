// Copyright ii887522

#ifndef IMAGE_GENERATOR_SRC_MAIN_VIEWGROUPFACTORY_IMAGEGENERATORVIEWGROUPFACTORY_H_
#define IMAGE_GENERATOR_SRC_MAIN_VIEWGROUPFACTORY_IMAGEGENERATORVIEWGROUPFACTORY_H_

#ifndef TEST

#include <viewify/Factory/ViewGroupFactory.h>
#include <viewify/View/ViewGroup.h>
#include <SDL.h>
#include <viewify/Struct/Size.h>
#include <random>
#include <string>

using ii887522::viewify::ViewGroupFactory;
using ii887522::viewify::ViewGroup;
using ii887522::viewify::Size;
using std::uniform_int_distribution;
using std::default_random_engine;
using std::string;

namespace ii887522::imageGenerator {

// Not Thread Safe: it must only be used in main thread
// See also View/ViewGroup.h for more details
class ImageGeneratorViewGroupFactory final : public ViewGroupFactory {
  // remove copy semantics
  ImageGeneratorViewGroupFactory(const ImageGeneratorViewGroupFactory&) = delete;
  ImageGeneratorViewGroupFactory& operator=(const ImageGeneratorViewGroupFactory&) = delete;

  // remove move semantics
  ImageGeneratorViewGroupFactory(ImageGeneratorViewGroupFactory&&) = delete;
  ImageGeneratorViewGroupFactory& operator=(ImageGeneratorViewGroupFactory&&) = delete;

  uniform_int_distribution<int> lengths;
  uniform_int_distribution<unsigned int> colorComponents;
  uniform_int_distribution<unsigned int> alphaComponents;
  default_random_engine randomEngine;
  const string outDir;
  unsigned int index;

 public:
  // Param outDir: it must ends with '/' or '\\'
  // See also View/ViewGroup.h for more details
  explicit ImageGeneratorViewGroupFactory(const string& outDir);

  // Param renderer: it must not be assigned to integer
  // See also View/ViewGroup.h for more details
  ViewGroup make(SDL_Renderer*const renderer, const Size<int>&) override;
};

}  // namespace ii887522::imageGenerator

#endif
#endif  // IMAGE_GENERATOR_SRC_MAIN_VIEWGROUPFACTORY_IMAGEGENERATORVIEWGROUPFACTORY_H_
