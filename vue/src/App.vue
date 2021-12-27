<template>
  <v-app id="app">
    <Header />
    <v-row>
      <v-col cols="2">
        <v-img height="100%" :src="base_URL + getSide(1).image" />
      </v-col>
      <v-col cols="8">
        <router-view />
      </v-col>
      <v-col cols="2">
        <v-img height="100%" :src="base_URL + getSide(2).image" />
      </v-col>
    </v-row>
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
import Vue from "vue";
import { mapGetters } from "vuex";
export default {
  name: "App",
  components: {
    Header,
    CookieConsent,
  },
  computed: {
    ...mapGetters({ getSide: "layouts/getSide" }),
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },
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
    this.$store.dispatch("layouts/getLayouts");
  },
};
</script>
<style scoped lang="scss">
@import "./node_modules/vue-cookieconsent-component/src/scss/cookie-consent";
// example or use it
@import "./node_modules/vue-cookieconsent-component/src/scss/cookie-consent-bottom";
@import "./node_modules/vue-cookieconsent-component/src/scss/cookie-consent-transition";
</style>
