"""
Binary installer for CLW (CSV Light Wizard).

Downloads the appropriate pre-built binary for the current platform.
"""

import sys
import stat
import platform
import subprocess
import urllib.request
from pathlib import Path
from importlib.metadata import version as pkg_version, PackageNotFoundError

GITHUB_REPO = "sshu2017/clw"

def get_package_version():
    """Return the installed wheel version (e.g. '0.1.4')."""
    try:
        return pkg_version("clw")
    except PackageNotFoundError:
        print("ERROR: Unable to determine package version for 'clw'.", file=sys.stderr)
        sys.exit(1)


def get_platform_binary_name():
    """Detect platform + architecture and return the correct release asset name."""
    system = platform.system().lower()
    machine = platform.machine().lower()

    if machine in ("x86_64", "amd64"):
        arch = "x86_64"
    elif machine in ("aarch64", "arm64"):
        arch = "aarch64"
    else:
        arch = machine

    if system == "linux":
        if arch == "x86_64":
            return "clw-linux-x86_64-musl"
        elif arch == "aarch64":
            return "clw-linux-aarch64-musl"

    elif system == "darwin":
        if arch == "x86_64":
            return "clw-macos-x86_64"
        elif arch == "aarch64":
            return "clw-macos-aarch64"

    elif system == "windows":
        if arch == "x86_64":
            return "clw-windows-x86_64.exe"

    raise RuntimeError(f"Unsupported platform: {system} {arch}")


def download_binary(url: str, dest_path: Path):
    """Download binary with progress output."""
    print(f"Downloading CLW binary from:\n  {url}")

    try:
        with urllib.request.urlopen(url) as response:
            total_size = int(response.headers.get("content-length", 0))
            downloaded = 0
            block_size = 8192

            with open(dest_path, "wb") as f:
                while True:
                    block = response.read(block_size)
                    if not block:
                        break
                    f.write(block)
                    downloaded += len(block)

                    if total_size > 0:
                        percent = (downloaded / total_size) * 100
                        print(f"\rProgress: {percent:.1f}%", end="", flush=True)

        print()

    except Exception as e:
        raise RuntimeError(f"Download failed: {e}")


def install_binary(force: bool = False):
    """Install the CLW binary matching the wheel version."""
    version = get_package_version()
    print(f"Detected CLW package version: {version}")

    try:
        binary_filename = get_platform_binary_name()
    except RuntimeError as e:
        print(e, file=sys.stderr)
        sys.exit(1)

    package_dir = Path(__file__).parent
    bin_dir = package_dir / "bin"
    bin_dir.mkdir(exist_ok=True)

    local_name = "clw.exe" if platform.system().lower() == "windows" else "clw"
    binary_path = bin_dir / local_name

    # If binary exists, see if it is already correct version
    if binary_path.exists() and not force:
        try:
            result = subprocess.run(
                [str(binary_path), "--version"],
                capture_output=True,
                text=True,
                timeout=5,
            )
            if result.returncode == 0 and version in result.stdout:
                print(f"CLW binary v{version} already installed at {binary_path}")
                return str(binary_path)
            else:
                print("Outdated/corrupted binary detected. Re-downloading...")
                binary_path.unlink()
        except Exception:
            print("Binary check failed. Re-downloading...")
            binary_path.unlink()

    # Download URL: release tag must be vX.Y.Z
    url = f"https://github.com/{GITHUB_REPO}/releases/download/v{version}/{binary_filename}"

    try:
        download_binary(url, binary_path)

        if platform.system().lower() != "windows":
            binary_path.chmod(binary_path.stat().st_mode | stat.S_IEXEC)

        print(f"Successfully installed CLW binary to {binary_path}")

    except Exception as e:
        if binary_path.exists():
            binary_path.unlink()
        raise

    return str(binary_path)


if __name__ == "__main__":
    install_binary(force=True)