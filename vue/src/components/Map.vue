<template>
  <v-main>
    <v-card max-height="70vh" max-width="70vw" class="mx-auto">
      <img contain center :src="base_URL + map_src" />
    </v-card>
  </v-main>
</template>

<script>
import Vue from "vue";
export default {
  name: "Map",
  computed: {
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },
    map_src() {
      return this.$store.getters["maps/get"].filter(
        (t) => t.name == this.$route.params.page
      )[0].image;
    },
  },
};
</script>
