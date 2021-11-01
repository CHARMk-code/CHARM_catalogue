import Vue from "vue";

export default {
  namespaced: true,
  state: () => ({
    prepages: [],
  }),
  mutations: {
    modifyPrepages(state, prepage) {
      if (!state.prepages.some((p) => (p.id = prepage.id))) {
        state.prepages.push(prepage);
      } else {
        state.prepages[state.prepages.findIndex((p) => p.id == prepage.id)];
      }
    },
    setPrepages(state, prepages) {
      state.prepages = prepages;
    },
    removePrepages(state, id) {
      state.prepages.splice(state.prepages.findIndex((p) => p.id == id));
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
    modifyPrepages({ commit }, prepages) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .put("/prepages", prepages)
          .then((resp) => {
            commit("modifyCompany", prepages);
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
