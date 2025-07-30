from __future__ import annotations

import subprocess

from pathlib import Path

def gen_binding_and_process():
    print("bindgen 生成中")
    subprocess.run(
        'bindgen ./crates/squirrel_ffi/src/sq_ffi/SQImports.h -o ./crates/squirrel_ffi/src/sq_ffi/raw.rs --no-layout-tests',
        shell=True,
    )
    print("bindgen 生成完成, 开始处理 Option")

    target_file = Path("./crates/squirrel_ffi/src/sq_ffi/raw.rs")

    with open(target_file, "r", encoding="utf-8") as bind_file:
        content = bind_file.read()

    option = "::std::option::Option<"
    # pub struct sq_api {
    raw_start_pos = content.find("pub struct sq_api {")
    raw_end_pos = content.find("}\n", raw_start_pos)
    raw_content = content[raw_start_pos:raw_end_pos]
    while (start_pos := raw_content.find(option)) != -1:
        raw_content = raw_content[:start_pos] + raw_content[start_pos + len(option) :]
        if (right_comma := raw_content.find(">,")) != -1:
            if len(raw_content[right_comma - 1].strip()) == 1:
                raw_content = raw_content[:right_comma] + raw_content[right_comma + 1 :]
            else:
                left_newline = raw_content.rfind("\n", 0, right_comma)
                raw_content = (
                    raw_content[: left_newline + 1] + raw_content[right_comma + 3 :]
                )

    print("处理 Option 完成")

    print("处理 unsafe extern")

    raw_content = raw_content.replace("unsafe extern", "extern")

    print("处理 unsafe extern 完成")

    content = content[:raw_start_pos] + raw_content + content[raw_end_pos:]

    with open(target_file, "w", encoding="utf-8") as bind_file:
        bind_file.write(content)

    print("写入完成, fmt")

    subprocess.run("cargo fmt", shell=True)

    print("fmt 完成 DONE!")


if __name__ == "__main__":
    gen_binding_and_process()
