// frontend/vite.config.ts
import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import electron from 'vite-plugin-electron'
import renderer from 'vite-plugin-electron-renderer'

export default defineConfig({
  server: {
    open: false,
  },
  plugins: [
    svelte(),
    electron([
      {
        // Entry point for the main process
        entry: 'electron/main.ts',
      },
      {
        // Entry point for the preload script
        entry: 'electron/preload.ts',
        onready(options) {
          options.reload()
        },
      },
    ]),
    renderer(),
  ],
})