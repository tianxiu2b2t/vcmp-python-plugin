from pathlib import Path

root = Path(__file__).parent.parent / "src" / "vcmp"


def create_py_file(file: Path):
    if file.suffix != ".pyi":
        return

    target = file.with_suffix(".py")
    with (
        open(target, "w", encoding="utf-8") as w,
        open(file, "r", encoding="utf-8") as r,
    ):
        w.write(r.read())


for file in root.rglob("*.pyi"):
    create_py_file(file)
