import Vue from "vue";
import Router from "vue-router";
const Company_view = () => import("@/views/company.vue");
const Search_view = () => import("@/views/search.vue");
const Login_view = () => import("@/views/login.vue");
const Landing_view = () => import("@/views/Landing.vue");
const CookieInfo_view = () => import("@/views/CookieInfo.vue");
const Map_view = () => import("@/views/Map.vue");
const Prepage_view = () => import("@/views/Prepage.vue");

const Admin_view = () => import("@/views/Administration.vue");
const Companies_admin = () => import("@/views/admin/Companies.vue");
const Prepage_admin = () => import("@/views/admin/Prepage.vue");
const Tags_admin = () => import("@/views/admin/Tags.vue");
const Account_admin = () => import("@/components/admin/Account.vue");
const Map_admin = () => import("@/views/admin/Map.vue");
const Layout_admin = () => import("@/views/admin/Layout.vue");
const Shortcuts_admin = () => import("@/views/admin/Shortcuts.vue");
const Upload_admin = () => import("@/components/admin/Upload.vue");

Vue.use(Router);

const router = new Router({
  mode: "history",
  routes: [
    {
      path: "/",
      name: "Landing",
      component: Landing_view,
      meta: {
        noAuth: true,
      },
    },
    {
      path: "/cookies",
      name: "Cookies",
      component: CookieInfo_view,
      meta: {
        noAuth: true,
      },
    },
    {
      path: "/Admin",
      name: "Admin",
      component: Admin_view,
      children: [
        {
          path: "prepages",
          name: "Admin/Prepage",
          component: Prepage_admin,
        },
        {
          path: "companies",
          name: "Admin/Companies",
          component: Companies_admin,
        },
        {
          path: "tags",
          name: "Admin/Tags",
          component: Tags_admin,
        },
        {
          path: "maps",
          name: "Admin/Maps",
          component: Map_admin,
        },
        {
          path: "shortcuts",
          name: "Admin/Shortcuts",
          component: Shortcuts_admin,
        },
        {
          path: "layout",
          name: "Admin/Layout",
          component: Layout_admin,
        },
        {
          path: "batch",
          name: "batch",
          component: Upload_admin,
        },
        {
          path: "account",
          name: "Account",
          component: Account_admin,
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
      path: "/search",
      name: "Search",
      component: Search_view,
      meta: {
        noAuth: true,
      },
    },
    {
      path: "/maps/:page",
      name: "Map",
      component: Map_view,
      meta: {
        noAuth: true,
      },
    },
    {
      path: "/prepages/:page",
      name: "Prepage",
      component: Prepage_view,
      meta: {
        noAuth: true,
      },
    },
    {
      path: "/login",
      name: "Login",
      component: Login_view,
      meta: {
        noAuth: true,
      },
    },
  ],
});

router.beforeEach(async (to, from, next) => {
  if (from.name == null) {
    // Arriving from offsite, need to load data
    router.app.$axios.defaults.headers.common["Authorization"] =
      "Basic " + router.app.$store.getters["auth/token"];
    router.app.$store.commit("favorites/loadForStorage");

    await Promise.all([
      router.app.$store.dispatch("maps/getMaps"),
      router.app.$store.dispatch("tags/getTags"),
      router.app.$store.dispatch("companies/getCompanies"),
      router.app.$store.dispatch("prepages/getPrepages"),
      router.app.$store.dispatch("layouts/getLayouts"),
      router.app.$store.dispatch("shortcuts/getShortcuts"),
      router.app.$store.dispatch("site_settings/getCompanyCards"),
    ]).then(() => {
      router.app.$store.dispatch("filter/filterCompanies");
    });
  }
  if (to.matched.some((record) => !record.meta.noAuth)) {
    if (router.app.$store.getters["auth/isLoggedIn"]) {
      next();
    } else {
      next({ name: "Login", params: { nextUrl: to.fullPath } });
    }
  } else {
    next();
  }
});

export default router;
