<template>
  <company_card_wrapper name="logo" flat>
    <v-img
      v-if="src != ''"
      :src="base_URL + src"
      max-height="150"
      max-width="500"
      contain
    />
  </company_card_wrapper>
</template>

<script>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

import Vue from "vue";
export default {
  name: "Company_logo",
  components: {
    company_card_wrapper,
  },
  computed: {
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },
  },
  props: ["src"],
};
</script>
