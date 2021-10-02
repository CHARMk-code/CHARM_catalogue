import Vue from 'vue'
import Router from 'vue-router'
import Company from '@/components/Company'
//import Search from '@/components/Search'
import Home from '@/components/Home'
import Administration from '@/views/Administration'
import Companies from '@/views/admin/Companies'
import Login from "@/components/Login";

Vue.use(Router)

export default new Router({
  mode: 'history',
  routes: [
    {
      path: '/',
      name: 'Home',
      component: Home
    },
//    {
//      path: '/Search',
//      name: 'Search',
//      component: Search
//    },
    {
      path: '/Admin',
      name: 'Admin',
      component: Administration,
      children: [
        {
          path: 'prepages'
        },
        {
          path: 'companies',
          name: 'Admin/Companies',
          component: Companies

        },
        {
          path: 'maps'

        },
        {
          path: 'layout'

        },
        {
          path: 'batch'

        },
        {
          path: 'account'

        }



      ]
    }, 
    {
      path: '/company/:company',
      name: 'Company',
      component: Company
    }, 
    {
      path: "/login",
      name: "Login",
      component: Login,
    },
  ]
})

