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

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";
import axios from "@/plugins/axios";

const props = defineProps<{
  src: string;
}>();

const base_URL = axios.defaults.baseURL + "/manage/image/";
</script>
