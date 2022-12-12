import Vue from "vue";
import axios from "@/plugins/axios";
import vuetify from "@/plugins/vuetify";
import Vuex from "vuex";
//import searchPlugin from "vuex-search";
import store from "./store";
import App from "./App.vue";
import router from "./router";

Vue.config.productionTip = false;
Vue.use(Vuex);

/* eslint-disable no-new */
new Vue({
  axios,
  router,
  vuetify,
  store,
  render: (h) => h(App),
}).$mount("#app");
