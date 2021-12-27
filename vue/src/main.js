// The Vue build version to load with the `import` command
// (runtime-only or standalone) has been set in webpack.base.conf with an alias.
import Vue from "vue";
import axios from "@/plugins/axios";
import vuetify from "@/plugins/vuetify";
import Vuex from "vuex";
//import searchPlugin from "vuex-search";
import store from "./store";
import App from "./App";
import router from "./router";
import VueCookies from "vue-cookies";

Vue.config.productionTip = false;
Vue.use(Vuex);
Vue.use(VueCookies);
/* eslint-disable no-new */
new Vue({
  el: "#app",
  axios,
  router,
  vuetify,
  store,
  components: { App },
  template: "<App/>",
});
