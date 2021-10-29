import Vue from "vue";

export default {
  namespaced: true,
  state: () => ({
    companies: {}
  }),
  mutations: {
    addCompany(state, { company }){
      state.companies[company.id] = company
    },
    setCompanies(state, { companies }){
      companies.forEach((c) => state.companies[c.id] = c )
    },
    removeCompany(state, { id }){
      delete state.companies[id] 
    },
    removeAllCompanies(state){
      state.companies = {};
    }
  },
  actions: {
    getCompanies({ commit }) {
      return new Promise((resolve,reject) => {
        Vue.prototype.$axios('get', {url: "company/get"})
        .then(resp => {
          const companies = resp.data;
          if (companies.length != 0) {
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
  }
}
