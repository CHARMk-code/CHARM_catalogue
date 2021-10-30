import Vue from "vue";
import Vuex from "vuex";
import auth_module from "@/store/modules/auth.js";
import companies_module from "@/store/modules/companies.js";

Vue.use(Vuex);

export default new Vuex.Store({
  modules: {
    auth: auth_module,
    companies: companies_module,
  },
});
