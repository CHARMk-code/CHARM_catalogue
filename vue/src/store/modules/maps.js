import Vue from "vue";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export default {
  namespaced: true,
  state: () => ({
    maps: [],
    load_wait: 0,
  }),
  mutations: {
    modifyMap(state, map) {
      if (!state.maps.some((p) => (p.id = map.id))) {
        state.maps.push(map);
      } else {
        state.maps[state.maps.findIndex((p) => p.id == map.id)];
      }
    },
    setMaps(state, maps) {
      state.maps = maps;
    },
    removeMap(state, id) {
      state.maps = state.maps.filter((p) => p.id != id);
    },

    removeAllMaps(state) {
      state.maps = [];
    },
  },
  actions: {
    getMaps({ commit }) {
      if (this.state.maps.load_wait < Date.now()) {
        this.state.maps.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
        return new Promise((resolve, reject) => {
          Vue.prototype.$axios
            .get("/map")
            .then((resp) => {
              commit("removeAllMaps");
              const maps = resp.data;
              if (maps.length > 0) {
                commit("setMaps", maps);
              }
              resolve(resp);
            })
            .catch((err) => {
              reject(err);
            });
        });
      }
    },
    modifyMap({ commit }, map) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .put("/map", map)
          .then((resp) => {
            commit("modifyMap", map);
            resolve(resp);
          })
          .catch((err) => {
            reject(err);
          });
      });
    },
    deleteMap({ commit }, map) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .delete("/map/" + map.id)
          .then((resp) => {
            commit("removeMap", map.id);
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
      return state.maps;
    },
  },
};
