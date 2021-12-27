<template>
  <v-img
    v-if="layout != undefined"
    :src="base_URL + layout.image"
    max-height="150"
    max-width="500"
  />
</template>

<script>
import Vue from "vue";
export default {
  name: "Company_layout",
  data() {
    return {
      layout: undefined,
    };
  },
  computed: {
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },
  },
  mounted() {
    console.log("mounting");
    this.layout = this.$store.getters["layouts/getMiddle"];
  },
};
</script>
