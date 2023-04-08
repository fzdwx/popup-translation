import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import vueJsx from '@vitejs/plugin-vue-jsx'
import UnoCSS from 'unocss/vite'

const mobile =
  process.env.TAURI_PLATFORM === 'android' ||
  process.env.TAURI_PLATFORM === 'ios'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vueJsx({}),
    vue(),
    UnoCSS({
      configFile: './unocss.config.ts'
    }),
    AutoImport({
      dts: './src/types/auto-import.d.ts',
      eslintrc: {
        enabled: true
      },
      imports: ['vue', '@vueuse/core'],
      vueTemplate: true,
      dirs: ['./src/api/*', './src/command/*', './src/utils/*', './src/use/*']
    }),
    Components({
      dts: './src/types/components.d.ts',
      dirs: ['./src/components/Setting/*']
    })
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  resolve: {
    alias: {
      '@': '/src'
    }
  },
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG
  }
}))
