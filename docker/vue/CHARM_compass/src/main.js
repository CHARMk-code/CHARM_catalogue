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
    companies: [],
    tags: [],
    companyTags: {},
    tagCompanies: {},
    filteredCompanies: {}
  },
  mutations: {
    addCompany: (state, company) => {
      state.companies.push(company)
    },
    setCompanies: (state, companies) => {
      state.companies = companies
    },
    addTag: (state, tag) => {
      state.tags.push(tag)
    },
    setTags: (state, tags) => {
      state.tags = tags
    },
    defaultFilteredCompanies: (state) => {
      state.filteredCompanies = state.companies
    },
    setFilteredCompanies: (state, companies) => {
      state.filteredCompanies = companies
    },
    addTagstoCompany: (state, {companyId, tags}) => {
      state.companyTags[companyId] = tags
    },
    addCompanytoTag: (state, {companyId, tagId}) => {
      if (typeof state.tagCompanies[tagId] === 'undefined') {
        state.tagCompanies[tagId] = [companyId]
      } else {
        state.tagCompanies[tagId].push(companyId)
      }
    }
  },
  actions: {
    retrieveCompanies ({commit, dispatch}) {
      return new Promise((resolve, reject) => {
        HTTP.get('company/get')
          .then(resp => {
            commit('setCompanies', resp.data)
            commit('defaultFilteredCompanies')

            resp.data.forEach((company) => { dispatch('retrieveTagsForCompany', company.id) })
          })
        resolve()
      })
    },
    retrieveTags ({commit}) {
      return new Promise((resolve, reject) => {
        HTTP.get('tag/get')
          .then(resp => {
            commit('setTags', resp.data)
          })
        resolve()
      })
    },
    retrieveTagsForCompany: ({commit}, companyId) => {
      return new Promise((resolve, reject) => {
        HTTP.get('tag/get?company_filter=' + companyId)
          .then(resp => {
            commit('addTagstoCompany', {companyId, tags: resp.data})
            resp.data.forEach(value => {
              commit('addCompanytoTag', {companyId, tagId: value.id})
            })
          })
        resolve()
      })
    }
  },
  getters: {
    getCompanyById: state => id => {
      return state.companies.find(company => company.id === parseFloat(id))
    },
    getTagById: state => id => {
      return state.tags.find(tag => tag.id === parseFloat(id))
    },
    getTagsByCompanyId: state => id => {
      return state.companyTags[id]
    },
    getCompaniesWithTagId: state => id => {
      return state.tagCompanies[id]
    }
  },
  plugins: [
    searchPlugin({
      resources: {
        companies: {
          index: ['name'],
          getter: state => state.filteredCompanies,
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
