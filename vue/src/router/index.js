import Vue from "vue";
import Router from "vue-router";
import Company_view from "@/views/company";
//import Search from '@/components/Search'
import Home from "@/components/Home";
import Administration from "@/views/Administration";
import Companies from "@/views/admin/Companies";
import Tags from "@/views/admin/Tags";
import Login from "@/views/login";
import Upload from "@/components/Upload";
import Prepage_view from "@/views/admin/Prepage";

Vue.use(Router);

const router = new Router({
  mode: "history",
  routes: [
    {
      path: "/",
      name: "Home",
      component: Home,
      meta: {
        noAuth: true,
      },
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
          name: "Admin/Prepage",
          component: Prepage_view,
        },
        {
          path: "companies",
          name: "Admin/Companies",
          component: Companies,
        },
        {
          path: "tags",
          name: "Admin/Tags",
          component: Tags,
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
      meta: {
        noAuth: true,
      },
    },
    {
      path: "/login",
      name: "Login",
      component: Login,
      meta: {
        noAuth: true,
      },
      beforeEnter: (to, from, next) => {
        if (router.app.$store.getters["auth/isLoggedIn"]) {
          next("/");
        } else {
          next();
        }
      },
    },
  ],
});

router.beforeEach((to, from, next) => {
  if (to.matched.some((record) => !record.meta.noAuth)) {
    if (router.app.$store.getters["auth/isLoggedIn"]) {
      next();
      return;
    }
    next("/login");
  } else {
    next();
  }
});

export default router;
