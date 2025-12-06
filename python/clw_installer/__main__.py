"""
Main entry point for the CLW command.

This script executes the CLW binary and passes all arguments to it.
"""

import sys
import subprocess
from pathlib import Path


def main():
    """Execute the CLW binary with all provided arguments."""
    try:
        from . import get_binary_path

        binary_path = get_binary_path()

        if not binary_path.exists():
            print(f"Error: CLW binary not found at {binary_path}", file=sys.stderr)
            print("Please try reinstalling: pip install --force-reinstall clw", file=sys.stderr)
            sys.exit(1)

        # Execute the binary with all arguments
        result = subprocess.run(
            [str(binary_path)] + sys.argv[1:],
            check=False
        )

        sys.exit(result.returncode)

    except KeyboardInterrupt:
        sys.exit(130)  # Standard exit code for SIGINT
    except Exception as e:
        print(f"Error running CLW: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()