<template>
  <company_card_wrapper :name="name">
    <v-card-title> {{ title }} </v-card-title>
    <v-card-text>
      <template v-if="tags.length === 0">
        <span>No {{ title }}</span>
      </template>
      <template v-else v-for="tag in tags">
        <TagComp :tag="tag"></TagComp>
      </template>
    </v-card-text>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";
import axios from "@/plugins/axios";
import type { Tag } from "@/stores/modules/tags";
import TagComp from "../Tag.vue";

const base_URL = axios.defaults.baseURL + "/manage/image/";

const props = defineProps<{
  title: string;
  name: string;
  tags: Tag[];
}>();
</script>
