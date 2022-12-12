<template>
  <v-app id="app">
    <v-app-bar color="white" app clipped-left>
      <v-app-bar-nav-icon
        class="hidden-md-and-up"
        @click="drawer = true"
      ></v-app-bar-nav-icon>

      <v-spacer class="hidden-md-and-up" />

      <router-link to="/">
        <img class="logo" src="@/assets/CHARM_logo.png" />
      </router-link>

      <v-spacer />

      <v-btn
        class="white mr-6 hidden-sm-and-down"
        v-for="link in links"
        :key="link.name"
        :to="link.route"
        depressed
      >
        {{ link.name }}
        <v-icon right>{{ link.icon }}</v-icon>
      </v-btn>
      <v-btn
        class="hidden-sm-and-down"
        color="primary"
        v-on:click="logout"
        v-if="this.isLoggedIn"
        depressed
      >
        Logout
      </v-btn>
    </v-app-bar>

    <v-navigation-drawer v-model="drawer" absolute temporary>
      <router-link class="ml-4" to="/">
        <img class="logo" src="@/assets/CHARM_logo.png" />
      </router-link>
      <v-list nav dense>
        <v-list-item-group v-model="group">
          <v-list-item to="/">
            <v-list-item-icon>
              <v-icon>mdi-home</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Home</v-list-item-title>
          </v-list-item>

          <v-list-item to="/search">
            <v-list-item-icon>
              <v-icon>mdi-magnify</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Search</v-list-item-title>
          </v-list-item>
        </v-list-item-group>
      </v-list>

      <v-btn
        class="ml-4 mt-4"
        color="primary"
        v-on:click="logout"
        v-if="this.isLoggedIn"
        depressed
      >
        Logout
      </v-btn>
    </v-navigation-drawer>
    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>

<script>
import { useAuthStore } from "@/stores/modules/auth"

export default {
  name: "Header",
  components: {},
  data() {
    return {
      drawer: false,
      group: null,
      links: [{ name: "Search", route: "/search", icon: "mdi-magnify" }],
    };
  },
  computed: {
    isLoggedIn() {
      return useAuthStore().isLoggedIn
    }
  },
  methods: {
    logout() {
      useAuthStore().logout();
      this.$router.push("/");
    },
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.logo {
  height: 60px;
  padding: 6.5px 1px;
}
</style>
