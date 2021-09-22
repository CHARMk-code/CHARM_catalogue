import Vue from 'vue'
import Router from 'vue-router'
import Company from '@/components/Company'
import Search from '@/components/Search'

Vue.use(Router)

export default new Router({
  mode: 'history',
  routes: [
    {
      path: '/search',
      name: 'Search',
      component: Search
    },
    {
      path: '/',
      redirect: '/1'
    },
    {
      path: '/:page',
      name: 'Page',
      component: Company
    }
  ]
})
