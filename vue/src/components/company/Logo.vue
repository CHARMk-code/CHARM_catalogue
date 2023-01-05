<template>
  <company_card_wrapper name="logo" flat>
    <q-card-section>
      <q-img
        v-if="src != ''"
        :src="base_URL + src"
        height="150px"
        fit="contain"
        :position="$q.screen.lt.md ? '50% 50%' : 'left 50%'"
      />
    </q-card-section>
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
