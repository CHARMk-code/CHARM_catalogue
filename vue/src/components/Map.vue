<template>
  <v-main>
    <v-sheet
      min-height="50vh"
      rounded="lg"
      class="d-flex"
      style="position: relative"
    >
      <img
        style="margin: auto"
        height="700px"
        width="1400px"
        justify="center"
        :src="base_URL + map_src"
      />
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
      return this.$store.getters["maps/get"].filter(
        (t) => t.name == this.$route.params.page
      )[0].image;
    },
  },
};
</script>
