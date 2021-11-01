import Vue from "vue";

export default {
  namespaced: true,
  state: () => ({
    companies: [],
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
      console.log(state, id);
      state.companies = state.companies.filter((c) => c.id != id);
    },
    removeAllCompanies(state) {
      state.companies = [];
    },
  },
  actions: {
    getCompanies({ commit }) {
      return new Promise((resolve, reject) => {
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
      });
    },
    modifyCompany({ commit }, company) {
      return new Promise((resolve, reject) => {
        let copy_company = {}; // FIXME: Workaround for tags be undefined, which breaks backend
        for (let key in company) {
          copy_company[key] = company[key];
        }
        delete copy_company.tags;
        Vue.prototype.$axios
          .put("/company", copy_company)
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
