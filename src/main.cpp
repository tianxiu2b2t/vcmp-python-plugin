#define UNICODE
#define _UNICODE

#include "main.h"
#include "logger.h"
#include "pymodule.h"
#include "update.h"


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

	logger.debug("Python script file: " + cfg.pythonscript);

	
	py::initialize_interpreter(false);

	try {
		{
			initCheckUpdate();
		}
		// eval py
		{
			py::eval_file(cfg.pythonscript.c_str());
		} 
	} catch (const py::error_already_set& e) {
		logger.error("Python eval script error: " + std::string(e.what()));
	} catch (const std::exception& e) {
		logger.error("Python script error: " + std::string(e.what()));
	} catch (...) {
		logger.error("Python script error: unknown error");
	}
	//initCheckUpdate();

	return 1;
}
