import { app, BrowserWindow } from 'electron';
import { spawn, ChildProcess } from 'child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

// --- Modern __dirname Fix for ESM ---
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

let serverProcess: ChildProcess | null = null;

function startRustServer() {
  const isDev = !app.isPackaged;
  
  // Adjusted path to look back from frontend/dist-electron to backend/target
  const binaryPath = isDev 
    ? path.resolve(__dirname, '../../backend/target/x86_64-pc-windows-gnu/release/iracing-telemetry.exe')
    : path.join(process.resourcesPath, 'bin', 'iracing-telemetry.exe');

  console.log(`🚀 Attempting to start Rust Server at: ${binaryPath}`);

  serverProcess = spawn(binaryPath, [], {
    stdio: 'inherit',
    windowsHide: true 
  });

  serverProcess.on('error', (err) => {
    console.error('❌ Failed to start Rust server process:', err.message);
  });
}

function createMainWindow() {
  const win = new BrowserWindow({
    width: 1100,
    height: 700,
    title: "Pitwall Manager",
    webPreferences: {
      // Point to the compiled preload script in dist-electron
      preload: path.join(__dirname, 'preload.js'),
    },
  });

  if (process.env.VITE_DEV_SERVER_URL) {
    win.loadURL(process.env.VITE_DEV_SERVER_URL);
  } else {
    win.loadFile(path.join(__dirname, '../dist/index.html'));
  }
}

app.whenReady().then(() => {
  startRustServer();
  createMainWindow();
});

app.on('window-all-closed', () => {
  if (serverProcess) serverProcess.kill();
  if (process.platform !== 'darwin') app.quit();
});