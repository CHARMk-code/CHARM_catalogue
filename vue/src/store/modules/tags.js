import Vue from "vue";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export default {
  namespaced: true,
  state: () => ({
    tags: [],
    business_areas: [],
    divisions: [],
    looking_fors: [],
    offers: [],
    load_wait: 0,
  }),
  mutations: {
    modifyTag(state, tag) {
      // As division and/or business_area could be modify so we can't use removeTag
      // and it easiest to just filter out all arrays and pushing the new tag
      state.business_areas = state.business_areas.filter((t) => t.id != tag.id);
      state.divisions = state.divisions.filter((t) => t.id != tag.id);
      state.looking_fors = state.looking_fors.filter((t) => t.id != tag.id);
      state.tags = state.tags.filter((t) => t.id != tag.id);

      if (tag.business_area) {
        state.business_areas.push(tag);
      } else if (tag.division) {
        state.divisions.push(tag);
      } else if (tag.looking_for) {
        state.looking_fors.push(tag);
      } else if (tag.offering) {
        state.offers.push(tag);
      } else {
        state.tags.push(tag);
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
    setLookingFors(state, tags) {
      state.looking_fors = tags;
    },
    setOffers(state, tags) {
      state.offers = tags;
    },
    removeTag(state, tag) {
      // No point wasting cpu time filtering arrays were the tag won't be
      if (tag.business_area) {
        state.business_areas = state.business_areas.filter(
          (t) => t.id != tag.id
        );
      } else if (tag.division) {
        state.divisions = state.divisions.filter((t) => t.id != tag.id);
      } else if (tag.looking_for) {
        state.looking_fors = state.looking_fors.filter((t) => t.id != tag.id);
      } else if (tag.offers) {
        state.offers = state.offers.filter((t) => t.id != tag.id);
      } else {
        state.tags = state.tags.filter((t) => t.id != tag.id);
      }
    },
    removeAllTags(state) {
      state.tags = [];
      state.business_areas = [];
      state.divisions = [];
      state.looking_fors = [];
      state.offers = [];
    },
  },
  actions: {
    getTags({ commit }) {
      return new Promise((resolve, reject) => {
        if (this.state.tags.load_wait < Date.now()) {
          this.state.tags.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          Vue.prototype.$axios
            .get("/tag")
            .then((resp) => {
              commit("removeAllTags");
              const tags = resp.data;
              if (tags.length > 0) {
                commit(
                  "setTags",
                  tags.filter(
                    (t) =>
                      !(
                        t.business_area ||
                        t.division ||
                        t.looking_for ||
                        t.offering
                      )
                  )
                );
                commit(
                  "setBusinessAreas",
                  tags.filter((t) => t.business_area)
                );
                commit(
                  "setDivisions",
                  tags.filter((t) => t.division)
                );
                commit(
                  "setLookingFors",
                  tags.filter((t) => t.looking_for)
                );
                commit(
                  "setOffers",
                  tags.filter((t) => t.offering)
                );
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
    deleteTag({ commit }, tags) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .delete("/tag/" + tags.id)
          .then((resp) => {
            commit("removeTag", tags.id);
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
        const result = state.all.filter((t) => t.id == id);
        if (result.length > 0) {
          return result[0];
        }
      }
      return [];
    },
    getTagsFromIds: (state) => (ids) => {
      const result = state.tags.filter((t) => ids.indexOf(t.id) != -1);
      return result;
    },
    getDivisionsFromIds: (state) => (ids) => {
      const result = state.divisions.filter((t) => ids.indexOf(t.id) != -1);
      return result;
    },
    getBusinessAreasFromIds: (state) => (ids) => {
      const result = state.business_areas.filter(
        (t) => ids.indexOf(t.id) != -1
      );
      return result;
    },
    getLookingForFromIds: (state) => (ids) => {
      const result = state.looking_fors.filter((t) => ids.indexOf(t.id) != -1);
      return result;
    },
    getOffersFromIds: (state) => (ids) => {
      const result = state.offers.filter((t) => ids.indexOf(t.id) != -1);
      return result;
    },

    tags: (state) => {
      return state.tags;
    },
    divisions: (state) => {
      return state.divisions;
    },
    business_areas: (state) => {
      return state.business_areas;
    },
    looking_fors: (state) => {
      return state.looking_fors;
    },
    offers: (state) => {
      return state.offers;
    },

    all: (state) => {
      return state.tags.concat(state.divisions, state.business_areas);
    },
  },
};
