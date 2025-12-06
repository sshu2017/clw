"""
CLW (CSV Light Wizard) - A blazing-fast CSV manipulation tool.

This is a Python wrapper that downloads and manages the pre-built
Rust binary for your platform.
"""

__version__ = "0.1.3"
__author__ = "CLW Contributors"
__license__ = "MIT"

from pathlib import Path


def get_binary_path():
    """Get the path to the CLW binary."""
    import platform

    package_dir = Path(__file__).parent
    binary_name = "clw.exe" if platform.system().lower() == "windows" else "clw"
    binary_path = package_dir / "bin" / binary_name

    if not binary_path.exists():
        # Binary doesn't exist, try to install it
        from .install import install_binary
        binary_path = Path(install_binary())

    return binary_path


__all__ = ["__version__", "get_binary_path"]