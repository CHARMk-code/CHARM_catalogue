import Vue from "vue";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export default {
  namespaced: true,
  state: () => ({
    layouts: [],
    current_center: "",
    load_wait: 0,
  }),
  mutations: {
    modifyLayout(state, layout) {
      if (!state.layouts.some((p) => (p.id = layout.id))) {
        state.layouts.push(layout);
      } else {
        state.layouts[state.layouts.findIndex((p) => p.id == layout.id)];
      }
    },
    setLayouts(state, layouts) {
      state.layouts = layouts;
    },
    removeLayout(state, id) {
      state.layouts = state.layouts.filter((p) => p.id != id);
    },

    removeAllLayouts(state) {
      state.layouts = [];
    },
    updateCenter(state){
    
      const layouts = state.layouts.filter((t) => t.placement === 0);
      if (layouts.length !== 0) {
        const index = Math.floor(Math.random() * layouts.length);
        state.current_center = layouts[index];
      }
    }
  },
  actions: {
    getLayouts({ commit }) {
      return new Promise((resolve, reject) => {
        if (this.state.layouts.load_wait < Date.now()) {
          this.state.layouts.load_wait =
            Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          Vue.prototype.$axios
            .get("/layout")
            .then((resp) => {
              commit("removeAllLayouts");
              const layouts = resp.data;
              if (layouts.length > 0) {
                commit("setLayouts", layouts);
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
    modifyLayout({ commit }, layout) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .put("/layout", layout)
          .then((resp) => {
            commit("modifyLayout", layout);
            resolve(resp);
          })
          .catch((err) => {
            reject(err);
          });
      });
    },
    deleteLayout({ commit }, layout) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .delete("/layout/" + layout.id)
          .then((resp) => {
            commit("removeLayout", layout.id);
            resolve(resp);
          })
          .catch((err) => {
            reject(err);
          });
      });
    },
    deleteAllLayout({commit}){
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .delete("/layout")
          .then((resp) => {
            commit("removeAllLayouts");
            resolve(resp);
          })
          .catch((err) => {
            reject(err);
          });
      });
    }
  },
  getters: {
    get: (state) => {
      return state.layouts;
    },
    getMiddle: (state) => {
      return state.current_center;
    },

    getSide: (state) => (side) => {
      const layout = state.layouts.filter((t) => t.placement === side);
      return layout.length === 0 ? undefined : layout[0];
    },
  },
};
