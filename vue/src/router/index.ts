import { createRouter, createWebHistory } from "vue-router";

import { useAuthStore } from "@/stores/modules/auth";
// import { useMapsStore } from "@/stores/modules/maps";
import { useTagsStore } from "@/stores/modules/tags";
import { useCompaniesStore } from "@/stores/modules/companies";
import { usePrepagesStore } from "@/stores/modules/prepages";
import { useLayoutsStore } from "@/stores/modules/layouts";
import { useShortcutsStore } from "@/stores/modules/shortcuts";
import { useSite_settingsStore } from "@/stores/modules/site_settings";
import { useFilterStore } from "@/stores/modules/filter";
import { useFavoritesStore } from "@/stores/modules/favorites";
import { useFeedbackStore } from "@/stores/modules/feedback";
import { useFairMapsStore } from "@/stores/modules/fairMaps";
import { useTagCategoriesStore } from "@/stores/modules/tag_category";

const UserLayout = () => import("@/views/userLayout.vue");
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
const Map_admin = () => import("@/views/admin/AdminMap.vue");
const Layout_admin = () => import("@/views/admin/Layout.vue");
const Shortcuts_admin = () => import("@/views/admin/Shortcuts.vue");
const Upload_admin = () => import("@/components/admin/Upload.vue");
const Feedback_admin = () => import("@/views/admin/Feedback.vue");
const Site_admin = () => import("@/views/admin/Site_settings.vue");

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "Home",
      component: UserLayout,
      meta: {
        noAuth: true,
      },
      children: [
        {
          path: "/",
          name: "Home",
          component: Landing_view,
          meta: {
            noAuth: true,
          },
        },
        {
          path: "/company/:name",
          name: "Company",
          component: Company_view,
          meta: {
            noAuth: true,
            navigation: true,
          },
        },
        {
          path: "/search",
          name: "Search",
          component: Search_view,
          meta: {
            noAuth: true,
            generated: false,
          },
          beforeEnter: (to) => {
            const filterStore = useFilterStore();
            if (!to.meta.generated || Object.keys(to.query).length > 0) {
              filterStore.setFiltersFromRouteQuery(to.query);
              to.meta.generated = true;
              return;
            }
            if (to.meta.generated) {
              return;
            }

            to.query = filterStore.generateSearchRouteQuery();
            to.meta.generated = true;
            return to;
          },
        },
        {
          path: "/maps",
          name: "Map",
          component: Map_view,
          meta: {
            noAuth: true,
          },
        },
        {
          path: "/prepage/:page",
          name: "Prepage",
          component: Prepage_view,
          meta: {
            noAuth: true,
            navigation: true,
          },
        },
      ],
    },
    {
      path: "/Admin",
      name: "Admin",
      component: Admin_view,
      beforeEnter: () => {
        useFeedbackStore().fetchFeedback();
      },
      children: [
        {
          path: "prepages",
          name: "Prepages",
          component: Prepage_admin,
        },
        {
          path: "companies",
          name: "Companies",
          component: Companies_admin,
        },
        {
          path: "tags",
          name: "Tags",
          component: Tags_admin,
        },
        {
          path: "maps",
          name: "Maps",
          component: Map_admin,
        },
        {
          path: "shortcuts",
          name: "Shortcuts",
          component: Shortcuts_admin,
        },
        {
          path: "layout",
          name: "Layout",
          component: Layout_admin,
        },
        {
          path: "feedback",
          name: "Feedback",
          component: Feedback_admin,
        },
        {
          path: "batch",
          name: "Batch",
          component: Upload_admin,
        },
        {
          path: "account",
          name: "Account",
          component: Account_admin,
        },
        {
          path: "site",
          name: "Site settings",
          component: Site_admin,
        },
      ],
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
    useAuthStore().setAuthorizationHeader();
    useFavoritesStore().loadFromStorage();

//    const mapsStore = useMapsStore();

    await Promise.all([
      useTagsStore().getTags(), // This one fails if db is empty, check why
      useTagCategoriesStore().getTagCategories(),
      useCompaniesStore().fetchCompanies(),
      usePrepagesStore().getPrepages(),
      useLayoutsStore().getLayouts(),
      useShortcutsStore().getShortcuts(),
      useSite_settingsStore().fetchSettings(),
      useFairMapsStore().fetchMaps()
    ])
      .then(() => {
        useFilterStore().filterCompanies();
      })
      .catch(() => {}); // add some error here in the future?
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
