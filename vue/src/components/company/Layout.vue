<template>
  <v-img v-if="layout != undefined" :src="base_URL + layout.image" contain />
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
    this.layout = this.$store.getters["layouts/getMiddle"];
  },
};
</script>
