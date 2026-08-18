import { fileURLToPath } from 'node:url'
import { mergeConfig, defineConfig, configDefaults } from 'vitest/config'
import viteConfig from './vite.config'

export default mergeConfig(
  viteConfig,
  defineConfig({
    test: {
      environment: 'jsdom',
      exclude: [...configDefaults.exclude, 'e2e/**'],
      root: fileURLToPath(new URL('./', import.meta.url)),
      // Vuetify раздаётся как ESM с импортами .css — без inline Node пытается
      // грузить их сам и падает на «Unknown file extension .css».
      server: { deps: { inline: ['vuetify'] } },
    },
  }),
)
