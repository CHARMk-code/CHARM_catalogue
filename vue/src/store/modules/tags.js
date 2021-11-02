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
    setBusinessAreas(state, tags) {
      state.business_areas = tags;
    },
    setDivisions(state, tags) {
      state.divisions = tags;
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
              // if tags such only contains tag not prefencent in business_areas or divisions
              // commit("setTags", tags.filter((t) => !(t.business_area || t.division));
              commit(
                "setBusinessAreas",
                tags.filter((t) => t.business_area)
              );
              commit(
                "setDivisions",
                tags.filter((t) => t.division)
              );
            }
            resolve(resp);
          })
          .catch((err) => {
            reject(err);
          });
      });
    },
    modifyTag({ commit }, tags) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .put("/tag", tags)
          .then((resp) => {
            commit("modifyTag", tags);
            resolve(resp);
          })
          .catch((err) => {
            reject(err);
          });
      });
    },
  },
  getters: {
    getTagFromId: (state) => (id) => {
      if (state.tags.length > 0) {
        const result = state.tags.filter((t) => t.id == id);
        if (result.length > 0) {
          return result[0];
        }
      }
      return [];
    },
    tags: (state) => {
      return state.tags;
    },
    division: (state) => {
      return state.tags;
    },
    business_area: (state) => {
      return state.tags;
    },
  },
};
