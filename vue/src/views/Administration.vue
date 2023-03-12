<template>
  <q-layout view="hHh LpR fFf">
    <q-header elevated class="bg-white text-primary">
      <q-toolbar>
        <q-btn to="/" flat>
          <q-toolbar-title>
            <img
              class="logo"
              :src="
                axios.defaults.baseURL +
                '/manage/image/' +
                useSite_settingsStore().settings.theme.logo
              "
            />
          </q-toolbar-title>
        </q-btn>
        <q-space></q-space>
      </q-toolbar>
    </q-header>

    <q-drawer
      persistent
      side="left"
      class="column"
      bordered
      :model-value="true"
      :width="200"
    >
      <q-list>
        <q-separator></q-separator>
        <q-item
          v-for="link in links"
          :key="link.name"
          v-ripple
          clickable
          :active="route.name === link.name"
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
      <q-space />
      <q-list>
        <q-item
          flat
          class="q-mb-md text-primary"
          icon="mdi-forum"
          clickable
          @click="$q.dialog({ component: feedback })"
        >
          <q-item-section avatar> <q-icon name="mdi-forum" /> </q-item-section>
          <q-item-section>
            <div>Report a problem</div>
            <div>or give feedback</div>
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
import { useAuthStore } from "@/stores/modules/auth";
import { useRoute, useRouter } from "vue-router";
import feedback from "@/components/feedback.vue";
import { useQuasar } from "quasar";
import { useSite_settingsStore } from "@/stores/modules/site_settings";
import axios from "@/plugins/axios";

const $q = useQuasar();

defineEmits<{
  (e: "next"): void;
  (e: "prev"): void;
}>();

const route = useRoute();
const router = useRouter();
const authStore = useAuthStore();

const links: { name: string; route: string; icon: string }[] = [
  {
    name: "Prepages",
    route: "/admin/prepages",
    icon: "mdi-book-open-page-variant",
  },
  { name: "Companies", route: "/admin/companies", icon: "mdi-account-group" },
  { name: "Tags", route: "/admin/tags", icon: "mdi-tag-heart-outline" },
  { name: "Maps", route: "/admin/maps", icon: "mdi-map" },
  { name: "Layout", route: "/admin/layout", icon: "mdi-human-male-board" },
  { name: "Shortcuts", route: "/admin/shortcuts", icon: "mdi-star" },
  { name: "Feedback", route: "/admin/feedback", icon: "mdi-forum" },
  { name: "Batch", route: "/admin/batch", icon: "mdi-tray-arrow-up" },
  { name: "Account", route: "/admin/account", icon: "mdi-account" },
  { name: "Site settings", route: "/admin/site", icon: "mdi-cog" },
];

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
