<template>
  <v-card :to="'/maps/' + map_object.ref">
    <v-card-title> We are here</v-card-title>
    <v-img max-height="400px" :src="base_URL + map_object.image" />
  </v-card>
</template>

<script>
import Vue from "vue";
export default {
  name: "Company_map",
  computed: {
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },
    map_object() {
      return this.$store.getters["maps/get"].filter(
        (t) => t.name == this.map
      )[0];
    },
  },

  props: ["map"],
};
</script>
