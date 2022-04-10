<template>
  <v-main>
    <v-sheet class="pa-6">
      <v-img contain justify="center" :src="base_URL + map_src" />
    </v-sheet>
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
      console.log(this.$store.getters["maps/get"]);
      return this.$store.getters["maps/get"].filter(
        (t) => t.name == this.$route.params.page
      )[0].image;
    },
  },
};
</script>
