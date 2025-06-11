from pathlib import Path


funcs: list[str] = []

# first raw.rs
RAW_FILE = Path("./crates/vcmp_bindings/src/raw.rs")

SRC_FILES = [
    Path("./crates/vcmp_bindings/src/func.rs"),
    Path("./crates/vcmp_bindings/src/func")
]

if __name__ == "__main__":
    with open(RAW_FILE, "r") as f:
        raw_content = f.read()
    
    start_pos = raw_content.find("pub struct PluginFuncs {")
    end_pos = raw_content.find("}\n", start_pos)
    content = raw_content[start_pos:end_pos].replace("\n", "")

    # find pub <func>: extern "C" fn(
    # find pub <func>: 
    while (start_pos := content.find("pub ")) != -1:
        end_pos = content.find(":", start_pos)
        func = content[start_pos + 4:end_pos].strip()
        funcs.append(func)
        content = content[end_pos + 1 :]

    refs = []

    for src_file in SRC_FILES:
        if src_file.is_dir():
            for file in src_file.rglob("*.rs"):
                with open(file, "r", encoding="utf-8") as f:
                    content = f.read()
                    for func in funcs:
                        if func in content:
                            refs.append(func)

        else:
            with open(src_file, "r", encoding="utf-8") as f:
                content = f.read()
                for func in funcs:
                    if func in content:
                        refs.append(func)

    unref = [func for func in funcs if func not in refs]
    print(unref)

