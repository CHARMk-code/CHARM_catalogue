<template>
  <v-app-bar color="white" app clipped-left>
    <v-row>
      <v-col class="pa-0 pt-2 pl-6">
        <router-link to="/1">
          <img class="logo" src="@/assets/CHARM_logo.png" />
        </router-link>
      </v-col>

      <v-col class="pa-0 pt-3 pr-6">
        <router-link
          style="text-decoration: none; color: inherit; float: right"
          v-for="link in links"
          :key="link.name"
          :to="link.route"
        >
          <v-btn icon>
            <v-icon large>{{ link.icon }}</v-icon>
          </v-btn>
        </router-link>
      </v-col>
    </v-row>
    <v-spacer />
    <v-btn
      color="primary"
      class="mt-4"
      v-on:click="logout"
      v-if="this.$store.getters['auth/isLoggedIn']"
    >
      Logout
    </v-btn>
  </v-app-bar>
</template>

<script>
export default {
  name: "Header",
  components: {},
  props: {
    search_disabled: {
      type: Boolean,
      default: false,
    },
  },
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
.header-inner {
  background: black;
  padding-left: 80px;
  padding-right: 80px;
  padding-top: 15px;
}

.logo {
  height: 60px;
  padding: 6.5px 1px;
}

header > div {
  display: flex;
  justify-content: space-between;
}
p {
  color: white;
  font-size: 2em;
  margin: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
