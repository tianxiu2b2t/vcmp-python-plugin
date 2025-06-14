import os
import sys
import subprocess

from pathlib import Path


def copy(src: Path, dest: Path):
    with open(dest, "wb") as w, open(src, "rb") as r:
        while data := r.read(1048576):
            w.write(data)


PLUGIN_NAME = "vcmp_plugin_rs.dll"
TARGET = Path("./target/release")
SERVER = Path("./server")
PLUGINS = SERVER / "plugins"

if __name__ == "__main__":
    args = sys.argv

    subprocess.run(["cargo", "build", "--release"], check=True)

    src = TARGET / PLUGIN_NAME

    if not src.exists():
        print("Error: DLL file does not exist.")
        sys.exit(1)

    dest = PLUGINS / PLUGIN_NAME

    PLUGINS.mkdir(exist_ok=True, parents=True)

    copy(src, dest)

    os.chdir(SERVER)
    try:
        subprocess.run(["./server64.exe", *sys.argv[1:]])

    except KeyboardInterrupt:
        print("Exiting...")
