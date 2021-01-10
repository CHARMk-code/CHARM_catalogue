// The Vue build version to load with the `import` command
// (runtime-only or standalone) has been set in webpack.base.conf with an alias.
import Vue from 'vue'
import vuetify from '@/plugins/vuetify'
import Vuex from 'vuex'
import searchPlugin from 'vuex-search'
import App from './App'
import router from './router'
import {HTTP} from '@/plugins/http-common.js'

Vue.config.productionTip = false
Vue.use(Vuex)

const store = new Vuex.Store({
  state: {
    companies: []
  },
  mutations: {
    addCompany: (state, company) => {
      state.companies.push(company)
    }
  },
  actions: {
    retrieveCompanies ({commit}) {
      HTTP.get('company/get')
        .then(resp => {
          resp.data.forEach(company => commit('addCompany', company))
        })
    }
  },
  getters: {
    getCompanyById: state => id => {
      return state.companies.find(company => company.id === parseFloat(id))
    }
  },
  plugins: [
    searchPlugin({
      resources: {
        companies: {
          index: ['name'],
          getter: state => state.companies,
          watch: {delay: 500}
        }
      }
    })
  ]
})

/* eslint-disable no-new */
new Vue({
  el: '#app',
  router,
  vuetify,
  store,
  components: { App },
  template: '<App/>'
})
