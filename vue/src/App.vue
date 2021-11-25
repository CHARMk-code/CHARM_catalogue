<template>
  <v-app id="app">
    <Header />
    <router-view />
  </v-app>
</template>

<script>
import Header from "@/components/Header";

export default {
  name: "App",
  components: {
    Header,
  },
  created() {
    this.$axios.defaults.headers.common["Authorization"] =
      "Basic " + this.$store.getters["auth/token"];
  },
};
</script>
