<template>
  <v-img
    v-if="src != ''"
    :src="base_URL + getMiddle.image"
    max-height="150"
    max-width="500"
  />
</template>

<script>
import Vue from "vue";
import { mapGetters } from "vuex";
export default {
  name: "Company_layout",
  computed: {
    ...mapGetters({ getMiddle: "layouts/getMiddle" }),
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },
  },
};
</script>
