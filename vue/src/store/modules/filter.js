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
        languages: [],
        areas:[]
      },
      favorites: false,
      charmtalk: false,
      sweden: false,
    },
    filteredCompanies: [],
  }),
  mutations: {
    setFilters(state, filters) {
      state.filters = {
        ...state.filters,
        ...filters,
        tags: { ...state.filters.tags, ...filters.tags },
      };
    },
    setFilteredCompanies(state, companies) {
      state.filteredCompanies = companies;
    },
  },
  actions: {
    filterCompanies({ commit, state, rootGetters }) {
      return new Promise((resolve) => {
        var filteredCompanies = rootGetters["companies/companies"];

        //remove all non active companies
        filteredCompanies = filteredCompanies.filter((t) => t.active);

        if (state.filters != {}) {
          if (state.filters.query != "") {
            filteredCompanies = filteredCompanies.filter((c) =>
              c.name.toLowerCase().includes(state.filters.query.toLowerCase())
            );
          }
          for (const key in state.filters.tags) {
            const filterTags = state.filters.tags[key];
            if (key == "areas") {
              if (filterTags.length > 0) {
                filteredCompanies = filteredCompanies.filter((c) => {
                  return filterTags.some((filterTag) => c.map_image == filterTag.name
                  );
                });
              }
            } else {
              if (filterTags.length > 0) {
                filteredCompanies = filteredCompanies.filter((c) => {
                  return c.tags.some((t) =>
                    filterTags.some((filterTag) => t == filterTag.id)
                  );
                });
              }
            }
            
          }
          if (state.filters.charmtalk) {
            filteredCompanies = filteredCompanies.filter((t) => t.charmtalk);
          }
          if (state.filters.favorites) {
            filteredCompanies = filteredCompanies.filter((t) =>
              rootGetters["favorites/favorites"].has(t.id)
            );
          }
          if (state.filters.sweden) {
            filteredCompanies = filteredCompanies.filter((t) => t.in_sweden);
          }
        }
        commit("setFilteredCompanies", filteredCompanies);
        resolve();
      });
    },
    setFilters({ commit }, filters) {
      commit("setFilters", filters);
    },
  },
  getters: {
    filteredCompanies: (state) => {
      return state.filteredCompanies;
    },
    getFilter: (state) => {
      return state.filters;
    },
    getUrlQuery: (state) => {
      let query = {};
      state.filters.query.length > 0 && (query.q = state.filters.query);
      if (state.filters.tags.divisions.length > 0) {
        query.divisions = state.filters.tags.divisions
          .map((t) => t.id.toString())
          .toString();
      }
      if (state.filters.tags.looking_for.length > 0) {
        query.looking_for = state.filters.tags.looking_for
          .map((t) => t.id.toString())
          .toString();
      }
      if (state.filters.tags.business_areas.length > 0) {
        query.business_areas = state.filters.tags.business_areas
          .map((t) => t.id.toString())
          .toString();
      }
      if (state.filters.tags.offerings.length > 0) {
        query.offerings = state.filters.tags.offerings
          .map((t) => t.id.toString())
          .toString();
      }
      if (state.filters.tags.languages.length > 0) {
        query.languages = state.filters.tags.languages
          .map((t) => t.id.toString())
          .toString();
      }
      state.filters.favorites && (query.favorites = true);
      state.filters.charmtalk && (query.charmtalk = true);
      state.filters.sweden && (query.sweden = true);

      return query;
    }
  },
};
