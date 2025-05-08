#define UNICODE
#define _UNICODE

#if WIN32
#include <windows.h>
#endif


#include "plugin.h"
#include "meta.hpp"
#include "config.h"
#include "encoding.h"
#include <functional>

#include <pybind11/embed.h>
namespace py = pybind11;

#ifdef _WIN32
#define EXPORT __declspec(dllexport)

#define strcpy strcpy_s
#else
#define EXPORT
#endif

PluginFuncs* funcs;
PluginCallbacks* calls;
