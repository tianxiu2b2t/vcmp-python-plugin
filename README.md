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

目前 `master` 是主分支, `rwir` 分支上正在将进行 rust 重写的工作, 欢迎贡献

> lets RWIR!

> 注：目前 `master` 分支暂不维护，目前 `Release` 版本为 `1.2.10.6` 处于稳定状态

## 鸣谢

[ysc3839](https://github.com/ysc3839/vcmp-python-plugin)

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
plugins vcmp-python-plugin-cpy<py_version>-rel64.dll
pythonscript main.py
```

5. 启动你的服务器
```bash
server.exe
```

## 示例脚本
main.py:
```python
import sys
from vcmp import callbacks, run
from vcmp.events.server import ServerInitialiseEvent
from vcmp.events.player import PlayerCommandEvent, PlayerEnterVehicleEvent, PlayerRequestEnterVehicleEvent
from vcmp.callbacks import Matcher
from vcmp.functions import plugin, server, vehicle

from vcmp.instance import get_vehicle_from_id

@callbacks.on()
def server_initialise(event: ServerInitialiseEvent):
    print("Server Loaded")

@callbacks.on()
def _(matcher: Matcher, event: PlayerRequestEnterVehicleEvent):
    matcher.send("You request enter a vehicle.")
    matcher.finish()

@callbacks.on()
def _(matcher: Matcher, event: PlayerEnterVehicleEvent):
    matcher.send("You enter a vehicle.")
    matcher.finish()

@callbacks.on()
def command(matcher: Matcher, event: PlayerCommandEvent):
    cmd = event.cmd.lower()
    text = event.text
    if cmd == "car":
        if not text.isdigit():
            matcher.send("Unable to spawn car. Please enter a valid car id.")
            matcher.finish()

        car_id = int(text)
        player = event.player
        veh = vehicle.create_vehicle(car_id, player.world, player.position, player.angle)
        player.vehicle = veh
        matcher.send(f"Spawned car with id {car_id}.")
        matcher.finish()

    if cmd == "pos":
        player = event.player
        matcher.send(f"Your position is {player.position}.")
        matcher.finish()

    if cmd == "eject":
        player = event.player
        player.vehicle = None

    if cmd == "veh":
        player = event.player
        if not text.isdigit():
            matcher.send("Please enter a valid car index id.")
            matcher.finish()
        
        index = int(text)
        veh = get_vehicle_from_id(index)
        if veh is None:
            matcher.send("Unable to find vehicle.")
            matcher.finish()

        player.vehicle = veh
        matcher.send(f"Teleported to vehicle with index {index}.")
        matcher.finish()

def main():
    #server.set_servername(f"Python Test Server of Python {sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}")
    #print(server.get_server_version(), server.get_servername())
    #print(plugin.get_plugins())
    print(f"Server runs on {server.get_server_version()} version")

if __name__ == '__main__':
    #run(main) # 避免阻塞主线程，也可以在主线程调用 （强烈推荐 run(main) ）
    print(
        "Python is running by __main__"
    )
```