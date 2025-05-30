from __future__ import annotations

# import string
import subprocess

from pathlib import Path

# BASIC_CMD = r"""bindgen .\c_src\plugin.h -o .\rs_src\bindings\raw.rs  --no-layout-tests --allowlist-item="(vcmp|Server|Plugin|PLUGIN).*""""

# BASIC_CMD = string.Template(" ".join([
#     "bindgen",
#     "./crates/bindings/src/v21.h",
#     "-o",
#     "./crates/bindings/src/${output}",
#     "--no-layout-tests",
#     '--allowlist-item="${items}"'
# ]))
# #  --blocklist-item <REGEX>

# # basic.rs
# # enums.rs
# # func.rs
# # callback.rs

# def gen_bindgen_cmd(output: Path, allows: str, black: str | None) -> str:
#     params = {
#         "output": str(output),
#         "items": allows
#     }
#     template = BASIC_CMD
#     if black is not None:
#         template.template += ' --blocklist-item="${black}"'
#         params['black'] = black

#     return template.safe_substitute(**params)

# FILES: dict[str, str] = {
#     "basic.rs": "(ServerSettings|PluginInfo)",
#     "enums.rs": "vcmp.*",
#     "func.rs": "PluginFuncs",
#     "callback.rs": "PluginCallbacks",
# }

# def gen_all_bindings():
#     for output, items in FILES.items():
#         cmd = BASIC_CMD.safe_substitute(output=output, items=items)
#         print(cmd)
#         subprocess.run(cmd, shell=True)
#         print("")


def gen_binding_and_process():
    subprocess.run(
        'bindgen ./crates/bindings/src/v21.h -o ./crates/bindings/src/raw.rs --no-layout-tests --allowlist-item="(vcmp|Server|Plugin|PLUGIN).*"',
        shell=True,
    )

    target_file = Path("./crates/bindings/src/raw.rs")

    with open(target_file, "r", encoding="utf-8") as bind_file:
        content = bind_file.read()

    option = "::std::option::Option<"
    # raw_cotent begin: pub struct PluginFuncs {
    # raw_content end: }\n
    raw_start_pos = content.find("pub struct PluginFuncs {")
    raw_end_pos = content.find("}\n", raw_start_pos)
    raw_content = content[raw_start_pos:raw_end_pos]
    while (start_pos := raw_content.find(option)) != -1:
        raw_content = raw_content[:start_pos] + raw_content[start_pos + len(option) :]
        if (right_comma := raw_content.find(">,")) != -1:
            if len(raw_content[right_comma - 1].strip()) == 1:
                raw_content = raw_content[:right_comma] + raw_content[right_comma + 1 :]
            else:
                left_newline = raw_content.rfind("\n", 0, right_comma)
                print(left_newline, raw_content[left_newline : right_comma + 1])
                raw_content = (
                    raw_content[: left_newline + 1] + raw_content[right_comma + 3 :]
                )

    content = content[:raw_start_pos] + raw_content + content[raw_end_pos:]

    with open(target_file, "w", encoding="utf-8") as bind_file:
        bind_file.write(content)

    subprocess.run(
        "cargo fmt",
        shell=True,
    )


if __name__ == "__main__":
    gen_binding_and_process()
    # gen_all_bindings()
