import Vue from "vue";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export default {
  namespaced: true,
  state: () => ({
    load_wait: 0,
    prepages: [],
    active_prepages: [],
  }),
  mutations: {
    modifyPrepage(state, prepage) {
      if (!state.prepages.some((p) => (p.id = prepage.id))) {
        state.prepages.push(prepage);
      } else {
        state.prepages[state.prepages.findIndex((p) => p.id == prepage.id)];
      }
    },
    setPrepages(state, prepages) {
      state.prepages = prepages;
    },
    removePrepage(state, id) {
      state.prepages = state.prepages.filter((p) => p.id != id);
    },
    setActive(state, prepages) {
      state.active_prepages = prepages;
    },
    removeAllPrepages(state) {
      state.prepages = [];
    },
  },
  actions: {
    getPrepages({ commit }) {
      if (this.state.prepages.load_wait < Date.now()) {
        this.state.prepages.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
        return new Promise((resolve, reject) => {
          Vue.prototype.$axios
            .get("/prepage")
            .then((resp) => {
              commit("removeAllPrepages");
              const prepages = resp.data;
              if (prepages.length > 0) {
                commit("setPrepages", prepages);
                let filtered_prepages = prepages.filter(
                  (page) => page.active && page.image != null
                );
                filtered_prepages.sort((a, b) => {
                  if (a.order > b.order) {
                    return 1;
                  } else if (a.order < b.order) {
                    return -1;
                  } else {
                    return 0;
                  }
                });
                commit("setActive", filtered_prepages);
              }
              resolve(resp);
            })
            .catch((err) => {
              reject(err);
            });
        });
      }
    },
    modifyPrepage({ commit }, prepage) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .put("/prepage", prepage)
          .then((resp) => {
            commit("modifyPrepage", prepage);
            resolve(resp);
          })
          .catch((err) => {
            reject(err);
          });
      });
    },
    deletePrepage({ commit }, prepage) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .delete("/prepage/" + prepage.id)
          .then((resp) => {
            commit("removePrepage", prepage.id);
            resolve(resp);
          })
          .catch((err) => {
            reject(err);
          });
      });
    },
  },
  getters: {
    get: (state) => {
      return state.prepages;
    },
    getActive: (state) => {
      return state.active_prepages;
    },
  },
};
