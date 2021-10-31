import Vue from "vue";

export default {
  namespaced: true,
  state: () => ({
    tags: [],
    business_areas: [],
    divisions: [],
  }),
  mutations: {
    modifyTag(state, tag) {
      if (!state.tags.some((t) => (t.id = tag.id))) {
        state.tags.push(tag);
      } else {
        state.tags[state.tags.findIndex((t) => t.id == tag.id)];
      }
    },
    setTags(state, tags) {
      state.tags = tags;
    },
    removeTag(state, id) {
      state.tags.splice(state.tags.findIndex((t) => t.id == id));
    },
    removeAllTags(state) {
      state.tags = [];
    },
  },
  actions: {
    getTags({ commit }) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .get("/tag")
          .then((resp) => {
            commit("removeAllTags");
            const tags = resp.data;
            if (tags.length > 0) {
              commit("setTags", tags);
            }
            resolve(resp);
          })
          .catch((err) => {
            reject(err);
          });
      });
    },
    modifyTags({ commit }, tags) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .put("/tags", tags)
          .then((resp) => {
            commit("modifyCompany", tags);
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
