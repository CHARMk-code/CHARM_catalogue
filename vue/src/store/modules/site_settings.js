import Vue from "vue";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.
export default {
  namespaced: true,
  state: () => ({
    settings: {
      company_view: {
        cards: [],
      },
    },
    load_wait: 0,
  }),
  mutations: {
    setSettings(state, settings) {
      state.settings = settings;
    },
    setCompanyCards(state, cards) {
      state.settings.company_view.cards = cards;
    },
  },
  actions: {
    getCompanyCards({ commit }, force = false) {
      return new Promise((resolve, reject) => {
        if (force || this.state.site_settings.load_wait < Date.now()) {
          this.state.site_settings.load_wait =
            Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          Vue.prototype.$axios
            .get("/settings/company_view")
            .then((resp) => {
              commit("setCompanyCards", resp.data);
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
    setCompanyCards({ commit, state }, cards) {
      return new Promise((resolve) => {
        const all_cards = state.settings.company_view.cards;

        const new_cards = all_cards.map((ac) => ({
          ...ac,
          active: cards.some((c) => ac.name === c.name) ? true : false,
        }));

        commit("setCompanyCards", new_cards);
        resolve();
      });
    },
    saveCompanyCards({ state }) {
      return new Promise((resolve, reject) => {
        const cards = state.settings.company_view.cards;
        Vue.prototype.$axios
          .put("/settings/company_view", cards)
          .then(() => {
            resolve();
          })
          .catch((err) => {
            reject(err);
          });
      });
    },
    resetCompanyCards({ dispatch }) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .get("/settings/company_view/reset")
          .then(() => {
            resolve(dispatch("getCompanyCards", true));
          })
          .catch((err) => {
            reject(err);
          });
      });
    },
  },
  getters: {
    getServerMode: (state) => state.settings.server_mode,
    getCompanyCards: (state) => state.settings.company_view.cards,
  },
};
