<template>
  <company_card_wrapper name="desc" v-if="body != ''">
    <q-card-section>
      <div class="text-h6">{{ title }}</div>
      {{ body }}
    </q-card-section>
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
