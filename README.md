# vcmp-python-plugin [![Build status](https://ci.appveyor.com/api/projects/status/6kcnwdbgkdxyux87?svg=true)](https://ci.appveyor.com/project/ysc3839/vcmp-python-plugin)
Python binding plugin for VC:MP.

# Building
This project uses [CMake](https://cmake.org/) build system.

## Cloning
This project includes `pybind11` submodule, use:

`git clone --recursive https://github.com/ysc3839/vcmp-python-plugin.git`

## Linux
You need to install `python3-dev` and `camke`.

On Debian-like systems just run:
`sudo apt install python3-dev cmake`

After installation, run:
```sh
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
make
```

For both Linux and Windows, you can use `-DPYTHON_EXECUTABLE=path/to/python` to specify Python executable.

For example: `cmake -DPYTHON_EXECUTABLE="C:\Program Files\Python36\python.exe" ..`

> TODO: Cross compile for other architecture.

## Windows
Download & install [CMake](https://cmake.org/download/) and [Python](https://www.python.org/downloads/).

After installation, run:
```cmd
mkdir build
cd build
cmake ..
cmake --build . --config Release
```
Or you can open generated solution files in Visual Studio.

The default architecture for cmake is 32 bit. If you want to build a 64 bit binary, you need to specify a 64 bit generator.

Run `cmake --help` and you can see all generators. Most cases you just need to use `Visual Studio 15 2017 Win64` generator.

For example: `cmake -G "Visual Studio 15 2017 Win64" ..`
