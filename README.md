# Webcam Viewer
**Webcam Viewer** is a Rust-based macOS application that allows users to access and display live video feeds from connected webcams in a resizable window. Designed for simplicity and performance, this application is perfect for developers and enthusiasts looking to integrate webcam functionalities into their workflows.

## Table of Contents
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
  - [1. Install OpenCV](#1-install-opencv)
  - [2. Build the Project](#2-build-the-project)
  - [3. Run the Application](#3-run-the-application)
- [Usage](#usage)
- [Packaging as a macOS Application](#packaging-as-a-macos-application)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgements](#acknowledgements)

## Features

- **Live Webcam Feed**: Access and display real-time video from your MacBook's built-in or external webcams.
- **Multiple Camera Support**: Detect and select from multiple connected cameras.
- **Resizable Window**: Dynamically adjust the application window size to fit your preferences.
- **Minimalist UI**: Focus solely on the webcam feed without any unnecessary UI elements.
- **High Performance**: Optimized for smooth video playback and efficient resource usage.

## Prerequisites

Before setting up Webcam Viewer, ensure you have the following installed on your macOS system:

- **Rust Toolchain**: [Install Rust](https://www.rust-lang.org/tools/install) via [rustup](https://rustup.rs/).
- **Homebrew**: [Install Homebrew](https://brew.sh/) for managing packages.
- **OpenCV**: OpenCV library for computer vision functionalities.
- **Xcode Command Line Tools**: Provides essential build tools and libraries.

## Installation

Follow these steps to set up and run Webcam Viewer on your Mac.

### 1. Install OpenCV

Use Homebrew to install OpenCV:

```bash 
brew install opencv
```
After installation, set the PKG_CONFIG_PATH environment variable so that Rust can locate OpenCV:
```bash
export PKG_CONFIG_PATH="$(brew --prefix opencv)/lib/pkgconfig"
```
To make this change permanent, add the above line to your shell profile (~/.bash_profile, ~/.zshrc, etc.). For example, if you’re using zsh:
```bash
echo 'export PKG_CONFIG_PATH="$(brew --prefix opencv)/lib/pkgconfig"' >> ~/.zshrc
source ~/.zshrc
```

### 2. Build the Project
Compile the project in release mode for optimal performance:
```bash
cargo build --release
```

### 3. Run the Application
```bash
./target/release/webcam_viewer
```

Upon running, the application will:

1.	Detect all connected webcams.
2.	If multiple cameras are found, prompt you to select one.
3.	Display the live video feed in a resizable window.
4.	Allow you to exit by pressing q or the <kbd>ESC</kbd> key.

## Usage
Selecting a Camera
```bash
Multiple cameras detected:
  0: Camera 0
  1: Camera 1
  2: Camera 2

Enter the number corresponding to the camera you want to use:
```
Type the desired number and press Enter to select the camera.
You can exit the Program with <kbd>ESC</kbd>.

## Packaging as a macOS Application
To create a standard macOS .app bundle for Webcam Viewer, follow these steps:

### 1. Build the Release Executable
Ensure you’ve built the project in release mode:
```bash
cargo build --release
```

### 2. Create the Application Bundle Structure
```bash
mkdir -p WebcamViewer.app/Contents/MacOS
mkdir -p WebcamViewer.app/Contents/Frameworks
mkdir -p WebcamViewer.app/Contents/Resources
```
### 3. Copy the Executable
```bash
cp target/release/webcam_viewer WebcamViewer.app/Contents/MacOS/
```

### 4. Bundle OpenCV Libraries
Identify and copy required OpenCV .dylib files:
```bash
otool -L target/release/webcam_viewer | grep opencv | awk '{print $1}' | while read lib; do
    cp "$lib" WebcamViewer.app/Contents/Frameworks/
done
```

### 5. Update Library Paths
Use install_name_tool to adjust library paths within the executable:
```bash
cd WebcamViewer.app/Contents/MacOS
for lib in ../Frameworks/*.dylib; do
    lib_name=$(basename "$lib")
    install_name_tool -change "$lib" "@executable_path/../Frameworks/$lib_name" webcam_viewer
done
```

### 6. Create Info.plist
Create and configure the Info.plist file:
```xml
cat <<EOF > WebcamViewer.app/Contents/Info.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>webcam_viewer</string>
    <key>CFBundleIdentifier</key>
    <string>com.yourname.webcamviewer</string>
    <key>CFBundleName</key>
    <string>Webcam Viewer</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>NSCameraUsageDescription</key>
    <string>This app requires access to the camera to display the webcam feed.</string>
</dict>
</plist>
EOF
```

### 7. (Optional) Add an Application Icon
1.	Create an .icns File: Use an icon editor or an online converter to create an .icns file.
2.	Copy the Icon: 
```bash
cp path_to_your_icon.icns WebcamViewer.app/Contents/Resources/
```
3.	Update Info.plist:
Add the following keys within the <dict> section:
```xml
<key>CFBundleIconFile</key>
<string>your_icon.icns</string>
<key>CFBundleIconName</key>
<string>your_icon</string>
```

### 8. Done
you can run the app now 🎉. Grant camera access when prompted.

## Troubleshooting

Common Issues

  - No Cameras Detected:
  -	Ensure your webcam is properly connected.
  -	Check camera permissions in System Preferences > Security & Privacy > Privacy > Camera.
	-	Application Crashes on Launch:
	  -	Verify all required .dylib files are present in Contents/Frameworks/.
	  -	Ensure library paths are correctly updated using install_name_tool.
	  -	Performance Lag:
  -	Lower the camera resolution in the code to reduce resource usage.
```bash
cam.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0)?;
cam.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0)?;
```

## Contributing

Contributions are welcome if you’d like to improve Webcam Viewer or add features.

## Reporting Issues

If you encounter any bugs or have suggestions for improvements, please open an issue.

## License

This project is licensed under the MIT License.

## Acknowledgements

-	[Rust Programming Language](https://www.rust-lang.org/)
-	[OpenCV](https://opencv.org/)
-	twistedfall/opencv-rust: [Rust bindings for OpenCV](https://github.com/twistedfall/opencv-rust)
-	[Homebrew](https://brew.sh/)
-	[Apple Developer Documentation](https://developer.apple.com/documentation/)
