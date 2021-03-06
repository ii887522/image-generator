// Copyright ii887522

#ifndef TEST

#include "ImageGeneratorViewGroupFactory.h"
#include <SDL.h>
#include <viewify/viewify.h>
#include <nitro/nitro.h>
#include <string>
#include <chrono>  // NOLINT(build/c++11)
#include <vector>

using std::string;
using ii887522::viewify::ViewGroupFactory;
using std::chrono::high_resolution_clock;
using ii887522::viewify::ViewGroup;
using ii887522::viewify::Point;
using ii887522::viewify::Size;
using ii887522::viewify::Rect;
using ii887522::viewify::Paint;
using ii887522::viewify::Color;
using ii887522::viewify::RectView;
using ii887522::viewify::snapshot;
using ii887522::viewify::View;
using ii887522::nitro::emptyDir;
using ii887522::nitro::Action;
using std::to_string;
using std::vector;

namespace ii887522::imageGenerator {

ImageGeneratorViewGroupFactory::ImageGeneratorViewGroupFactory(const string& outDirPath) : ViewGroupFactory{ }, lengths{ 1, 256 }, colorComponents{ 32u, 224u },
  alphaComponents{ 128u, 255u }, randomEngine{ static_cast<unsigned int>(high_resolution_clock::now().time_since_epoch().count()) }, outDirPath{ outDirPath }, i{ 0u } { }

ViewGroup ImageGeneratorViewGroupFactory::make(SDL_Renderer*const renderer, const Size<int>&) {
  emptyDir(outDirPath);
  return ViewGroup{ renderer, Point{ 0, 0 }, [this](ViewGroup*const, SDL_Renderer*const renderer) {
    return vector<View*>{
      new RectView{
        renderer, Point{ 0, 0 }, Paint{
          Size{ lengths(randomEngine), lengths(randomEngine) },
          Color{ colorComponents(randomEngine), colorComponents(randomEngine), colorComponents(randomEngine), alphaComponents(randomEngine) }
        }, [this, renderer](Rect<int>& rect, Color<unsigned int>& color) {
          snapshot(renderer, rect, outDirPath + to_string(i) + ".png");
          rect.size = Size{ lengths(randomEngine), lengths(randomEngine) };
          color = Color{ colorComponents(randomEngine), colorComponents(randomEngine), colorComponents(randomEngine), alphaComponents(randomEngine) };
          ++i;
          constexpr auto IMAGE_COUNT{ 64u };
          return i != IMAGE_COUNT ? Action::NONE : Action::QUIT;
        }
      }
    };
  } };
}

}  // namespace ii887522::imageGenerator

#endif
