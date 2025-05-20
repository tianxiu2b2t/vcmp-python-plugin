#ifndef _CONFIG
#define _CONFIG

#include <fstream>
#include <string>
#include <stdio.h>
#include "logger.h"

using namespace std;
#define servercfg "server.cfg"

typedef struct {
    bool loggerDebug;
    std::string pythonscript;
    std::string pythonpath;
    bool preloader;
    std::string virualenv;
} config;

config cfg;

string readConfig(ifstream& stream, string key, string defaultValue) {
    string line;
    string value = defaultValue;

    // move to the beginning of the file
    stream.clear();
    stream.seekg(0, ios::beg);

    while (std::getline(stream, line)) {
        if (line.empty() || line[0] == '#') {
            continue;
        }

        size_t spiltSpace = line.find(' ');

        if (spiltSpace == std::string::npos) continue;

        std::string linekey = line.substr(0, spiltSpace);
        std::string linevalue = line.substr(spiltSpace + 1);

        if (linekey.compare(key) == 0) {
            value = linevalue;
            break;
        }
    }
    return value;
}
bool parseValueToBool(string value) {
    string lowercase;

    lowercase.resize(value.size());
    copy(value.begin(), value.end(), lowercase.begin());

    transform(lowercase.begin(), lowercase.end(), lowercase.begin(), ::tolower);
    return lowercase.compare("true") == 0 || lowercase.compare("yes") == 0 || lowercase.compare("y") == 0 || lowercase.compare("t") == 0 || lowercase.compare("1") == 0;
}

void loadConfig() {
    // check server.cfg exists
    std::ifstream stream(servercfg);
    if (!stream.is_open()) {
        logger.debug("Failed to open file: " + string(servercfg));
        return;
    }
    cfg.pythonscript = readConfig(stream, "python_script", "main.py");
    cfg.pythonpath = readConfig(stream, "python_path", "");
    cfg.loggerDebug = parseValueToBool(readConfig(stream, "pythonpython_loggerdebug", "false"));
    cfg.preloader = parseValueToBool(readConfig(stream, "python_preloader", "false"));
    cfg.virualenv = readConfig(stream, "python_virualenv", "");

    // debug
    stream.close();
}
#endif