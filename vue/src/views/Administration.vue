<template>
  <q-layout view="hHh LpR fFf">
    <q-header elevated class="bg-white text-primary">
      <q-toolbar>
        <q-btn to="/" flat>
          <q-toolbar-title>
            <img class="logo" src="@/assets/CHARM_logo.png" />
          </q-toolbar-title>
        </q-btn>
        <q-space></q-space>
      </q-toolbar>
    </q-header>

    <q-drawer persistent side="left" bordered :modelValue="true" :width="200">
      <q-list>
        <q-separator></q-separator>
        <q-item
          v-for="link in links"
          clickable
          :active="route.name === link.name"
          v-ripple
          :to="link.route"
        >
          <q-item-section avatar>
            <q-icon :name="link.icon"></q-icon>
          </q-item-section>
          <q-item-section>
            {{ link.name }}
          </q-item-section>
        </q-item>
        <q-separator v-if="authStore.isLoggedIn"></q-separator>
        <q-item v-if="authStore.isLoggedIn">
          <q-item-section>
            <q-btn color="primary" text-color="white" @click="logout"
              >Logout</q-btn
            >
          </q-item-section>
        </q-item>
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
import { useAuthStore } from "@/stores/modules/auth";
import { useLayoutsStore } from "@/stores/modules/layouts";
import { computed, ref } from "vue";
import { useRoute, useRouter } from "vue-router";

const emit = defineEmits<{
  (e: "next"): void;
  (e: "prev"): void;
}>();

const route = useRoute();
const router = useRouter();
const authStore = useAuthStore();
const layoutsStore = useLayoutsStore();

const leftDrawerOpen = ref(false);

const base_URL = axios.defaults.baseURL + "/manage/image/";

const links: { name: string; route: string; icon: string }[] = [
  {
    name: "prepages",
    route: "/admin/prepages",
    icon: "mdi-book-open-page-variant",
  },
  { name: "companies", route: "/admin/companies", icon: "mdi-account-group" },
  { name: "Tags", route: "/admin/tags", icon: "mdi-tag-heart-outline" },
  { name: "maps", route: "/admin/maps", icon: "mdi-map" },
  { name: "layout", route: "/admin/layout", icon: "mdi-human-male-board" },
  { name: "shortcuts", route: "/admin/shortcuts", icon: "mdi-star" },
  { name: "feedback", route: "/admin/feedback", icon: "mdi-forum" },
  { name: "batch", route: "/admin/batch", icon: "mdi-tray-arrow-up" },
  { name: "account", route: "/admin/account", icon: "mdi-account" },
];

const leftLayout = computed(() => layoutsStore.getSide("left"));
const rightLayout = computed(() => layoutsStore.getSide("right"));

function logout() {
  useAuthStore().logout();
  router.push("/");
}
</script>

<style scoped>
.logo {
  height: 40px;
  padding: 6.5px 1px;
}
.navigation {
  text-decoration: none;
  margin: 50px;
  position: fixed;
  z-index: 9999999;
  top: 50%;
}
.navigation > * {
  top: -100%;
}
</style>
