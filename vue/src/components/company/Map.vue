<template>
  <company_card_wrapper class="mb-6" name="map">
    <v-card-title> Location </v-card-title>
    <v-card-text class="text-subtitle-1 text-center">
      <v-icon color="primary" large>mdi-map-marker</v-icon> Booth #{{
        booth_number
      }}
      <v-card flat :to="'/maps/' + map_ref_object.name">
        <v-img
          class="ma-2"
          max-height="400px"
          contain
          :src="base_URL + map_object.image"
        />
      </v-card>
    </v-card-text>
  </company_card_wrapper>
</template>

<script>
import Vue from "vue";
import company_card_wrapper from "@/components/company/card_wrapper.vue";

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
    map_ref_object() {
      return this.$store.getters["maps/get"].filter(
        (t) => t.id == this.map_object.ref
      )[0];
    },
  },

  props: ["map", "booth_number"],
};
</script>
