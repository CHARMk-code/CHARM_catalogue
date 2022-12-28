import { createRouter, createWebHistory } from "vue-router";

import { useAuthStore } from "@/stores/modules/auth";
import { useMapsStore } from "@/stores/modules/maps";
import { useTagsStore } from "@/stores/modules/tags";
import { useCompaniesStore } from "@/stores/modules/companies";
import { usePrePagesStore } from "@/stores/modules/prepages";
import { useLayoutsStore } from "@/stores/modules/layouts";
import { useShortcutsStore } from "@/stores/modules/shortcuts";
import { useSite_settingsStore } from "@/stores/modules/site_settings";
import { useFilterStore } from "@/stores/modules/filter";
import { useFavoritesStore } from "@/stores/modules/favorites";
import { mapStores } from "pinia";

const Company_view = () => import("@/views/company.vue");
const Search_view = () => import("@/views/search.vue");
const Login_view = () => import("@/views/login.vue");
const Landing_view = () => import("@/views/Landing.vue");
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

const router = createRouter({
  history: createWebHistory(),
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
    useAuthStore().setAuthorizationHeader()
    useFavoritesStore().loadFromStorage();

    const mapsStore = useMapsStore();

    await Promise.all([
      mapsStore.getMaps(),
      useTagsStore().getTags(), // This one fails if db is empty, check why
      useCompaniesStore().fetchCompanies(),
      usePrePagesStore().getPrepages(),
      useLayoutsStore().getLayouts(),
      useShortcutsStore().getShortcuts(),
      useSite_settingsStore().getCompanyCards(),
    ])
      .then(() => {
        useFilterStore().filterCompanies()
      })
      .catch(() => { }); // add some error here in the future?
  }
  if (to.matched.some((record) => !record.meta.noAuth)) {
    const authStore = useAuthStore();

    if (authStore.isLoggedIn) {
      next();
    } else {
      next({ name: "Login", params: { nextUrl: to.fullPath } });
    }
  } else {
    next();
  }
});

export default router;
