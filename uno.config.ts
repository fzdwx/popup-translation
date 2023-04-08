// uno.config.ts
import { defineConfig } from 'unocss'
import { presetIcons, presetAttributify, presetUno } from 'unocss'
import transformerAttributifyJsx from '@unocss/transformer-attributify-jsx'

export default defineConfig({
  presets: [
    presetIcons({
      /* options */
    }),
    presetAttributify(),
    presetUno()
  ],
  transformers: [
    transformerAttributifyJsx() // <--
  ]
})
