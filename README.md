<div align="center">

# Python for VC:MP Server Plugin

![GitHub Issues or Pull Requests](https://img.shields.io/github/issues-pr/tianxiu2b2t/vcmp-python-plugin)
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/tianxiu2b2t/vcmp-python-plugin)
![GitHub License](https://img.shields.io/github/license/tianxiu2b2t/vcmp-python-plugin)
![GitHub Release](https://img.shields.io/github/v/release/tianxiu2b2t/vcmp-python-plugin)
![GitHub Tag](https://img.shields.io/github/v/tag/tianxiu2b2t/vcmp-python-plugin)
![GitHub Repo stars](https://img.shields.io/github/stars/tianxiu2b2t/vcmp-python-plugin)
<!-- [![Build](https://github.com/TTB-Network/python-openbmclapi/actions/workflows/build.yml/badge.svg)](https://github.com/TTB-Network/python-openbmclapi/actions/workflows/build.yml)
[![Docker Build](https://github.com/TTB-Network/python-openbmclapi/actions/workflows/docker_build.yml/badge.svg)](https://github.com/TTB-Network/python-openbmclapi/actions/workflows/docker_build.yml)
[![Release](https://github.com/TTB-Network/python-openbmclapi/actions/workflows/release.yml/badge.svg)](https://github.com/TTB-Network/python-openbmclapi/actions/workflows/release.yml) -->
[赞助](https://afdian.net/a/atianxiua)
</div>

## 许可证

[MIT](LICENSE)

## 贡献

如果你有能力，你可以向我们的仓库提交 Pull Request 或 Issue。

### 开发进度

目前 `master` 是主分支, ~~`rwir` 分支上正在将进行 rust 重写的工作~~, 在修一些小问题，欢迎贡献

> lets RWIR!

> 注：目前 `rwir` 分支合并成 `master` 分支，目前处于 `PreRelease` 版本（测试版），但目前的 `Release` 版本为 `1.2.10.6` 处于稳定状态

## 鸣谢

[ysc3839](https://github.com/ysc3839/vcmp-python-plugin) 提供一些 Python 开发思路
[shenjack](https://github.com/shenjackyuanjie/icalingua-bridge-bot) 帮助解决一些~~很屎的~~代码，并且提供了一些参考代码

## 使用

### 如果你是 Linux 用户 (Docker)

```bash
docker run -d --name vcmp-python -v /path/to/your/server:/app -p 8192:8192 tianxiu2b2t/vcmp-python server
```

### 如果你是 Windows 用户

1. pip 包安装
```bash
pip install vcmp-python-plugin
```

2. 下载对应的 python 版本 dll 文件，或在 pip 包中寻找对应的 dll 文件
3. 将 dll 文件放入你的 VCMP 服务器 plugins 文件夹中
4. 修改 server.cfg 文件，添加以下内容

```cfg
plugins python04rel64rspyo3py<python version>.dll
# 非必要不要开启……
# python_preloader false
python_script_path main.py
# 虚拟环境，比如 .venv/lib/python3.8/site-packages
# python_virtual_env 
# 日志等级，默认 INFO
# python_log_level INFO
# 检查更新，默认为 true
# python check_update true
# 是否记录日志，默认为 false
# python_file_log
```

5. 启动你的服务器
```bash
server.exe
```

## 示例脚本 ~~（等有缘人提供）~~
main.py:
```python

```