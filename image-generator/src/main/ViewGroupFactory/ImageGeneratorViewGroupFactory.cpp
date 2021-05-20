// Copyright ii887522

#ifndef TEST

#include "ImageGeneratorViewGroupFactory.h"
#include <viewify/Factory/ViewGroupFactory.h>
#include <string>
#include <chrono>  // NOLINT(build/c++11)
#include <viewify/View/ViewGroup.h>
#include <SDL.h>
#include <viewify/Struct/Point.h>
#include <viewify/Struct/Size.h>
#include <viewify/Struct/Rect.h>
#include <viewify/Struct/Paint.h>
#include <viewify/View/RectView.h>
#include <viewify/Struct/Color.h>
#include <viewify/Any/Enums.h>
#include <viewify/Functions/sdl_ext.h>
#include <viewify/Any/View.h>
#include <nitro/Functions/fs_ext.h>
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
using ii887522::viewify::Action;
using ii887522::viewify::snapshot;
using ii887522::viewify::View;
using ii887522::nitro::emptyDir;
using std::to_string;
using std::vector;

namespace ii887522::imageGenerator {

ImageGeneratorViewGroupFactory::ImageGeneratorViewGroupFactory(const string& outDir) : ViewGroupFactory{ }, lengths{ 1, 256 }, colorComponents{ 32u, 224u }, alphaComponents{ 128u, 255u },
  randomEngine{ static_cast<unsigned int>(high_resolution_clock::now().time_since_epoch().count()) }, outDir{ outDir }, index{ 0u } { }

ViewGroup ImageGeneratorViewGroupFactory::make(SDL_Renderer*const renderer, const Size<int>&) {
  emptyDir(outDir);
  return ViewGroup{ renderer, Point{ 0, 0 }, [this](ViewGroup&, SDL_Renderer*const renderer) {
    return vector<View*>{
      new RectView{
        renderer, Point{ 0, 0 }, Paint{
          Size{ lengths(randomEngine), lengths(randomEngine) },
          Color{ colorComponents(randomEngine), colorComponents(randomEngine), colorComponents(randomEngine), alphaComponents(randomEngine) }
        }, [this, renderer](Rect<int>& rect, Color<unsigned int>& color) {
          snapshot(renderer, rect, outDir + to_string(index) + ".png");
          rect.size = Size{ lengths(randomEngine), lengths(randomEngine) };
          color = Color{ colorComponents(randomEngine), colorComponents(randomEngine), colorComponents(randomEngine), alphaComponents(randomEngine) };
          ++index;
          constexpr auto imageCount{ 64u };
          return index != imageCount ? Action::NONE : Action::QUIT;
        }
      }
    };
  } };
}

}  // namespace ii887522::imageGenerator

#endif
