<template>
  <v-main>
    <v-row>
      <v-col cols="2">
        <v-img
          v-if="leftSide != undefined"
          height="100%"
          :src="base_URL + leftSide.image"
        />
      </v-col>
      <v-col cols="8">
        <slot />
      </v-col>
      <v-col cols="2">
        <v-img
          v-if="rightSide != undefined"
          height="100%"
          :src="base_URL + rightSide.image"
        />
      </v-col>
    </v-row>
  </v-main>
</template>

<script>
import Vue from "vue";
import { mapGetters } from "vuex";
export default {
  name: "sideLayout",
  computed: {
    ...mapGetters({ getSide: "layouts/getSide" }),
    leftSide() {
      return this.getSide(1);
    },
    rightSide() {
      return this.getSide(2);
    },
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },
  },
};
</script>
