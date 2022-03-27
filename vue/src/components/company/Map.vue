<template>
  <company_card_wrapper name="map">
    <v-card-title> Location </v-card-title>
    <v-sheet :to="'/maps/' + map_object.ref">
      <v-img class="ma-2" max-height="400px" contain :src="base_URL + map_object.image" />
    </v-sheet>
    <v-card-text> <v-icon>mdi-map-marker</v-icon> {{booth_number}}</v-card-text>

  </company_card_wrapper>
</template>

<script>
import Vue from "vue";
import company_card_wrapper from "@/components/company/card_wrapper";

export default {
  name: "Company_map",
  components: {
    company_card_wrapper,
  },
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

  props: ["map","booth_number"],
};
</script>
