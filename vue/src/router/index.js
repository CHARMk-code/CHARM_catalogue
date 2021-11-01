import Vue from "vue";
import Router from "vue-router";
import Company_view from "@/views/company";
//import Search from '@/components/Search'
import Home from "@/components/Home";
import Administration from "@/views/Administration";
import Companies from "@/views/admin/Companies";
import Login from "@/views/login";
import Upload from "@/components/Upload";
import Prepage from "@/components/Prepage";
Vue.use(Router);

export default new Router({
  mode: "history",
  routes: [
    {
      path: "/",
      name: "Home",
      component: Home,
    },
    //    {
    //      path: '/Search',
    //      name: 'Search',
    //      component: Search
    //    },
    {
      path: "/Admin",
      name: "Admin",
      component: Administration,
      children: [
        {
          path: "prepages",
        },
        {
          path: "companies",
          name: "Admin/Companies",
          component: Companies,
        },
        {
          path: "maps",
        },
        {
          path: "layout",
        },
        {
          path: "batch",
          name: "batch",
          component: Upload,
        },
        {
          path: "account",
        },
      ],
    },
    {
      path: "/company/:name",
      name: "Company",
      component: Company_view,
    },
    {
      path: "/prepages/:page",
      name: "Prepage",
      component: Prepage,
    },
    {
      path: "/login",
      name: "Login",
      component: Login,
    },
  ],
});
