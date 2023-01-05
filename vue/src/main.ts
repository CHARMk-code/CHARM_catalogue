import { createApp, markRaw } from "vue";
import { createPinia } from "pinia";

import { piniaAxiosPlugin } from "@/plugins/axios";

import App from "@/App.vue";

import router from "@/router";

// Vuetify
// import 'vuetify/styles'
// import { createVuetify } from 'vuetify'
// import * as components from 'vuetify/components'
// import * as directives from 'vuetify/directives'

import { Quasar } from "quasar"

// Import icon libraries
import '@quasar/extras/material-icons/material-icons.css'

// Import Quasar css
import 'quasar/src/css/index.sass'

const app = createApp(App)

app.use(Quasar, {
    plugins: {}, // import Quasar plugins and add here
})



// Pinia
const pinia = createPinia();
app.use(pinia)
//injects axios into all pinia stores
pinia.use(piniaAxiosPlugin)

// Vuetify
// const customLightTheme = {
//     dark: false,
//     colors: {
//         primary: "#d60000",
//         secondary: "#9c0000",
//     },
// };
// const vuetify = createVuetify({
//     components,
//     directives,
//     theme: {
//         defaultTheme: 'customLightTheme',
//         themes: {
//             customLightTheme
//         }
//     }
// });

// app.use(vuetify)



//Router
app.use(router)
//router.app = app

app.mount('#app')
