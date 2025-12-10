import { invoke } from '@tauri-apps/api/core'; // Tauri 2 import style

// This file is the main part of the user interface for StackHammer. It's written in a language called TypeScript, which helps create interactive parts of the app.
// The code here handles how users interact with the app, like dragging and dropping images to process them into tilesets.

const dropZone = document.getElementById('drop-zone') as HTMLDivElement;
const result = document.getElementById('result') as HTMLDivElement;
const tilesetImg = document.getElementById('tileset') as HTMLImageElement;
const saveBtn = document.getElementById('save-btn') as HTMLButtonElement;
// These lines find specific parts of the app's display, like the area where users drop images, the area to show the result, the image element to display the processed tileset, and the save button.

if (dropZone && result && tilesetImg && saveBtn) {
  dropZone.addEventListener('dragover', (e) => e.preventDefault());
  dropZone.addEventListener('drop', async (e) => {
// This section sets up actions for when a user drags an image over the drop area and then drops it.
// It prepares the app to read the image file and send it to the backend for processing.
    e.preventDefault();
    const file = e.dataTransfer?.files[0];
    if (!file) return;

    try {
      // Read the file content as a data URL to pass to the backend
      const reader = new FileReader();
      reader.onload = async (event) => {
        const dataUrl = event.target?.result as string;
        if (dataUrl) {
          // This part sends the image data to the backend part of the app (written in Rust) to process it into a tileset.
          const url = await invoke('extract_tileset', { dataUrl });
          tilesetImg.src = url as string; // Shows the processed tileset image in the app.
          result.style.display = 'block'; // Makes the result area visible to the user.
        }
      };
      reader.onerror = () => {
        console.error('Failed to read file');
      };
      reader.readAsDataURL(file);
    } catch (error) {
      console.error('Tile extraction failed:', error);
    }
  });

  saveBtn.addEventListener('click', async () => {
    try {
      // This sets up the save button to trigger a function in the backend that lets the user save the processed tileset as a file on their computer.
      await invoke('save_tileset');
    } catch (error) {
      console.error('Save failed:', error);
    }
  });
}
