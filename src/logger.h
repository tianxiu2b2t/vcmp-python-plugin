#pragma once
#define _CRT_SECURE_NO_WARNINGS

#ifndef LOGGER_H
#define LOGGER_H

#include <string>
#include <ctime>
#include <vector>
#include <map>
#include <iostream>
#include <regex>
#include <variant>
#include <time.h>
#include <iomanip>

#ifdef WIN32
#include <Windows.h>
#endif
#define PYBIND11_DETAILED_ERROR_MESSAGES

using namespace std;

class Logger {
private:
#ifdef WIN32
    HANDLE hstdout;
#else
    void* hstdout;
#endif
    map<string, int> COLORS;
    map<int, int> COLORS_32;
    map<string, int> LEVELS;
    string FORMAT;
    string timeFormat = "%Y-%m-%d %H:%M:%S";
    regex REGEXP;

    string getFormattedTime() {
        time_t t = time(nullptr);
        tm* localTime = nullptr;
        if ((localTime = localtime(&t)) != nullptr) {
            char buffer[256];
            strftime(buffer, sizeof(buffer), timeFormat.c_str(), localTime);
            return string(buffer);
        }
        return "1970-01-01 00:00:00";
    }

    vector<string> parseLogMessage(const string level, const string message) {
        string format = FORMAT;
        smatch matches;
        vector<string> parsedResults;
        while (regex_search(format, matches, REGEXP, regex_constants::match_any)) {
            string result = matches.str();
            parsedResults.push_back("0" + format.substr(0, matches.position()));
            if (result.substr(0, 1) == "%" && result.substr(result.length() - 1) == "%") {
                result = result.substr(1, result.length() - 2);
                if (result == "datetime") {
                    parsedResults.push_back("0" + getFormattedTime());
                }
                else if (result == "level") {
                    parsedResults.push_back("0" + string(level.c_str()));
                }
                else if (result == "message") {
                    parsedResults.push_back("0" + string(message.c_str()));
                }
            }
            else {
                string color = result.substr(1, result.length() - 2);
                if (color.substr(0, 1) == "/") {
                    parsedResults.push_back("1-1");
                }
                else if (COLORS.find(color) != COLORS.end()) {
                    parsedResults.push_back("1" + to_string(COLORS[color]));
                }
                else if (LEVELS.find(level) != LEVELS.end()) {
                    parsedResults.push_back("1" + to_string(LEVELS[level]));
                }
            }
            format = format.substr(matches.length() + matches.position());
        }
        parsedResults.push_back("0" + format);
        return parsedResults;
    }

    void rawLogger(const std::string level, const std::string message) {
        fprintf(stderr, "%s\n", "test");
        if (level == "DEBUG" && !DEBUG) {
            return;
        }
        fprintf(stderr, "%s\n", "test1");
    
        try {
            std::vector<std::string> parsed = parseLogMessage(level, message);
            fprintf(stderr, "%s\n", "test2");
            std::vector<int> lastColors = {COLORS.at("clear")}; // 使用.at()检查键存在
            fprintf(stderr, "%s\n", "test3");
    
            for (const std::string& str : parsed) {
                fprintf(stderr, "%s\n", "str");
                if (str.empty()) continue;
    
                if (!str.empty() && str[0] == '1') { // 添加空检查
                    if (str.size() > 1) {
                        int number = 0;
                        try {
                            number = std::stoi(str.substr(1));
                        } catch (const std::exception&) {
                            number = COLORS.at("clear");
                        }
    
                        if (number == -1 && !lastColors.empty()) {
                            lastColors.pop_back();
                        } else {
                            if (COLORS_32.find(number) != COLORS_32.end()) { // 检查颜色有效性
                                lastColors.push_back(number);
                            } else {
                                lastColors.push_back(COLORS.at("clear"));
                            }
                        }
                    }
                    continue;
                }
                fprintf(stderr, "%s\n", "str1");
                if (str.size() > 1) { // 确保足够长度
                    const std::string text = str.substr(1);
                    fprintf(stderr, "%s\n%s", "str2", text.c_str());
                    int color = lastColors.empty() ? COLORS.at("clear") : lastColors.back();
                    fprintf(stderr, "%s\n", "str3");
    
                    #ifdef WIN32
                    if (hstdout) { // 确保句柄有效
                        fprintf(stderr, "%s\n", "str4");
                        CONSOLE_SCREEN_BUFFER_INFO csbBefore;
                        GetConsoleScreenBufferInfo(hstdout, &csbBefore);
                        SetConsoleTextAttribute(hstdout, COLORS_32.at(color)); // 使用.at()
                        if (!text.empty()) {
                            fputs(text.c_str(), stdout);
                        }
                        SetConsoleTextAttribute(hstdout, csbBefore.wAttributes);
                    }
                    #else
                    fprintf(stderr, "%s\n", "str5");
                    if (!text.empty()) {
                        fprintf(stderr, "%s\n", "str6");
                        if (COLORS_32.find(color) != COLORS_32.end()) {
                            fprintf(stderr, "%s\n", "str7");
                            printf("\x1b[%sm%s\x1b[0m", COLORS_32.at(color), text.c_str());
                        } else {
                            fprintf(stderr, "%s\n", "str8");
                            printf("%s", text.c_str());
                        }
                    }
                    #endif
                }
            }
        } catch (const std::exception& e) {
            fprintf(stderr, " Logging Error: %s\n", e.what());
        }
    }
public:
    bool DEBUG;
    Logger(string formatter = "<white>[%datetime%]</white> <level>[%level%]</level><yellow>:</yellow> <level>%message%\n", bool debug = false) {
        FORMAT = formatter;
        DEBUG = debug;

        COLORS = {
            {"red", 31},
            {"green", 32},
            {"yellow", 33},
            {"blue", 34},
            {"light_yellow", 93},
            {"white", 97},
            {"clear", -1},
            {"cyan", 36},
        };

        COLORS_32 = {
            {31, 12},
            {-1, -1},
            {32, 10},
            {97, 15},
            {33, 14},
            {34, 11},
#ifdef WIN32
            {36, FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY}
#endif
        };

        LEVELS = {
            {"INFO", COLORS["white"]},
            {"SUCCESS", COLORS["green"]},
            {"ERROR", COLORS["red"]},
            {"WARNING", COLORS["yellow"]},
            {"DEBUG", COLORS["blue"]},
        };

        REGEXP = regex("<(/?)[a-z]+>|%[a-zA-Z0-9-_]+%");

#ifdef WIN32
        hstdout = GetStdHandle(STD_OUTPUT_HANDLE);
#else
        hstdout = NULL;
#endif
    }

    void setDebug(bool debug) {
        DEBUG = debug;
    }

    Logger opt(string formatter = "<white>[%datetime%]</white> <level>[%level%]</level><yellow>:</yellow> <level>%message%\n", bool debug = false) {
        return Logger(formatter, debug);
    }

    void info(const char* message) {
        rawLogger("INFO", string(message));
    }

    void info(string message) {
        rawLogger("INFO", message);
    }

    void error(const char* message) {
        rawLogger("ERROR", string(message));
    }

    void error(string message) {
        rawLogger("ERROR", message);
    }

    void debug(const char* message) {
        rawLogger("DEBUG", string(message));
    }

    void debug(string message) {
        rawLogger("DEBUG", message);
    }

    void success(const char* message) {
        rawLogger("SUCCESS", string(message));
    }

    void success(string message) {
        rawLogger("SUCCESS", message);
    }

    void warning(const char* message) {
        rawLogger("WARNING", string(message));
    }

    void warning(string message) {
        rawLogger("WARNING", message);
    }
};

extern const Logger defaultLogger;
extern Logger logger;

// 初始化全局变量
const Logger defaultLogger = Logger();
Logger logger = Logger();

#endif // LOGGER_H