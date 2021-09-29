import Vue from 'vue'
import Router from 'vue-router'
import Company from '@/components/Company'
import Search from '@/components/Search'
import Home from '@/components/Home'
import Login from '@/components/Login'

Vue.use(Router)

export default new Router({
  mode: 'history',
  routes: [
    {
      path: '/',
      name: 'Home',
      component: Home
    },
    {
      path: '/Search',
      name: 'Search',
      component: Search
    },
    {
      path: '/company/:company',
      name: 'Company',
      component: Company
    },
    {
      path: '/login',
      name: 'Login',
      component: Login
    }
  ]
})
