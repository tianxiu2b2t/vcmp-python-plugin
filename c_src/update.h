#include <pybind11/embed.h>
#include "meta.hpp"
#include "config.h"
#include "logger.h"

namespace py = pybind11;

//pybind11::dict locals = pybind11::dict();
py::object locals;

const std::string STRING_PLUGIN_VERSION = "v" + std::string(PLUGIN_VERSION);

void initCheckUpdate() {
    locals = py::dict();
    if (cfg.disableUpdateChecker)
        return;
    std::string python_code = R"(
import threading

def loop_check_update(locals):
    notice, noticeError, noticeError200 = locals["notice"], locals["noticeError"], locals["noticeError200"]
    import time
    import requests
    url = "https://api.github.com/repos/)" + std::string(GITHUB_REPO) + R"(/releases/latest"
    check_interval = 86400
    last_check_time = time.perf_counter() - check_interval
    while locals["running"]:
        if time.perf_counter() - last_check_time >= check_interval:
            last_check_time = time.perf_counter()
            try:
                response = requests.get(url)
                if response.status_code != 200:
                    noticeError200()
                    return
                latest_release = response.json()
                latest_version = latest_release["tag_name"]
                notice(latest_version)
            except:
                noticeError()
        time.sleep(1)

t = threading.Thread(target=loop_check_update, name="UpdateChecker", daemon=True, args=(locals(),))
t.start()
)";
    locals["notice"] = py::cpp_function([](py::str version) {
        if (version.cast<std::string>() != STRING_PLUGIN_VERSION) {
            logger.success("New version available: " + std::string(version) + ". Current version: " + STRING_PLUGIN_VERSION);
            return;
        } 
        logger.info("This is the latest version (" + STRING_PLUGIN_VERSION + ").");
    });
    locals["noticeError"] = py::cpp_function([]() {
        try {
            throw;
        } catch (const py::error_already_set& e) {
            logger.error("Python error: " + std::string(e.what()));
        }
    });
    locals["noticeError200"] = py::cpp_function([]() {
        logger.info("Unable to fetch latest release information.");
    });
    locals["running"] = 1;

    try {
        // 执行Python代码
        py::exec(python_code, py::globals(), locals);
    } catch (const py::error_already_set& e) {
        // 处理Python异常
        logger.error("Python error: " + std::string(e.what()));
    }
}
void exitCheckUpdate() {
    locals["running"] = 0;
}