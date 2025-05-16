#include <pybind11/embed.h>
#include "meta.hpp"
#include "logger.h"

namespace py = pybind11;

//pybind11::dict locals = pybind11::dict();

void initCheckUpdate() {
    py::object locals = py::dict();
    std::string python_code = R"(
import threading

def loop_check_update(notice, noticeError, noticeError200):
    import time
    import requests
    url = "https://api.github.com/repos/)" + std::string(GITHUB_REPO) + R"(/releases/latest"
    check_interval = 86400
    while True:
        try:
            response = requests.get(url)
            if response.status_code != 200:
                noticeError200()
                return
            latest_release = response.json()
            latest_version = latest_release["tag_name"]
            print(f"Latest version: {latest_version}")
            notice(latest_version)
        except:
            noticeError()
        time.sleep(check_interval)

t = threading.Thread(target=loop_check_update, name="UpdateChecker", daemon=True, args=(notice, noticeError, noticeError200))
t.start()
)";
    locals["notice"] = py::cpp_function([](py::str version) {
        if (version.cast<std::string>() != "v" + std::string(PLUGIN_VERSION)) {
            logger.success("New version available: " + std::string(version));
            return;
        } 
        logger.info("This is the latest version.");
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

    try {
        // 执行Python代码
        py::exec(python_code, py::globals(), locals);
    } catch (const py::error_already_set& e) {
        // 处理Python异常
        logger.error("Python error: " + std::string(e.what()));
    }

}