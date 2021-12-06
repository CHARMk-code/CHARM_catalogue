import Vue from "vue";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export default {
  namespaced: true,
  state: () => ({
    companies: [],
    load_wait: 0,
  }),
  mutations: {
    modifyCompany(state, company) {
      if (!state.companies.some((c) => (c.id = company.id))) {
        state.companies.push(company);
      } else {
        state.companies[state.companies.findIndex((c) => c.id == company.id)];
      }
    },
    setCompanies(state, companies) {
      state.companies = companies;
    },
    removeCompany(state, id) {
      state.companies = state.companies.filter((c) => c.id != id);
    },
    removeAllCompanies(state) {
      state.companies = [];
    },
  },
  actions: {
    getCompanies({ commit }) {
      return new Promise((resolve, reject) => {
        if (this.state.companies.load_wait < Date.now()) {
          this.state.companies.load_wait =
            Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          Vue.prototype.$axios
            .get("/company")
            .then((resp) => {
              commit("removeAllCompanies");
              const companies = resp.data;
              if (companies.length > 0) {
                commit("setCompanies", companies);
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
    modifyCompany({ commit }, company) {
      return new Promise((resolve, reject) => {
        company = {
          ...company,
          tags: company.looking_for
            .map((o) => o.id)
            .concat(company.offering.map((o) => o.id))
            .concat(company.divisions.map((o) => o.id))
            .concat(company.business_area.map((o) => o.id)),
        };
        Vue.prototype.$axios
          .put("/company", company)
          .then((resp) => {
            commit("modifyCompany", company);
            resolve(resp);
          })
          .catch((err) => {
            reject(err);
          });
      });
    },
    deleteCompany({ commit }, company) {
      return new Promise((resolve, reject) => {
        Vue.prototype.$axios
          .delete("/company/" + company.id)
          .then((resp) => {
            commit("removeCompany", company.id);
            resolve(resp);
          })
          .catch((err) => {
            reject(err);
          });
      });
    },
  },
  getters: {
    companies: (state) => {
      return state.companies;
    },
    companyByName: (state) => (name) => {
      if (state.companies.length > 0) {
        return state.companies.filter((c) => c.name == name);
      }
      return [];
    },
  },
};
