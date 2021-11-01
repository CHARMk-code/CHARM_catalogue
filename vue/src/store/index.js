import Vue from "vue";
import Vuex from "vuex";
import auth_module from "@/store/modules/auth.js";
import companies_module from "@/store/modules/companies.js";
import tags_module from "@/store/modules/tags.js";
import prepages_module from "@/store/modules/prepages.js";

Vue.use(Vuex);

export default new Vuex.Store({
  modules: {
    auth: auth_module,
    companies: companies_module,
    tags: tags_module,
    prepages: prepages_module,
  },
});
