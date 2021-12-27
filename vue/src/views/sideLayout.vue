<template>
  <!-- <v-row> -->
  <!--   <v-col cols="2" class="hidden-sm-and-down"> -->
  <!--     <v-img -->
  <!--       v-if="leftSide != undefined" -->
  <!--       height="100%" -->
  <!--       :src="base_URL + leftSide.image" -->
  <!--     /> -->
  <!--   </v-col> -->
  <!--   <v-col> -->
  <!--     <slot /> -->
  <!--   </v-col> -->
  <!--   <v-col cols="2" class="hidden-sm-and-down"> -->
  <!--     <v-img -->
  <!--       v-if="rightSide != undefined" -->
  <!--       height="100%" -->
  <!--       :src="base_URL + rightSide.image" -->
  <!--     /> -->
  <!--   </v-col> -->
  <!-- </v-row> -->
  <v-main>
    <div
      style="position: fixed; top: 0; width: 100vw"
      class="hidden-sm-and-down"
    >
      <div class="ma-0 pa-0 d-flex justify-content-between" style="width: 100%">
        <v-row class="ma-0 pa-0">
          <v-col cols="2" class="pa-0">
            <v-img
              class="ma-0 pa-0"
              max-height="100vh"
              v-if="leftSide != undefined"
              :src="base_URL + leftSide.image"
            />
          </v-col>
          <v-spacer />
          <v-col cols="2" class="ma-0">
            <v-img
              class="ml-0"
              max-height="100vh"
              v-if="rightSide != undefined"
              :src="base_URL + rightSide.image"
            />
          </v-col>
        </v-row>
      </div>
    </div>
    <div style="position: absolute; top: 0; width: 100vw">
      <v-row>
        <v-col cols="2" />
        <v-col>
          <slot />
        </v-col>
        <v-col cols="2" />
      </v-row>
    </div>
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
