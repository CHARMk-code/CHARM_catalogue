<template>
  <v-app id="app">
    <Header :search_disabled="search_disabled" />
    <router-view />
  </v-app>
</template>

<script>
import Header from "@/components/Header";
import { mapActions } from "vuex";

export default {
  name: "App",
  components: {
    Header,
  },
  watch: {
    $route(to) {
      if (to.name === "Search") {
        this.search_disabled = true;
      } else {
        this.search_disabled = false;
      }
    },
  },
  created() {
    this.retrieveCompanies();
    this.retrieveTags();
  },
  data() {
    return {
      search_disabled: false,
    };
  },
  methods: {
    ...mapActions(["retrieveCompanies", "retrieveTags"]),
  },
};
</script>

<style>
* {
  font-family: Open Sans Condensed;
}

a {
  text-style: none;
}

body {
  margin: 0px;
}
svg {
  color: red;
}
</style>
