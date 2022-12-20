<template>
  <company_card_wrapper name="desc" v-if="body != ''">
    <v-card-title>
      {{ title }}
    </v-card-title>
    <v-card-text>
      {{ body }}
    </v-card-text>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

const props = withDefaults(
  defineProps<{
    title?: string;
    body: string;
  }>(),
  { title: "" }
);
</script>
