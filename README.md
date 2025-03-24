# RayTracing

Ray Tracing program written in Rust for the FMF class 'Programiranje 2'.


## Idea
The goal of this project is to implement a ray tracing algorithm capable of rendering all five Platonic solids and a sphere. Additionally, we may extend support for custom meshes to allow more flexibility in the rendering process. The focus is on correctness, efficiency, and learning about ray tracing fundamentals.

Ray tracing is a powerful technique for generating realistic images by simulating how rays of light interact with objects in a scene. The program will calculate intersections between rays and geometric objects, handle shading and lighting effects, and produce rendered images.

The project consists of two main components:

1. Ray tracing implementation – The core algorithm responsible for casting rays, detecting intersections, handling shading, and rendering images.
2. Executable program – A program that utilizes our ray tracing implementation to generate and display images.

### Requirements

We require SDL2 to run. On linux you can use your package manager to get it.

Examples:
- Ubuntu: `sudo apt-get install libsdl2-dev`
- Fedora: `sudo dnf install SDL2-devel`
- Arch: `pacman -S sdl2`
- MacOS: `brew install sdl2`
- Windows (MSVC):

1. Download MSVC development libraries from http://www.libsdl.org/ (SDL2-devel-2.0.x-VC.zip). (Note: version of SDL2 must be 2.28.x or newer else it wont work)
2. Unpack SDL2-devel-2.0.x-VC.zip to a folder of your choosing (You can delete it afterwards).
3. Copy all lib files from
    > SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

    to (for Rust 1.6 and above)
    > C:\Program Files\Rust\\**lib**\rustlib\x86_64-pc-windows-msvc\lib

    or to (for Rust versions 1.5 and below)
    > C:\Program Files\Rust\\**bin**\rustlib\x86_64-pc-windows-msvc\lib

    or to your library folder of choice, and ensure you have a system environment variable of
    > LIB = C:\your\rust\library\folder

    For Rustup users, this folder will be in
    > C:\Users\\{Your Username}\\.rustup\toolchains\\{current toolchain}\lib\rustlib\\{current toolchain}\lib

  Where current toolchain is likely `stable-x86_64-pc-windows-msvc`.

4. Copy `SDL2.lib` and `SDL2.dll` from
    > SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\

    into your cargo project, right next to your Cargo.toml.

5. When you're shipping your game make sure to copy `SDL2.dll` to the same directory that your compiled exe is in, otherwise the game won't launch.

#### Literature

- [Raytracing in One Weekend](https://raytracing.github.io/)
