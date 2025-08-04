#!/usr/bin/env python3
"""
Film Grain Generator - Cross-Platform Build and Install Script
Works on both macOS and Windows
"""

import os
import sys
import subprocess
import platform
import shutil
from pathlib import Path

# Colors for cross-platform terminal output
class Colors:
    if platform.system() == "Windows":
        # Windows doesn't support ANSI colors by default in older versions
        RED = GREEN = YELLOW = BLUE = CYAN = NC = ""
    else:
        RED = '\033[0;31m'
        GREEN = '\033[0;32m'
        YELLOW = '\033[1;33m'
        BLUE = '\033[0;34m'
        CYAN = '\033[0;36m'
        NC = '\033[0m'  # No Color

def print_colored(message, color=""):
    """Print colored message"""
    print(f"{color}{message}{Colors.NC}")

def print_header():
    """Print script header"""
    print()
    print_colored("🎬 Film Grain Generator - Cross-Platform Build & Install", Colors.CYAN)
    print_colored("=" * 60, Colors.CYAN)
    print()

def check_command(command):
    """Check if a command exists"""
    try:
        subprocess.run([command, "--version"], capture_output=True, check=True)
        return True
    except (subprocess.CalledProcessError, FileNotFoundError):
        return False

def run_command(command, description=""):
    """Run a command and handle errors"""
    if description:
        print_colored(f"🔄 {description}...", Colors.BLUE)
    
    try:
        result = subprocess.run(command, shell=True, check=True, capture_output=True, text=True)
        return True
    except subprocess.CalledProcessError as e:
        print_colored(f"❌ Error: {e}", Colors.RED)
        if e.stdout:
            print(e.stdout)
        if e.stderr:
            print(e.stderr)
        return False

def check_prerequisites():
    """Check if all required tools are installed"""
    print_colored("🔍 Checking prerequisites...", Colors.BLUE)
    
    required_tools = {
        "node": "Node.js",
        "npm": "npm", 
        "cargo": "Rust/Cargo"
    }
    
    missing_tools = []
    for command, name in required_tools.items():
        if check_command(command):
            print_colored(f"  ✅ {name} found", Colors.GREEN)
        else:
            print_colored(f"  ❌ {name} not found", Colors.RED)
            missing_tools.append(name)
    
    if missing_tools:
        print_colored(f"\n❌ Missing required tools: {', '.join(missing_tools)}", Colors.RED)
        print_colored("Please install the missing tools and try again.", Colors.YELLOW)
        return False
    
    print_colored("✅ All prerequisites found!", Colors.GREEN)
    return True

def build_app():
    """Build the application"""
    print_colored("📦 Installing dependencies...", Colors.BLUE)
    if not run_command("npm install", "Installing npm packages"):
        return False
    
    print_colored("🔨 Building Film Grain Generator...", Colors.BLUE)
    print_colored("This may take a few minutes...", Colors.YELLOW)
    
    if not run_command("npm run tauri build", "Building application"):
        return False
    
    print_colored("✅ Build completed successfully!", Colors.GREEN)
    return True

def install_macos():
    """Install on macOS"""
    app_path = Path("src-tauri/target/release/bundle/macos/Film Grain Generator.app")
    install_path = Path("/Applications/Film Grain Generator.app")
    
    if not app_path.exists():
        print_colored(f"❌ Built app not found at: {app_path}", Colors.RED)
        return False
    
    print_colored("📱 Installing to Applications folder...", Colors.BLUE)
    
    # Remove existing installation
    if install_path.exists():
        print_colored("🗑️  Removing existing installation...", Colors.YELLOW)
        shutil.rmtree(install_path)
    
    # Copy new app
    try:
        shutil.copytree(app_path, install_path)
        print_colored("✅ Successfully installed to Applications!", Colors.GREEN)
        
        # Set permissions
        os.chmod(install_path, 0o755)
        
        return True
    except Exception as e:
        print_colored(f"❌ Installation failed: {e}", Colors.RED)
        return False

def install_windows():
    """Install on Windows"""
    # Look for MSI installer
    msi_pattern = "src-tauri/target/release/bundle/msi/Film Grain Generator_*.msi"
    msi_files = list(Path(".").glob(msi_pattern))
    
    if not msi_files:
        print_colored(f"❌ MSI installer not found matching: {msi_pattern}", Colors.RED)
        return False
    
    msi_path = msi_files[0]
    print_colored(f"💿 Found installer: {msi_path}", Colors.GREEN)
    
    # Ask user if they want to run the installer
    response = input(f"{Colors.YELLOW}Would you like to run the installer now? (y/n): {Colors.NC}")
    if response.lower() in ['y', 'yes']:
        try:
            print_colored("🚀 Running installer...", Colors.BLUE)
            os.startfile(str(msi_path))
            return True
        except Exception as e:
            print_colored(f"❌ Failed to run installer: {e}", Colors.RED)
            return False
    else:
        print_colored(f"💿 Installer ready at: {msi_path.absolute()}", Colors.CYAN)
        return True

def show_completion_info():
    """Show completion information"""
    print()
    print_colored("🎉 Installation Complete!", Colors.GREEN)
    print_colored("=" * 30, Colors.GREEN)
    
    system = platform.system()
    if system == "Darwin":  # macOS
        print_colored("📱 App installed to: /Applications/Film Grain Generator.app", Colors.BLUE)
        print()
        print_colored("🚀 You can now:", Colors.YELLOW)
        print("   • Launch from Applications folder")
        print("   • Launch from Spotlight (⌘+Space, type 'Film Grain')")
        print("   • Launch from Dock (if you add it)")
        
        # Ask if user wants to launch
        response = input(f"\n{Colors.YELLOW}Would you like to launch Film Grain Generator now? (y/n): {Colors.NC}")
        if response.lower() in ['y', 'yes']:
            print_colored("🚀 Launching Film Grain Generator...", Colors.BLUE)
            subprocess.run(["open", "/Applications/Film Grain Generator.app"])
    
    elif system == "Windows":
        print_colored("🚀 You can now:", Colors.YELLOW)
        print("   • Launch from Start Menu")
        print("   • Launch from Desktop shortcut (if created)")
        print("   • Search for 'Film Grain Generator' in Windows Search")
    
    print()
    print_colored("📊 App Features:", Colors.BLUE)
    print("   • 40+ realistic film stocks with authentic characteristics")
    print("   • Multi-threaded grain rendering")
    print("   • Density slider (10K-50K grains)")
    print("   • Professional export (up to 2048x2048 PNG)")
    print("   • Film-specific colors, shapes, and clustering")
    print()
    print_colored("✨ Enjoy creating authentic film grain overlays!", Colors.GREEN)

def main():
    """Main script execution"""
    print_header()
    
    # Check if we're in the right directory
    if not Path("film-grain-generator").exists():
        print_colored("❌ Error: film-grain-generator directory not found!", Colors.RED)
        print_colored("Please run this script from the project root directory.", Colors.YELLOW)
        sys.exit(1)
    
    # Change to project directory
    print_colored("📁 Entering film-grain-generator directory...", Colors.BLUE)
    os.chdir("film-grain-generator")
    
    # Check prerequisites
    if not check_prerequisites():
        sys.exit(1)
    
    # Build the app
    if not build_app():
        print_colored("❌ Build failed!", Colors.RED)
        sys.exit(1)
    
    # Install based on platform
    system = platform.system()
    success = False
    
    if system == "Darwin":  # macOS
        success = install_macos()
    elif system == "Windows":
        success = install_windows()
    else:
        print_colored(f"❌ Unsupported platform: {system}", Colors.RED)
        print_colored("This script supports macOS and Windows only.", Colors.YELLOW)
        sys.exit(1)
    
    if success:
        show_completion_info()
    else:
        print_colored("❌ Installation failed!", Colors.RED)
        sys.exit(1)

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print_colored("\n\n❌ Installation cancelled by user.", Colors.YELLOW)
        sys.exit(1)
    except Exception as e:
        print_colored(f"\n❌ Unexpected error: {e}", Colors.RED)
        sys.exit(1)