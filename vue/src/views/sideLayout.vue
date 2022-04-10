<template>
  <v-main>
    <v-btn
      v-if="button_left"
      class="navigation hidden-sm-and-down"
      elevation="4"
      x-large
      v-on:click="$emit('prev')"
      icon
    >
      <v-icon x-large>mdi-arrow-left</v-icon>
    </v-btn>
    <v-btn
      v-if="button_right"
      class="navigation hidden-sm-and-down"
      v-on:click="$emit('next')"
      style="right: 0%"
      x-large
      elevation="4"
      icon
    >
      <v-icon x-large>mdi-arrow-right</v-icon>
    </v-btn>
    <div
      style="position: fixed; top: 0; width: 100vw"
      class="hidden-sm-and-down"
    >
      <v-container
        class="ma-0 pa-0 d-flex justify-content-between"
        style="width: 100vw"
      >
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
          <v-col cols="2" class="ma-0 pa-0">
            <v-img
              class="ml-0"
              max-height="100vh"
              v-if="rightSide != undefined"
              :src="base_URL + rightSide.image"
            />
          </v-col>
        </v-row>
      </v-container>
    </div>
    <v-container
      style="position: absolute; top: 0; width: 100vw; max-width: 100%"
    >
      <v-row class="justify-center">
        <v-col
          xs="0"
          md="2"
          style="height: 0px"
          class="pa-0 ma-0 hidden-sm-and-down"
        />
        <v-col xs="12" md="8" class="pa-0 ma-md-0 ma-sm-0">
          <slot />
        </v-col>
        <v-col
          xs="0"
          md="2"
          style="height: 0px"
          class="pa-0 ma-0 hidden-sm-and-down"
        />
      </v-row>
    </v-container>
  </v-main>
</template>

<script>
import Vue from "vue";
import { mapGetters } from "vuex";
export default {
  name: "sideLayout",
  props: ["button_left", "button_right"],
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

<style scoped>
.navigation {
  text-decoration: none;
  margin: 50px;
  position: fixed;
  z-index: 9999999;
  top: 50%;
}
.navigation > * {
  top: -50%;
}
</style>
