<template>
  <v-img
    v-if="src != ''"
    :src="base_URL + src"
    max-height="150"
    max-width="500"
    contain
  />
</template>

<script>
import Vue from "vue";
export default {
  name: "Company_logo",
  computed: {
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },
  },
  props: ["src"],
};
</script>
