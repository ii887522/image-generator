'use strict'

import { options, dependencies, zip, file, dll } from '@ii887522/packify'

const projectDirPath = 'image-generator/'

options.outDirPath = 'libs/'
options.x86DllOutDirPaths = [`${projectDirPath}Debug/`, `${projectDirPath}Release/`, `${projectDirPath}Test/`],
options.x64DllOutDirPaths = [`${projectDirPath}x64/Debug/`, `${projectDirPath}x64/Release/`, `${projectDirPath}x64/Test/`]

const sdl2Version = '2.0.12'
const sdl2ImageVersion = '2.0.5'
const sdl2TTFVersion = '2.0.15'
const nitroVersion = '1.4.3'
const viewifyVersion = '1.7.2'
const texturePackerVersion = '1.1.5'
const fontPackerVersion = '1.2.3'

dependencies(async () => {
  await Promise.all([
    zip(`https://www.libsdl.org/release/SDL2-devel-${sdl2Version}-VC.zip`),
    zip(`https://www.libsdl.org/projects/SDL_image/release/SDL2_image-devel-${sdl2ImageVersion}-VC.zip`),
    zip(`https://www.libsdl.org/projects/SDL_ttf/release/SDL2_ttf-devel-${sdl2TTFVersion}-VC.zip`),
    zip(`https://github.com/ii887522/nitro/releases/download/v${nitroVersion}/nitro-${nitroVersion}.zip`),
    zip(`https://github.com/ii887522/viewify/releases/download/v${viewifyVersion}/viewify-${viewifyVersion}.zip`),
    zip(`https://github.com/ii887522/texture-packer/releases/download/v${texturePackerVersion}/texture-packer-${texturePackerVersion}.zip`),
    zip(`https://github.com/ii887522/font-packer/releases/download/v${fontPackerVersion}/font-packer-${fontPackerVersion}.zip`),
    file('https://github.com/catchorg/Catch2/releases/download/v2.13.6/catch.hpp')
  ])
  dll('x86', `SDL2-${sdl2Version}/lib/x86/SDL2.dll`)
  dll('x64', `SDL2-${sdl2Version}/lib/X64/SDL2.dll`)
  dll('x86', `SDL2_image-${sdl2ImageVersion}/lib/x86/libpng16-16.dll`)
  dll('x86', `SDL2_image-${sdl2ImageVersion}/lib/x86/SDL2_image.dll`)
  dll('x86', `SDL2_image-${sdl2ImageVersion}/lib/x86/zlib1.dll`)
  dll('x64', `SDL2_image-${sdl2ImageVersion}/lib/x64/libpng16-16.dll`)
  dll('x64', `SDL2_image-${sdl2ImageVersion}/lib/x64/SDL2_image.dll`)
  dll('x64', `SDL2_image-${sdl2ImageVersion}/lib/x64/zlib1.dll`)
  dll('x86', `SDL2_ttf-${sdl2TTFVersion}/lib/x86/libfreetype-6.dll`)
  dll('x86', `SDL2_ttf-${sdl2TTFVersion}/lib/x86/SDL2_ttf.dll`)
  dll('x64', `SDL2_ttf-${sdl2TTFVersion}/lib/x64/libfreetype-6.dll`)
  dll('x64', `SDL2_ttf-${sdl2TTFVersion}/lib/x64/SDL2_ttf.dll`)
})
