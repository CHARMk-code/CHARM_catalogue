import { createApp, markRaw } from "vue";
import { createPinia } from "pinia";

import axios from "@/plugins/axios";

import App from "@/App.vue";

import router from "@/router";

// Vuetify
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

const app = createApp(App)

// Pinia
const pinia = createPinia();
app.use(pinia)
//injects axios into all pinia stores
pinia.use(({ store }) => store.axios = markRaw(axios))

// Vuetify
const vuetify = createVuetify({ components, directives })
app.use(vuetify)



//Router
app.use(router)
router.app = app

app.mount('#app')
