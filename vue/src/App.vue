<template>
  <v-app id="app">
    <Header />
    <router-view />
    <CookieConsent href="cookies">
      <template slot="button">
        <v-btn class="primary x-large ml-5">Got it</v-btn>
      </template>
    </CookieConsent>
  </v-app>
</template>

<script>
import Header from "@/components/Header";
import CookieConsent from "vue-cookieconsent-component";
export default {
  name: "App",
  components: {
    Header,
    CookieConsent,
  },
  created() {
    this.$axios.defaults.headers.common["Authorization"] =
      "Basic " + this.$store.getters["auth/token"];
    this.$store.commit("favorites/loadForStorage");
    this.$store.dispatch("filter/filterCompanies");
    this.$store.dispatch("maps/getMaps");
    this.$store.dispatch("tags/getTags");
    this.$store.dispatch("companies/getCompanies");
    this.$store.dispatch("filter/filterCompanies");
    this.$store.dispatch("prepages/getPrepages");
  },
};
</script>
<style scoped lang="scss">
@import "./node_modules/vue-cookieconsent-component/src/scss/cookie-consent";
// example or use it
@import "./node_modules/vue-cookieconsent-component/src/scss/cookie-consent-bottom";
@import "./node_modules/vue-cookieconsent-component/src/scss/cookie-consent-transition";
</style>
