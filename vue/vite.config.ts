import { fileURLToPath, URL } from "node:url";

import vue from "@vitejs/plugin-vue"
import { defineConfig } from "vite";
import legacy from "@vitejs/plugin-legacy";
import { quasar, transformAssetUrls } from '@quasar/vite-plugin'
// import { VuetifyResolver } from 'unplugin-vue-components/resolvers';
// import Components from 'unplugin-vue-components/vite';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue({
    template: { transformAssetUrls }
  }),
  quasar({
    sassVariables: 'src/quasar-variables.sass'
  }),
  legacy({
    targets: ["ie >= 11"],
    additionalLegacyPolyfills: ["regenerator-runtime/runtime"],
  }),
    // Components({
    //   resolvers: [
    //     // Vuetify
    //     VuetifyResolver(),
    //   ],
    // })
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
});
