// Copyright ii887522

#ifndef IMAGE_GENERATOR_SRC_MAIN_VIEWGROUPFACTORY_IMAGEGENERATORVIEWGROUPFACTORY_H_
#define IMAGE_GENERATOR_SRC_MAIN_VIEWGROUPFACTORY_IMAGEGENERATORVIEWGROUPFACTORY_H_

#ifndef TEST

#include <Factory/ViewGroupFactory.h>
#include <View/ViewGroup.h>
#include <SDL.h>
#include <Struct/Size.h>
#include <Struct/Point.h>
#include <View/RectView.h>
#include <Struct/Paint.h>
#include <Struct/Color.h>
#include <Struct/Rect.h>
#include <Any/Enums.h>
#include <Functions/sdl_ext.h>
#include <random>
#include <chrono>  // NOLINT(build/c++11)
#include <string>

using ii887522::viewify::ViewGroupFactory;
using ii887522::viewify::ViewGroup;
using ii887522::viewify::Size;
using ii887522::viewify::Point;
using ii887522::viewify::RectView;
using ii887522::viewify::Paint;
using ii887522::viewify::Rect;
using ii887522::viewify::Size;
using ii887522::viewify::Color;
using ii887522::viewify::Action;
using ii887522::viewify::snapshot;
using std::uniform_int_distribution;
using std::default_random_engine;
using std::chrono::high_resolution_clock;
using std::to_string;

namespace ii887522::imageGenerator {

// Not Thread Safe: it must only be used in main thread
// See also View/ViewGroup.h for more details
template <unsigned int viewCount> class ImageGeneratorViewGroupFactory final : public ViewGroupFactory<viewCount> {
  // remove copy semantics
  ImageGeneratorViewGroupFactory(const ImageGeneratorViewGroupFactory&) = delete;
  ImageGeneratorViewGroupFactory& operator=(const ImageGeneratorViewGroupFactory&) = delete;

  // remove move semantics
  ImageGeneratorViewGroupFactory(ImageGeneratorViewGroupFactory&&) = delete;
  ImageGeneratorViewGroupFactory& operator=(ImageGeneratorViewGroupFactory&&) = delete;

  uniform_int_distribution<int> lengths;
  uniform_int_distribution<unsigned int> colorComponents;
  default_random_engine randomEngine;
  const string outDir;
  unsigned int index;

 public:
  // Param outDir: it must ends with '/' or '\\'
  // See also View/ViewGroup.h for more details
  explicit ImageGeneratorViewGroupFactory(const string& outDir) : ViewGroupFactory<viewCount>{ }, lengths{ 1, 256 }, colorComponents{ 32u, 224u },
    randomEngine{ static_cast<unsigned int>(high_resolution_clock::now().time_since_epoch().count()) }, outDir{ outDir }, index{ 0u } { }

  // Param renderer: it must not be assigned to integer
  // See also View/ViewGroup.h for more details
  ViewGroup<viewCount> make(SDL_Renderer*const renderer, const Size<int>&) override {
    return ViewGroup<viewCount>{ renderer, Point{ 0, 0 }, {
      new RectView{
        renderer, Point{ 0, 0 },
        Paint{ Size{ lengths(randomEngine), lengths(randomEngine) }, Color{ colorComponents(randomEngine), colorComponents(randomEngine), colorComponents(randomEngine) } },
        [this, renderer](Rect<int>& rect, Color<unsigned int>& color) {
          snapshot(renderer, rect, outDir + to_string(index) + ".png");
          rect.size = { lengths(randomEngine), lengths(randomEngine) };
          color = { colorComponents(randomEngine), colorComponents(randomEngine), colorComponents(randomEngine) };
          ++index;
          constexpr auto imageCount{ 64u };
          return index != imageCount ? Action::NONE : Action::QUIT;
        }
      }
    } };
  }
};

}  // namespace ii887522::imageGenerator

#endif
#endif  // IMAGE_GENERATOR_SRC_MAIN_VIEWGROUPFACTORY_IMAGEGENERATORVIEWGROUPFACTORY_H_
