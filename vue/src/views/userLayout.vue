<template>
  <q-layout view="hhh LpR fFf">
    <q-header reveal elevated class="bg-white text-primary">
      <q-toolbar>
        <q-btn
          dense
          flat
          round
          icon="menu"
          @click="leftDrawerOpen = !leftDrawerOpen"
        />

        <q-btn to="/" flat>
          <q-toolbar-title>
            <img 
              class="logo"
              :src="axios.defaults.baseURL + '/v2/image/' + useSite_settingsStore().server_settings.theme.logo"

            >
          </q-toolbar-title>
        </q-btn>
        <q-space></q-space>
        <q-btn
          flat
          color="primary"
          icon-right="mdi-magnify"
          label="Search"
          to="/search"
        ></q-btn>
      </q-toolbar>
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      side="left"
      overlay
      bordered
      :width="200"
      class="column"
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
        <q-item v-if="authStore.isLoggedIn" v-ripple clickable to="/admin">
          <q-item-section avatar>
            <q-icon name="mdi-cog"></q-icon>
          </q-item-section>
          <q-item-section> Admin </q-item-section>
        </q-item>
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

    <q-drawer
      v-if="$q.screen.gt.sm"
      :width="200"
      side="left"
      persistent
      show-if-above
    >
      <div class="navigation">
        <q-btn
          v-if="hasPrev && $route.meta.navigation"
          elevation="4"
          icon="mdi-arrow-left"
          size="lg"
          round
          @click="prev()"
        ></q-btn>
      </div>
      <Image
        v-if="leftLayout"
        class="ma-0 pa-0"
        height="100%"
        :image-name="leftLayout.image"
      />
    </q-drawer>

    <q-drawer
      v-if="$q.screen.gt.sm"
      :width="200"
      side="right"
      persistent
      show-if-above
    >
      <div class="navigation">
        <q-btn
          v-if="hasNext && $route.meta.navigation"
          elevation="4"
          icon="mdi-arrow-right"
          size="lg"
          round
          @click="next()"
        ></q-btn>
      </div>
      <Image
        v-if="rightLayout"
        class="ma-0 pa-0"
        height="100%"
        :image-name="rightLayout.image"
      />
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script lang="ts" setup>
import Image from "@/components/utils/Image.vue";
import { useAuthStore } from "@/stores/modules/auth";
import { useLayoutsStore } from "@/stores/modules/layouts";
import { useSite_settingsStore } from "@/stores/modules/site_settings";
import { computed, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import feedback from "@/components/feedback.vue";
import { useQuasar } from "quasar";
import axios from "@/plugins/axios";

const $q = useQuasar();

const route = useRoute();
const router = useRouter();
const authStore = useAuthStore();
const layoutsStore = useLayoutsStore();
const site_settingsStore = useSite_settingsStore();

const leftDrawerOpen = ref(false);

const links = [
  { name: "Home", route: "/", icon: "mdi-home" },
  { name: "Search", route: "/search", icon: "mdi-magnify" },
  { name: "Map", route: "/map", icon: "mdi-map" },
];

const leftLayout = computed(() => layoutsStore.getSide("left"));
const rightLayout = computed(() => layoutsStore.getSide("right"));
const hasNext = computed(
  () => site_settingsStore.session_settings.navigation.next !== undefined
);

const hasPrev = computed(
  () => site_settingsStore.session_settings.navigation.prev !== undefined
);

function next() {
  const maybeNext: string | undefined = site_settingsStore.consumeNext();
  if (maybeNext) router.push(maybeNext);
}

function prev() {
  const maybePrev: string | undefined = site_settingsStore.consumePrev();
  if (maybePrev) router.push(maybePrev);
}

function logout() {
  useAuthStore().logout();
  router.push("/");
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.logo {
  height: 40px;
  object-fit: contain;
}
.navigation {
  text-decoration: none;
  margin: 50px;
  position: fixed;
  z-index: 9999999;
  top: 50%;
}

.navigation button {
  background-color: rgba(255, 255, 255, 0.75);
}

.navigation > * {
  top: -100%;
}
</style>
