import Vue from "vue";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.
export default {
  namespaced: true,
  state: () => ({
    settings: {},
    load_wait: 0
  }),
  mutations: {
    setSettings(state, settings){
      state.settings = settings;
    }
  },
  actions: {
    getSettings({ commit }) {
      return new Promise((resolve, reject) => {
        if (this.state.site_settings.load_wait < Date.now()) {
          this.state.site_settings.load_wait =
            Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          Vue.prototype.$axios
            .get("/site_settings")
            .then((resp) => {
              commit("setSettings", resp.data);
              resolve(resp);
            })
            .catch((err) => {
              reject(err);
            });
        } else {
          resolve();
        }
      });
    },
  },
  getters: {
    getServerMode: (state) => state.settings.server_mode,
  },
};
