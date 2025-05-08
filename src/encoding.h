
#ifndef ENCODING_H
#define ENCODING_H
#include <string>
#include <vector>

#ifdef _WIN32
#include <windows.h>
#else
#include <iconv.h>
#endif

std::string utf8_to_gbk(const std::string& utf8_str) {
    std::string gbk_str;
#ifdef _WIN32
    // Windows 系统下使用 MultiByteToWideChar 和 WideCharToMultiByte 函数
    int wide_len = MultiByteToWideChar(CP_UTF8, 0, utf8_str.c_str(), -1, nullptr, 0);
    std::vector<wchar_t> wide_str(wide_len, 0);
    MultiByteToWideChar(CP_UTF8, 0, utf8_str.c_str(), -1, wide_str.data(), wide_len);

    int gbk_len = WideCharToMultiByte(CP_ACP, 0, wide_str.data(), -1, nullptr, 0, nullptr, nullptr);
    gbk_str.resize(gbk_len);
    WideCharToMultiByte(CP_ACP, 0, wide_str.data(), -1, &gbk_str[0], gbk_len, nullptr, nullptr);
#else
    // Linux 系统下使用 iconv 函数
    iconv_t conv = iconv_open("GBK", "UTF-8");
    if (conv == (iconv_t)-1) {
        return "";
    }

    std::vector<char> in_buf(utf8_str.begin(), utf8_str.end());
    in_buf.push_back('\0');
    char* in_ptr = &in_buf[0];
    size_t in_len = in_buf.size();

    std::vector<char> out_buf(in_len * 2); // 初始分配可能的两倍大小
    char* out_ptr = &out_buf[0];
    size_t out_len = out_buf.size();

    size_t res = iconv(conv, &in_ptr, &in_len, &out_ptr, &out_len);
    iconv_close(conv);

    if (res == (size_t)-1) {
        return "";
    }

    gbk_str.assign(&out_buf[0], out_buf.size() - out_len);
#endif
    return gbk_str;
}

std::string gbk_to_utf8(const std::string& gbk_str) {
    std::string utf8_str;
#ifdef _WIN32
    // Windows 系统下使用 MultiByteToWideChar 和 WideCharToMultiByte 函数
    int wide_len = MultiByteToWideChar(CP_ACP, 0, gbk_str.c_str(), -1, nullptr, 0);
    std::vector<wchar_t> wide_str(wide_len, 0);
    MultiByteToWideChar(CP_ACP, 0, gbk_str.c_str(), -1, wide_str.data(), wide_len);

    int utf8_len = WideCharToMultiByte(CP_UTF8, 0, wide_str.data(), -1, nullptr, 0, nullptr, nullptr);
    utf8_str.resize(utf8_len);
    WideCharToMultiByte(CP_UTF8, 0, wide_str.data(), -1, &utf8_str[0], utf8_len, nullptr, nullptr);
#else
    // Linux 系统下使用 iconv 函数
    iconv_t conv = iconv_open("UTF-8", "GBK");
    if (conv == (iconv_t)-1) {
        return "";
    }

    std::vector<char> in_buf(gbk_str.begin(), gbk_str.end());
    in_buf.push_back('\0');
    char* in_ptr = &in_buf[0];
    size_t in_len = in_buf.size();

    std::vector<char> out_buf(in_len * 3); // 初始分配可能的三倍大小
    char* out_ptr = &out_buf[0];
    size_t out_len = out_buf.size();

    size_t res = iconv(conv, &in_ptr, &in_len, &out_ptr, &out_len);
    iconv_close(conv);

    if (res == (size_t)-1) {
        return "";
    }

    utf8_str.assign(&out_buf[0], out_buf.size() - out_len);
#endif
    return utf8_str;
}
#endif // ENCODING_H