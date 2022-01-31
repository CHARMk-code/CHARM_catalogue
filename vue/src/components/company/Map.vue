<template>
  <company_card_wrapper name="map">
    <v-card-title> We are here</v-card-title>
    <v-sheet :to="'/maps/' + map_object.ref">
      <v-img max-height="400px" :src="base_URL + map_object.image" />
    </v-sheet>
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

  props: ["map"],
};
</script>
