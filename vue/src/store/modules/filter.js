export default {
  namespaced: true,
  state: () => ({
    filters: {
      query: "",
      tags: {
        divisions: [],
        looking_for: [],
        business_areas: [],
        offerings: [],
      },
      charmtalk: false,
    },
    filteredCompanies: [],
  }),
  mutations: {
    setFilters(state, filters) {
      state.filters = filters;
    },
    setFilteredCompanies(state, companies) {
      state.filteredCompanies = companies;
    },
  },
  actions: {
    filterCompanies({ commit, state, rootGetters }) {
      var filteredCompanies = rootGetters["companies/companies"];
      if (state.filters != {}) {
        if (state.filters.query != "") {
          filteredCompanies = filteredCompanies.filter((c) =>
            c.name.toLowerCase().includes(state.filters.query.toLowerCase())
          );
        }
        for (const key in state.filters.tags) {
          const filterTags = state.filters.tags[key];

          if (filterTags.length > 0) {
            filteredCompanies = filteredCompanies.filter((c) => {
              return c.tags.some((t) =>
                filterTags.some((filterTag) => t == filterTag.id)
              );
            });
          }
        }
        if (state.filters.charmtalk) {
          filteredCompanies = filteredCompanies.filter((t) => t.charmtalk);
        }
        filteredCompanies = filteredCompanies.filter((t) => t.active);
      }
      commit("setFilteredCompanies", filteredCompanies);
    },
    setFilters({ commit }, filters) {
      commit("setFilters", filters);
      //      dispatch("filterCompanies");
    },
  },
  getters: {
    filteredCompanies: (state) => {
      return state.filteredCompanies;
    },
  },
};
