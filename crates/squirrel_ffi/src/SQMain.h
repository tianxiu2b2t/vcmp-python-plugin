// #pragma once
// #ifndef _SQMAIN_H
// #define _SQMAIN_H

// #ifdef _WIN32
// 	#define WIN32_LEANANDMEAN
// 	#include <windows.h>

// 	#define EXPORT __declspec( dllexport )
// #else
// 	#include <sys/types.h>
// 	#define EXPORT
// 	#define MAX_PATH 250
// 	#define stricmp strcasecmp
// #endif

// #include "plugin.h"
// #include "SQImports.h"
// #include "ConsoleUtils.h"

// #ifdef __cplusplus
// extern "C" {
// #endif
// 	EXPORT	unsigned int			VcmpPluginInit					( PluginFuncs* pluginFuncs, PluginCallbacks* pluginCalls, PluginInfo* pluginInfo );
// #ifdef __cplusplus
// }
// #endif

// #endif