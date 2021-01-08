import Vue from 'vue'
import Router from 'vue-router'
import Company from '@/components/Company'
import Search from '@/components/Search'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/company/:company',
      name: 'Company_Page',
      component: Company
    },
    {
      path: '/search',
      name: 'Search',
      component: Search
    }
  ]
})
