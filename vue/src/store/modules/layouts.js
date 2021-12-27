import Vue from "vue";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export default {
  namespaced: true,
  state: () => ({
    layouts: [],
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
  },
  actions: {
    getLayouts({ commit }) {
      if (this.state.layouts.load_wait < Date.now()) {
        this.state.layouts.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
        return new Promise((resolve, reject) => {
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
        });
      }
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
  },
  getters: {
    get: (state) => {
      return state.layouts;
    },
    getMiddle: (state) => {
      const layouts = state.layouts.filter((t) => t.placement === 0);
      if (layouts.length === 0) {
        console.log("undefined ");
        return undefined;
      }

      const index = Math.floor(Math.random() * layouts.length);
      console.log(layouts, index, layouts[index].image);
      return layouts[index];
    },
    getSide: (state) => (side) => {
      const layout = state.layouts.filter((t) => t.placement === side);
      return layout.length === 0 ? undefined : layout[0];
    },
  },
};
