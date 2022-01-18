import Vue from "vue";
import Vuex from "vuex";
import auth_module from "@/store/modules/auth.js";
import companies_module from "@/store/modules/companies.js";
import tags_module from "@/store/modules/tags.js";
import filter_module from "@/store/modules/filter.js";
import prepages_module from "@/store/modules/prepages.js";
import favorites from "@/store/modules/favorites";
import notes from "@/store/modules/notes";
import maps from "@/store/modules/maps";
import layouts from "@/store/modules/layouts";
import shortcuts from "@/store/modules/shortcuts";

Vue.use(Vuex);

export default new Vuex.Store({
  modules: {
    auth: auth_module,
    companies: companies_module,
    tags: tags_module,
    filter: filter_module,
    prepages: prepages_module,
    favorites: favorites,
    notes: notes,
    maps: maps,
    layouts: layouts,
    shortcuts: shortcuts,
  },
});
