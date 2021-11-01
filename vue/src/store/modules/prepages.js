import Vue from "vue";

export default {
  namespaced: true,
  state: () => ({
    prepages: [],
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
      state.prepages.filter((p) => p.id != id);
    },
    removeAllPrepages(state) {
      state.prepages = [];
    },
  },
  actions: {
    getPrepages({ commit }) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .get("/prepage")
          .then((resp) => {
            commit("removeAllPrepages");
            const prepages = resp.data;
            if (prepages.length > 0) {
              commit("setPrepages", prepages);
            }
            resolve(resp);
          })
          .catch((err) => {
            reject(err);
          });
      });
    },
    modifyPrepage({ commit }, prepage) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .put("/prepages", prepage)
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
          .put("/prepages/" + prepage.id)
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
  getters: {},
};
