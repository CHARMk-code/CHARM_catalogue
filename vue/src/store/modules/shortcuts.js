import Vue from "vue";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export default {
  namespaced: true,
  state: () => ({
    shortcuts: [],
    load_wait: 0,
  }),
  mutations: {
    modifyShortcut(state, shortcut) {
      if (!state.shortcuts.some((p) => p.id === shortcut.id)) {
        //when copy pasteing I added so it was === instead of =
        state.shortcuts.push(shortcut);
      } else {
        state.shortcuts[state.shortcuts.findIndex((p) => p.id === shortcut.id)];
      }
    },
    setShortcuts(state, shortcuts) {
      state.shortcuts = shortcuts;
    },
    removeShortcut(state, id) {
      state.shortcuts = state.shortcuts.filter((p) => p.id != id);
    },
    removeAllShortcuts(state) {
      state.shortcuts = [];
    },
  },
  actions: {
    getShortcuts({ commit }) {
      return new Promise((resolve, reject) => {
        if (this.state.shortcuts.load_wait < Date.now()) {
          this.state.shortcuts.load_wait =
            Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          Vue.prototype.$axios
            .get("/shortcut")
            .then((resp) => {
              commit("removeAllShortcuts");
              const shortcuts = resp.data;
              if (shortcuts.length > 0) {
                commit("setShortcuts", shortcuts);
              }
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
    modifyShortcut({ commit }, shortcut) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .put("/shortcut", shortcut)
          .then((resp) => {
            commit("modifyShortcut", shortcut);
            resolve(resp);
          })
          .catch((err) => {
            reject(err);
          });
      });
    },
    deleteMap({ commit }, shortcut) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .delete("/shortcut/" + shortcut.id)
          .then((resp) => {
            commit("removeShortcut", shortcut.id);
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
      return state.shortcuts;
    },
  },
};
