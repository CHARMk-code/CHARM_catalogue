<template>
  <v-app-bar color="white" app clipped-left>
    <v-row fill-height>
      <v-col class="flex-grow-0 flex-shrink-1" justify="start">
        <router-link to="/1">
          <img class="logo" src="@/assets/CHARM_logo.png" />
        </router-link>
      </v-col>
      <v-col class="flex-grow-1 flex-shrink-1 mt-4" justify="center">
        <v-btn
          class="white"
          v-for="link in links"
          :key="link.name"
          :to="link.route"
          depressed
        >
          {{ link.name }}
          <v-icon right>{{ link.icon }}</v-icon>
        </v-btn>
      </v-col>
      <v-col class="flex-grow-0 flex-shrink-1 mt-4" justify="end">
        <v-btn
          color="primary"
          v-on:click="logout"
          v-if="this.$store.getters['auth/isLoggedIn']"
          depressed
        >
          Logout
        </v-btn>
      </v-col>
    </v-row>
  </v-app-bar>
</template>

<script>
export default {
  name: "Header",
  components: {},
  data() {
    return {
      links: [{ name: "Search", route: "/search", icon: "mdi-magnify" }],
    };
  },
  methods: {
    logout() {
      this.$store.dispatch("auth/logout");
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
