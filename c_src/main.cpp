#define UNICODE
#define _UNICODE

#include "main.h"
#include "logger.h"
#include "pymodule.h"


extern "C" EXPORT uint32_t VcmpPluginInit(PluginFuncs* pluginFunctions, PluginCallbacks* pluginCallbacks, PluginInfo* pluginInfo)
{
	logger.setDebug(true);

	logger.setPrintCallback([&pluginFunctions](const std::string& msg) {
		pluginFunctions->LogMessage(msg.c_str());
	});

	pluginInfo->pluginVersion = 0x110;
	pluginInfo->apiMajorVersion = PLUGIN_API_MAJOR;
	pluginInfo->apiMinorVersion = PLUGIN_API_MINOR;
	strcpy(pluginInfo->name, PLUGIN_NAME);

	funcs = pluginFunctions;
	calls = pluginCallbacks;

	initVCMP(funcs, calls);

	// load config
	loadConfig();
	//logger.setDebug(cfg.loggerDebug);

	//logger.debug("Python script file: " + cfg.pythonscript);
	showPythonEnvironment();
	
	initPythonInterpreter();
	//initCheckUpdate();

	return 1;
}
