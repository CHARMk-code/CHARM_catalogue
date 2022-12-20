<template>
  <company_card_wrapper name="summerjob">
    <v-card-title> Summer Jobs</v-card-title>

    <v-card-text> {{ desc }}</v-card-text>
    <v-card-text> Apply before: {{ deadline }}</v-card-text>

    <v-card-actions>
      <v-btn :href="link"> Apply here </v-btn>
    </v-card-actions>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

const props = defineProps<{
  desc: string;
  link: string;
  deadline: string;
}>();
</script>
