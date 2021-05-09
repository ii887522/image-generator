# image-generator
It is a CLI tool used to generate many sample images with different sizes and colors for testing purposes.

## Table of Contents
- [For developers reading this in GitHub](https://gitlab.com/ii887522/image-generator#for-developers-reading-this-in-github)
- [Coding Style](https://gitlab.com/ii887522/image-generator#coding-style)
- [Prerequisites](https://gitlab.com/ii887522/image-generator#prerequisites)
- [Update .NET Framework SDK version used by ms-build-tools to match your OS](https://gitlab.com/ii887522/image-generator#update-net-framework-sdk-version-used-by-ms-build-tools-to-match-your-os)
- [Build custom-node docker image](https://gitlab.com/ii887522/image-generator#build-custom-node-docker-image)
- [Build ms-build-tools docker image](https://gitlab.com/ii887522/image-generator#build-ms-build-tools-docker-image)
- [Install dependencies](https://gitlab.com/ii887522/image-generator#install-dependencies)
- [Set correct cpplint executable path in .vscode/settings.json](https://gitlab.com/ii887522/image-generator#set-correct-cpplint-executable-path-in-vscodesettingsjson)
- [Build project](https://gitlab.com/ii887522/image-generator#build-project)
- [Deploy project](https://gitlab.com/ii887522/image-generator#deploy-project)

## For developers reading this in GitHub
Please go to https://gitlab.com/ii887522/image-generator to start contributing instead.

## Coding Style
This project follows [Google C++ Style Guide](https://google.github.io/styleguide/cppguide.html). Please familiarize yourself with the rules provided
in the coding style and make sure all the proposed code changes in your commits are conforming to the style before making a merge request. You can
also make use of cpplint which is a [Visual Studio Code](https://code.visualstudio.com/) plugin and `build` command under the
[Build project](https://gitlab.com/ii887522/image-generator#build-project) section to support you.

## Prerequisites
- Windows 10
- [Docker Desktop](https://www.docker.com/products/docker-desktop) using Windows containers
- [Visual Studio Code](https://code.visualstudio.com/)
  - C/C++
  - cpplint
  - Docker
  - EditorConfig for VS Code
  - Markdown All in One
  - Remote - WSL
  - YAML
- [Visual Studio Community 2019](https://visualstudio.microsoft.com/) and later
  - Extensions:
    - Markdown Editor
    - EditorConfig Language Service
  - Workloads:
    - Desktop development with C++
  - Individual components:
    - JavaScript and TypeScript language support
    - Git for Windows
- [Python 3.9.1](https://www.python.org/downloads/) and later

## Update .NET Framework SDK version used by ms-build-tools to match your OS
1. Left click on the start button at the bottom left corner of your desktop.

  <img src="docs/start-button.png" alt="Start button" width="770" /><br /><br />

2. Left click on the settings button at the left side of the start menu.

  <img src="docs/settings.png" alt="Settings" width="770" /><br /><br />

3. Left click on the system button at the top left corner of settings window.

  <img src="docs/system.png" alt="System" width="770" /><br /><br />

4. Left click on the about button at the bottom left corner of settings window.

  <img src="docs/about.png" alt="About" width="770" /><br /><br />

5. Take note of the version shown in the settings window. You will need it to search for appropriate tags later.

  <img src="docs/version.png" alt="Version" width="770" /><br /><br />

6. Go to https://hub.docker.com/_/microsoft-dotnet-framework-sdk/ and find Full Tag Listing section.
7. Copy a tag that contains the version you have taken note of under Full Tag Listing section.
8. Finally, inside the repository, navigate to `ms-build-tools/Dockerfile`. Then, paste to replace the tag as highlighted and shown in the screenshot below and save it.

  <img src="docs/tag.png" alt="Tag" width="770" /><br /><br />

## Build custom-node docker image
```sh
cd custom-node
build
cd ..
```

## Build ms-build-tools docker image
```sh
cd ms-build-tools
build
cd ..
```

## Install dependencies
```sh
install
```

## Set correct cpplint executable path in `.vscode/settings.json`
1. In the explorer panel of [Visual Studio Code](https://code.visualstudio.com/), navigate to `env\Scripts\` or `env/bin/` in the project directory
and find the cpplint executable.
2. Right click on the cpplint executable and click on Copy Path to copy its absolute path.
3. In the explorer panel of [Visual Studio Code](https://code.visualstudio.com/), navigate to `.vscode/settings.json` in the project directory and
open it.
4. Finally, paste the absolute path as a string value of `"cpplint.cpplintPath"` in `settings.json` and save it.

### Build project
```sh
build
```

### Deploy project
```sh
deploy <version> <access-token>
```
