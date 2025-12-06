"""
Binary installer for CLW (CSV Light Wizard).

Downloads the appropriate pre-built binary for the current platform.
"""

import platform
import subprocess
import sys
import urllib.request
import stat
from pathlib import Path


GITHUB_REPO = "sshu2017/clw"
VERSION = "0.1.3"  # Should match package version


def get_platform_info():
    """Detect the current platform and architecture, return binary name."""
    system = platform.system().lower()
    machine = platform.machine().lower()

    # Normalize architecture names
    if machine in ("x86_64", "amd64"):
        arch = "x86_64"
    elif machine in ("aarch64", "arm64"):
        arch = "aarch64"
    else:
        arch = machine

    # Map to binary names (matching GitHub release assets)
    if system == "linux":
        if arch == "x86_64":
            # Prefer musl for better compatibility
            return "clw-linux-x86_64-musl"
    elif system == "darwin":
        if arch == "x86_64":
            return "clw-macos-x86_64"
        elif arch == "aarch64":
            return "clw-macos-aarch64"
    elif system == "windows":
        if arch == "x86_64":
            return "clw-windows-x86_64.exe"

    raise RuntimeError(
        f"Unsupported platform: {system} {arch}. "
        "Please build from source: https://github.com/sshu2017/clw"
    )


def download_binary(url, dest_path):
    """Download a file with progress indication."""
    print(f"Downloading CLW binary from {url}...")

    try:
        with urllib.request.urlopen(url) as response:
            total_size = int(response.headers.get('content-length', 0))
            block_size = 8192
            downloaded = 0

            with open(dest_path, 'wb') as out_file:
                while True:
                    block = response.read(block_size)
                    if not block:
                        break
                    out_file.write(block)
                    downloaded += len(block)

                    if total_size > 0:
                        percent = (downloaded / total_size) * 100
                        print(f"\rProgress: {percent:.1f}%", end='', flush=True)

            print()  # New line after progress

    except Exception as e:
        raise RuntimeError(f"Failed to download binary: {e}")


def install_binary(force=False):
    """Download and install the CLW binary.

    Args:
        force: If True, always download even if binary exists with correct version.
               Use during pip install to ensure fresh binary.
    """
    try:
        binary_filename = get_platform_info()
    except RuntimeError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

    # Determine paths
    package_dir = Path(__file__).parent
    bin_dir = package_dir / "bin"
    bin_dir.mkdir(exist_ok=True)

    # Local binary name (standardized)
    binary_name = "clw.exe" if platform.system().lower() == "windows" else "clw"
    binary_path = bin_dir / binary_name

    # Check if binary already exists and validate version
    if binary_path.exists():
        if force:
            # Force re-download (used during pip install)
            print(f"Forcing fresh download of CLW binary v{VERSION}...")
            binary_path.unlink()
        else:
            # Normal mode: check version before re-downloading
            try:
                # Run binary to check version
                result = subprocess.run(
                    [str(binary_path), '--version'],
                    capture_output=True,
                    text=True,
                    timeout=5
                )
                # Check if the expected version is in the output
                if result.returncode == 0 and VERSION in result.stdout:
                    print(f"CLW binary v{VERSION} already installed at {binary_path}")
                    return str(binary_path)
                else:
                    # Version mismatch or command failed
                    print(f"Outdated or corrupted binary found (current: {result.stdout.strip()}), updating...")
                    binary_path.unlink()
            except (subprocess.TimeoutExpired, subprocess.SubprocessError, Exception) as e:
                # Binary is corrupted or failed to run
                print(f"Binary check failed ({e}), re-downloading...")
                binary_path.unlink()

    # Construct download URL for raw binary
    # Format: https://github.com/sshu2017/clw/releases/download/v0.1.0/clw-linux-x86_64-musl
    url = f"https://github.com/{GITHUB_REPO}/releases/download/v{VERSION}/{binary_filename}"

    try:
        # Download binary directly to destination
        download_binary(url, binary_path)

        # Make executable on Unix-like systems
        if platform.system().lower() != "windows":
            binary_path.chmod(binary_path.stat().st_mode | stat.S_IEXEC)

        print(f"Successfully installed CLW binary to {binary_path}")

    except Exception as e:
        # Clean up partial download on failure
        if binary_path.exists():
            binary_path.unlink()
        raise RuntimeError(f"Failed to install binary: {e}")

    return str(binary_path)


if __name__ == "__main__":
    install_binary()