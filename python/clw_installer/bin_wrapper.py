import os
import sys
import subprocess


def main():
    """
    Entry-point script that dispatches to the installed CLW binary.

    - On Unix: use os.execv() to replace the current process
    - On Windows: fallback to subprocess.run() because execv() doesn't work for .exe
    """

    # get_binary_path() will download the binary if it doesn't exist
    from . import get_binary_path

    try:
        binary_path = get_binary_path()
    except Exception as e:
        print(f"ERROR: Failed to get CLW binary: {e}", file=sys.stderr)
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
