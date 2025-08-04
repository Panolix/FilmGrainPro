#!/usr/bin/env python3
"""
🎬 Film Grain Generator - Universal Installer
Cross-platform installer that works on macOS, Windows, and Linux
Just double-click this file to install!
"""

import os
import sys
import subprocess
import platform
import shutil
import tkinter as tk
from tkinter import messagebox, ttk
from pathlib import Path
import threading
import time

class FilmGrainInstaller:
    def __init__(self):
        self.system = platform.system()
        self.root = None
        self.progress_var = None
        self.status_var = None
        self.log_text = None
        
    def show_gui(self):
        """Show the GUI installer"""
        self.root = tk.Tk()
        self.root.title("🎬 Film Grain Generator Installer")
        self.root.geometry("600x500")
        self.root.resizable(False, False)
        
        # Configure style
        style = ttk.Style()
        if self.system == "Darwin":  # macOS
            style.theme_use('aqua')
        elif self.system == "Windows":
            style.theme_use('vista')
        else:  # Linux
            style.theme_use('clam')
        
        # Main frame
        main_frame = ttk.Frame(self.root, padding="20")
        main_frame.grid(row=0, column=0, sticky=(tk.W, tk.E, tk.N, tk.S))
        
        # Title
        title_label = ttk.Label(main_frame, text="🎬 Film Grain Generator", 
                               font=("Arial", 20, "bold"))
        title_label.grid(row=0, column=0, columnspan=2, pady=(0, 10))
        
        # Description
        desc_text = """Professional film grain generator with 40+ authentic film stocks.
Creates realistic grain overlays based on actual film characteristics.

This installer will:
• Check system requirements
• Install dependencies automatically  
• Build and install the application
• Set up desktop shortcuts"""
        
        desc_label = ttk.Label(main_frame, text=desc_text, justify=tk.LEFT)
        desc_label.grid(row=1, column=0, columnspan=2, pady=(0, 20), sticky=tk.W)
        
        # System info
        system_info = f"Platform: {self.system} {platform.release()}\nPython: {sys.version.split()[0]}"
        info_label = ttk.Label(main_frame, text=system_info, font=("Courier", 10))
        info_label.grid(row=2, column=0, columnspan=2, pady=(0, 20), sticky=tk.W)
        
        # Progress bar
        self.progress_var = tk.DoubleVar()
        progress_bar = ttk.Progressbar(main_frame, variable=self.progress_var, 
                                     maximum=100, length=400)
        progress_bar.grid(row=3, column=0, columnspan=2, pady=(0, 10), sticky=(tk.W, tk.E))
        
        # Status label
        self.status_var = tk.StringVar(value="Ready to install")
        status_label = ttk.Label(main_frame, textvariable=self.status_var)
        status_label.grid(row=4, column=0, columnspan=2, pady=(0, 10))
        
        # Log text area
        log_frame = ttk.LabelFrame(main_frame, text="Installation Log", padding="10")
        log_frame.grid(row=5, column=0, columnspan=2, pady=(0, 20), sticky=(tk.W, tk.E, tk.N, tk.S))
        
        self.log_text = tk.Text(log_frame, height=12, width=70, font=("Courier", 9))
        scrollbar = ttk.Scrollbar(log_frame, orient=tk.VERTICAL, command=self.log_text.yview)
        self.log_text.configure(yscrollcommand=scrollbar.set)
        
        self.log_text.grid(row=0, column=0, sticky=(tk.W, tk.E, tk.N, tk.S))
        scrollbar.grid(row=0, column=1, sticky=(tk.N, tk.S))
        
        # Buttons
        button_frame = ttk.Frame(main_frame)
        button_frame.grid(row=6, column=0, columnspan=2, pady=(10, 0))
        
        self.install_button = ttk.Button(button_frame, text="🚀 Install Film Grain Generator", 
                                        command=self.start_installation, style="Accent.TButton")
        self.install_button.pack(side=tk.LEFT, padx=(0, 10))
        
        self.quit_button = ttk.Button(button_frame, text="Cancel", command=self.root.quit)
        self.quit_button.pack(side=tk.LEFT)
        
        # Configure grid weights
        self.root.columnconfigure(0, weight=1)
        self.root.rowconfigure(0, weight=1)
        main_frame.columnconfigure(0, weight=1)
        main_frame.rowconfigure(5, weight=1)
        log_frame.columnconfigure(0, weight=1)
        log_frame.rowconfigure(0, weight=1)
        
        # Center window
        self.root.update_idletasks()
        x = (self.root.winfo_screenwidth() // 2) - (600 // 2)
        y = (self.root.winfo_screenheight() // 2) - (500 // 2)
        self.root.geometry(f"600x500+{x}+{y}")
        
        self.log("🎬 Film Grain Generator Installer Ready")
        self.log(f"Platform: {self.system}")
        self.log("Click 'Install' to begin...")
        
        self.root.mainloop()
    
    def log(self, message):
        """Add message to log"""
        if self.log_text:
            self.log_text.insert(tk.END, f"{message}\n")
            self.log_text.see(tk.END)
            self.root.update_idletasks()
        print(message)
    
    def update_status(self, message):
        """Update status label"""
        if self.status_var:
            self.status_var.set(message)
            self.root.update_idletasks()
    
    def update_progress(self, value):
        """Update progress bar"""
        if self.progress_var:
            self.progress_var.set(value)
            self.root.update_idletasks()
    
    def start_installation(self):
        """Start installation in a separate thread"""
        self.install_button.config(state="disabled")
        self.quit_button.config(text="Close")
        
        # Start installation in background thread
        install_thread = threading.Thread(target=self.run_installation)
        install_thread.daemon = True
        install_thread.start()
    
    def run_installation(self):
        """Run the actual installation process"""
        try:
            self.update_status("Checking prerequisites...")
            self.update_progress(10)
            
            if not self.check_prerequisites():
                return
            
            self.update_status("Checking project structure...")
            self.update_progress(20)
            
            if not self.check_project_structure():
                return
            
            self.update_status("Installing dependencies...")
            self.update_progress(30)
            
            if not self.install_dependencies():
                return
            
            self.update_status("Building application...")
            self.update_progress(50)
            
            if not self.build_application():
                return
            
            self.update_status("Installing application...")
            self.update_progress(80)
            
            if not self.install_application():
                return
            
            self.update_status("✅ Installation complete!")
            self.update_progress(100)
            
            self.log("🎉 Installation completed successfully!")
            self.show_completion_dialog()
            
        except Exception as e:
            self.log(f"❌ Installation failed: {str(e)}")
            self.update_status("❌ Installation failed")
            messagebox.showerror("Installation Error", f"Installation failed:\n{str(e)}")
    
    def check_prerequisites(self):
        """Check if all required tools are installed"""
        self.log("🔍 Checking prerequisites...")
        
        required_tools = {
            "node": "Node.js",
            "npm": "npm",
            "cargo": "Rust/Cargo"
        }
        
        missing_tools = []
        for command, name in required_tools.items():
            if self.check_command(command):
                self.log(f"  ✅ {name} found")
            else:
                self.log(f"  ❌ {name} not found")
                missing_tools.append(name)
        
        if missing_tools:
            error_msg = f"Missing required tools: {', '.join(missing_tools)}\n\n"
            error_msg += "Please install:\n"
            if "Node.js" in missing_tools:
                error_msg += "• Node.js from https://nodejs.org\n"
            if "Rust/Cargo" in missing_tools:
                error_msg += "• Rust from https://rustup.rs\n"
            
            messagebox.showerror("Missing Prerequisites", error_msg)
            return False
        
        self.log("✅ All prerequisites found!")
        return True
    
    def check_command(self, command):
        """Check if a command exists"""
        try:
            subprocess.run([command, "--version"], capture_output=True, check=True)
            return True
        except (subprocess.CalledProcessError, FileNotFoundError):
            return False
    
    def check_project_structure(self):
        """Check if we're in the right directory"""
        if not Path("film-grain-generator").exists():
            error_msg = "film-grain-generator directory not found!\n\n"
            error_msg += "Please make sure this installer is in the same folder as the film-grain-generator directory."
            messagebox.showerror("Project Not Found", error_msg)
            return False
        
        self.log("✅ Project structure found")
        return True
    
    def install_dependencies(self):
        """Install npm dependencies"""
        self.log("📦 Installing npm dependencies...")
        
        try:
            os.chdir("film-grain-generator")
            result = subprocess.run(["npm", "install"], capture_output=True, text=True, check=True)
            self.log("✅ Dependencies installed successfully")
            return True
        except subprocess.CalledProcessError as e:
            self.log(f"❌ Failed to install dependencies: {e}")
            self.log(f"Error output: {e.stderr}")
            return False
        finally:
            os.chdir("..")
    
    def build_application(self):
        """Build the Tauri application"""
        self.log("🔨 Building Film Grain Generator...")
        self.log("This may take several minutes...")
        
        try:
            os.chdir("film-grain-generator")
            
            # Run build command
            process = subprocess.Popen(
                ["npm", "run", "tauri", "build"],
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                universal_newlines=True
            )
            
            # Stream output to log
            for line in process.stdout:
                if line.strip():
                    self.log(f"  {line.strip()}")
            
            process.wait()
            
            if process.returncode == 0:
                self.log("✅ Build completed successfully")
                return True
            else:
                self.log(f"❌ Build failed with exit code {process.returncode}")
                return False
                
        except Exception as e:
            self.log(f"❌ Build error: {e}")
            return False
        finally:
            os.chdir("..")
    
    def install_application(self):
        """Install the built application"""
        self.log(f"📱 Installing for {self.system}...")
        
        if self.system == "Darwin":  # macOS
            return self.install_macos()
        elif self.system == "Windows":
            return self.install_windows()
        elif self.system == "Linux":
            return self.install_linux()
        else:
            self.log(f"❌ Unsupported platform: {self.system}")
            return False
    
    def install_macos(self):
        """Install on macOS"""
        app_path = Path("film-grain-generator/src-tauri/target/release/bundle/macos/Film Grain Generator.app")
        install_path = Path("/Applications/Film Grain Generator.app")
        
        if not app_path.exists():
            self.log(f"❌ Built app not found at: {app_path}")
            return False
        
        try:
            # Remove existing installation
            if install_path.exists():
                self.log("🗑️  Removing existing installation...")
                shutil.rmtree(install_path)
            
            # Copy new app
            self.log("📱 Copying to Applications folder...")
            shutil.copytree(app_path, install_path)
            
            # Set permissions
            os.chmod(install_path, 0o755)
            
            self.log("✅ Successfully installed to Applications!")
            return True
            
        except Exception as e:
            self.log(f"❌ Installation failed: {e}")
            return False
    
    def install_windows(self):
        """Install on Windows"""
        # Look for MSI installer
        msi_pattern = "film-grain-generator/src-tauri/target/release/bundle/msi/Film Grain Generator_*.msi"
        msi_files = list(Path(".").glob(msi_pattern))
        
        if not msi_files:
            self.log(f"❌ MSI installer not found")
            return False
        
        msi_path = msi_files[0]
        self.log(f"💿 Found installer: {msi_path}")
        
        try:
            self.log("🚀 Running installer...")
            subprocess.run(["msiexec", "/i", str(msi_path), "/quiet"], check=True)
            self.log("✅ Installation completed!")
            return True
        except Exception as e:
            self.log(f"❌ Failed to run installer: {e}")
            # Try opening the MSI file directly
            try:
                os.startfile(str(msi_path))
                self.log("📂 Opened installer manually")
                return True
            except:
                return False
    
    def install_linux(self):
        """Install on Linux"""
        # Look for AppImage or deb package
        appimage_files = list(Path("film-grain-generator/src-tauri/target/release/bundle/appimage").glob("*.AppImage"))
        deb_files = list(Path("film-grain-generator/src-tauri/target/release/bundle/deb").glob("*.deb"))
        
        if appimage_files:
            appimage_path = appimage_files[0]
            install_path = Path.home() / "Applications" / "Film Grain Generator.AppImage"
            
            try:
                # Create Applications directory if it doesn't exist
                install_path.parent.mkdir(exist_ok=True)
                
                # Copy AppImage
                shutil.copy2(appimage_path, install_path)
                os.chmod(install_path, 0o755)
                
                self.log(f"✅ AppImage installed to: {install_path}")
                return True
            except Exception as e:
                self.log(f"❌ AppImage installation failed: {e}")
                return False
        
        elif deb_files:
            deb_path = deb_files[0]
            try:
                subprocess.run(["sudo", "dpkg", "-i", str(deb_path)], check=True)
                self.log("✅ Debian package installed!")
                return True
            except Exception as e:
                self.log(f"❌ Debian package installation failed: {e}")
                return False
        
        else:
            self.log("❌ No Linux installer found")
            return False
    
    def show_completion_dialog(self):
        """Show completion dialog with launch option"""
        message = "🎉 Film Grain Generator has been installed successfully!\n\n"
        
        if self.system == "Darwin":
            message += "You can now:\n"
            message += "• Launch from Applications folder\n"
            message += "• Search with Spotlight (⌘+Space)\n"
            message += "• Add to Dock for quick access\n\n"
            message += "Would you like to launch the app now?"
        elif self.system == "Windows":
            message += "You can now:\n"
            message += "• Launch from Start Menu\n"
            message += "• Search for 'Film Grain Generator'\n"
            message += "• Use Desktop shortcut (if created)\n\n"
            message += "Would you like to launch the app now?"
        else:  # Linux
            message += "Installation complete!\n"
            message += "Check your applications menu or run from terminal."
        
        if messagebox.askyesno("Installation Complete", message):
            self.launch_app()
    
    def launch_app(self):
        """Launch the installed application"""
        try:
            if self.system == "Darwin":
                subprocess.run(["open", "/Applications/Film Grain Generator.app"])
            elif self.system == "Windows":
                # Try to find and launch the installed app
                subprocess.run(["start", "Film Grain Generator"], shell=True)
            else:  # Linux
                # Try common launch methods
                try:
                    subprocess.run(["film-grain-generator"])
                except:
                    appimage_path = Path.home() / "Applications" / "Film Grain Generator.AppImage"
                    if appimage_path.exists():
                        subprocess.run([str(appimage_path)])
            
            self.log("🚀 Launched Film Grain Generator!")
        except Exception as e:
            self.log(f"❌ Failed to launch app: {e}")

def main():
    """Main entry point"""
    try:
        # Try to show GUI
        installer = FilmGrainInstaller()
        installer.show_gui()
    except ImportError:
        # Fallback to command line if tkinter is not available
        print("GUI not available, falling back to command line installer...")
        print("Please install tkinter or run: python3 build-and-install.py")
    except Exception as e:
        print(f"Error: {e}")
        input("Press Enter to exit...")

if __name__ == "__main__":
    main()