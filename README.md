# StackHammer v0.1

## Overview

StackHammer is a desktop application designed to help game developers create tilesets from retro game screenshots. A tileset is a collection of small image pieces that can be used to build game levels or environments. This tool makes it easy to turn old game images into organized sets of tiles that are ready to use in game development platforms like Godot.

The app is built using a technology called Tauri, which combines different programming tools to create a fast and lightweight application. It processes images efficiently and provides a simple user interface to interact with.

## Key Components

StackHammer is made up of different parts that work together to achieve its goal:

- **Backend (Rust)**: This is the part of the app that does the heavy work. It's written in a programming language called Rust, which is very good at handling complex tasks like image processing. The backend takes a screenshot image, breaks it into small square pieces called tiles (each 16x16 pixels), finds unique tiles, and arranges them into a new image called a tileset. It also handles saving this tileset to your computer.

- **Frontend (TypeScript)**: This is the part of the app you see and interact with. It's written in a language called TypeScript, which helps create interactive experiences on the screen. The frontend lets you drag and drop an image file into the app, shows you the processed tileset, and provides a button to save it.

- **User Interface (HTML/CSS)**: These are the building blocks of how the app looks on your screen. HTML defines the structure, like where the drop area for images is, and CSS adds colors, sizes, and styles to make it visually appealing. Together, they create the layout where you can drop images, see results, and click to save.

## How It Works

Here's a simple step-by-step guide on how to use StackHammer:

1. **Open the App**: When you start StackHammer, you'll see a window with a dashed area labeled "Drop image here."
2. **Drop an Image**: Drag a retro game screenshot (like a PNG or JPG file) from your computer and drop it into this area. The app will read the image.
3. **Processing**: Behind the scenes, the app breaks the image into small 16x16 pixel tiles, finds the unique ones, and arranges them into a new image (the tileset).
4. **See the Result**: Once processing is done, a new section appears showing the tileset image with a "Save tileset.png" button.
5. **Save the Tileset**: Click the save button, choose a location on your computer, and save the tileset image file. This file is now ready to be imported into game development tools like Godot.

## Getting Started

If you're new to StackHammer and want to try it out or contribute to its development:

- **Download**: Check the releases section on GitHub (if available) to download the latest version of StackHammer for your computer.
- **Run**: Open the app on your computer. No complicated setup is needed.
- **Use**: Follow the steps above to process your retro screenshots into tilesets.
- **Contribute**: If you're a developer and want to help improve StackHammer, look at the code files in this repository. The main parts are explained with comments to help you understand what each does.

## File Structure

Here's a quick look at the important files in this project:

- **src-tauri/src/main.rs**: The backend code that processes images into tilesets (written in Rust).
- **src/main.ts**: The interactive part of the app that handles user actions like dropping images (written in TypeScript).
- **index.html**: The structure and layout of the app's visual interface (written in HTML).
- **src/styles.css**: The style settings that control colors and appearance of the app (written in CSS).

Each of these files has comments inside explaining what different sections do, making it easier for anyone to understand the code, even if they're not familiar with programming.

## License

[Add license information here if applicable, or state that it's to be determined.]

## Contact

[Add contact information or GitHub issues link for feedback or support if desired.]

Thank you for using StackHammer! We hope this tool helps bring your game development ideas to life with ease.
