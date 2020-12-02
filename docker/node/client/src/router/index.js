import Vue from "vue";
import VueRouter from "vue-router";
Vue.use(VueRouter);

const routes = [
  {
    path: "/",
    name: "search",
    component: () => import("../components/Search.vue"),
  },
  {
    path: "/add",
    name: "add_tag",
    component: () => import("../components/Add_tag.vue"),
  },
  {
    path: "/create",
    name: "create_tag",
    component: () => import("../components/Create_tag.vue"),
  }
];

const router = new VueRouter({
  mode: "history",
  base: process.env.BASE_URL,
  routes,
});

export default router;
