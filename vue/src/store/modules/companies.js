import Vue from "vue";

export default {
  namespaced: true,
  state: () => ({
    companies: {}
  }),
  mutations: {
    addCompany(state, company){
      if (!state.companies.some((c) => c.id = company.id)) {
        state.companies.push(company)
      }
    },
    setCompanies(state, companies){
      state.companies = companies
    },
    removeCompany(state, id){
      state.companies.splic(state.companies.findIndex((c) => c.id == id))
    },
    removeAllCompanies(state){
      state.companies = [];
    }
  },
  actions: {
    getCompanies({ commit }) {
      return new Promise((resolve,reject) => {
        Vue.prototype.$axios('get', {url: "company/get"})
        .then(resp => {
          const companies = resp.data;
          if (companies.length > 0) {
            commit("setCompanies", companies)
          }
          resolve(resp)
        })
        .catch(err => {
          reject(err)
        })
      })
    },
    updateCompanies({ commit }) {
      return new Promise((resolve,reject) => {
        Vue.prototype.$axios('get', {url: "company/get"})
        .then(resp => {
          const companies = resp.data;
          commit("removeAllCompanies")
          if (companies.length != 0) {
            commit("setCompanies", companies)
          }
          resolve(resp)
        })
        .catch(err => {
          reject(err)
        })
      })
    }
  },
  getters: {
    companies: (state) => { return state.companies }
  }
}
