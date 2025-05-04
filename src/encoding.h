#include <string>
#include <vector>
#include <stdexcept>
#include <iostream>
#ifndef _WIN32
#include <iconv.h>
#else
#include <windows.h>
#endif

std::string utf8_to_gbk(const std::string& utf8_str) {
    std::string gbk_str;
    #ifdef _WIN32
        int wide_char_count = MultiByteToWideChar(CP_UTF8, 0, utf8_str.c_str(), -1, nullptr, 0);
        if (wide_char_count == 0) {
            throw std::runtime_error("MultiByteToWideChar failed");
        }
        std::vector<wchar_t> wide_buf(wide_char_count, 0);
        MultiByteToWideChar(CP_UTF8, 0, utf8_str.c_str(), -1, wide_buf.data(), wide_char_count);

        int gbk_char_count = WideCharToMultiByte(CP_ACP, 0, wide_buf.data(), -1, nullptr, 0, nullptr, nullptr);
        if (gbk_char_count == 0) {
            throw std::runtime_error("WideCharToMultiByte failed");
        }
        std::vector<char> gbk_buf(gbk_char_count, 0);
        WideCharToMultiByte(CP_ACP, 0, wide_buf.data(), -1, gbk_buf.data(), gbk_char_count, nullptr, nullptr);

        gbk_str = std::string(gbk_buf.begin(), gbk_buf.end() - 1);
    #else
        iconv_t conv = iconv_open("GBK", "UTF-8");
        if (conv == (iconv_t)-1) {
            throw std::runtime_error("iconv_open failed");
        }

        const char* in_buf = utf8_str.c_str();
        size_t in_len = utf8_str.size();
        std::vector<char> out_buf(in_len * 2); 
        char* out_ptr = out_buf.data();
        size_t out_len = out_buf.size();

        size_t converted = iconv(conv, &in_buf, &in_len, &out_ptr, &out_len);
        iconv_close(conv);

        if (converted == (size_t)-1 || in_len != 0) {
            throw std::runtime_error("iconv conversion failed");
        }

        out_buf.resize(out_buf.size() - out_len);
        while (!out_buf.empty() && (out_buf.back() == '\0')) {
            out_buf.pop_back();
        }

        gbk_str = std::string(out_buf.begin(), out_buf.end());
    #endif
    return gbk_str;
}

std::string gbk_to_utf8(const std::string& gbk_str) {
    std::string utf8_str;
    #ifdef _WIN32
        int wide_char_count = MultiByteToWideChar(CP_ACP, 0, gbk_str.c_str(), -1, nullptr, 0);
        if (wide_char_count == 0) {
            throw std::runtime_error("MultiByteToWideChar failed");
        }
        std::vector<wchar_t> wide_buf(wide_char_count, 0);
        MultiByteToWideChar(CP_ACP, 0, gbk_str.c_str(), -1, wide_buf.data(), wide_char_count);

        int utf8_char_count = WideCharToMultiByte(CP_UTF8, 0, wide_buf.data(), -1, nullptr, 0, nullptr, nullptr);
        if (utf8_char_count == 0) {
            throw std::runtime_error("WideCharToMultiByte failed");
        }
        std::vector<char> utf8_buf(utf8_char_count, 0);
        WideCharToMultiByte(CP_UTF8, 0, wide_buf.data(), -1, utf8_buf.data(), utf8_char_count, nullptr, nullptr);

        utf8_str = std::string(utf8_buf.begin(), utf8_buf.end() - 1);
    #else
        iconv_t conv = iconv_open("UTF-8", "GBK");
        if (conv == (iconv_t)-1) {
            throw std::runtime_error("iconv_open failed");
        }

        const char* in_buf = gbk_str.c_str();
        size_t in_len = gbk_str.size();
        std::vector<char> out_buf(in_len * 3); 
        char* out_ptr = out_buf.data();
        size_t out_len = out_buf.size();

        size_t converted = iconv(conv, &in_buf, &in_len, &out_ptr, &out_len);
        iconv_close(conv);

        if (converted == (size_t)-1 || in_len != 0) {
            throw std::runtime_error("iconv conversion failed");
        }

        out_buf.resize(out_buf.size() - out_len);
        while (!out_buf.empty() && (out_buf.back() == '\0')) {
            out_buf.pop_back();
        }

        utf8_str = std::string(out_buf.begin(), out_buf.end());
    #endif
    return utf8_str;
}