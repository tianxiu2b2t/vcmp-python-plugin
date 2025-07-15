import subprocess

if __name__ == "__main__":
    try:
        subprocess.run(["cargo", "+nightly", "fmt", "--all"])
    except subprocess.CalledProcessError:
        print("Failed to format all files, use stable")
        subprocess.run(["cargo", "fmt", "--all"])

    # py
    subprocess.run(["ruff", "format", "src"])
