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
} config;

config cfg;

string readConfig(ifstream& stream, string key, string defaultValue) {
    string line;
    string value = defaultValue;

    while (std::getline(stream, line)) {
        if (line.empty() || line[0] == '#') {
            continue;
        }

        size_t spiltSpace = line.find(' ');

        if (spiltSpace == std::string::npos) continue;

        std::string linekey = line.substr(0, spiltSpace);
        std::string linevalue = line.substr(spiltSpace + 1);

        transform(linekey.begin(), linekey.end(), linekey.begin(), ::tolower);
        transform(key.begin(), key.end(), key.begin(), ::tolower);
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
    // true, yes, y, t, 1
    return lowercase.compare("true") == 0 || lowercase.compare("yes") == 0 || lowercase.compare("y") == 0 || lowercase.compare("t") == 0 || lowercase.compare("1") == 0;
}

void loadConfig() {
    // check server.cfg exists
    std::ifstream stream(servercfg);
    if (!stream.is_open()) {
        logger.debug("Failed to open file: " + string(servercfg));
        return;
    }
    cfg.pythonscript = readConfig(stream, "pythonscript", "main.py");
    cfg.pythonpath = readConfig(stream, "pythonpath", "");
    cfg.loggerDebug = parseValueToBool(readConfig(stream, "pythonloggerdebug", "false"));

    // debug
    stream.close();
}