import os
import sys
import subprocess
from pathlib import Path


def main():
    """
    Entry-point script that dispatches to the installed CLW binary.

    - On Unix: use os.execv() to replace the current process
    - On Windows: fallback to subprocess.run() because execv() doesn't work for .exe
    """

    # Directory: <site-packages>/clw_installer/bin
    bin_dir = Path(__file__).parent / "bin"

    # Binary name depends on platform
    if sys.platform.startswith("win"):
        binary_name = "clw.exe"
    else:
        binary_name = "clw"

    binary_path = bin_dir / binary_name

    # Ensure the binary exists (installer should have placed it there)
    if not binary_path.exists():
        print(f"ERROR: CLW binary not found at: {binary_path}", file=sys.stderr)
        print("Please reinstall: pip install --force-reinstall clw", file=sys.stderr)
        sys.exit(1)

    # Build full argument list: ["clw", arg1, arg2, ...]
    args = [str(binary_path)] + sys.argv[1:]

    # Replace current process (Unix)
    if not sys.platform.startswith("win"):
        os.execv(str(binary_path), args)

    # Windows fallback: subprocess.run
    try:
        result = subprocess.run(args)
        sys.exit(result.returncode)
    except OSError as e:
        print(f"Failed to run CLW: {e}", file=sys.stderr)
        sys.exit(1)
