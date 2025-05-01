#include <string>
#include <vector>
#include <stdexcept>
#include <windows.h>

// UTF-8 转 GBK
std::string utf8_to_gbk(const std::string& utf8_str) {
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
    std::vector<char> gbk_buf(gbk_char_count, 0); // 使用 std::vector<char> 来存储可写缓冲区
    WideCharToMultiByte(CP_ACP, 0, wide_buf.data(), -1, gbk_buf.data(), gbk_char_count, nullptr, nullptr);

    return std::string(gbk_buf.begin(), gbk_buf.end() - 1); // 转换为 std::string 并去掉末尾的空字符
}

// GBK 转 UTF-8
std::string gbk_to_utf8(const std::string& gbk_str) {
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
    std::vector<char> utf8_buf(utf8_char_count, 0); // 使用 std::vector<char> 来存储可写缓冲区
    WideCharToMultiByte(CP_UTF8, 0, wide_buf.data(), -1, utf8_buf.data(), utf8_char_count, nullptr, nullptr);

    return std::string(utf8_buf.begin(), utf8_buf.end() - 1); // 转换为 std::string 并去掉末尾的空字符
}