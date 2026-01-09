// frontend/electron/preload.ts
import { contextBridge, ipcRenderer } from 'electron'

contextBridge.exposeInMainWorld('electronAPI', {
  // We will add methods here later to talk to the Rust server
  onTelemetry: (callback: any) => ipcRenderer.on('telemetry-data', callback)
})