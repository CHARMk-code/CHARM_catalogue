<template>
  <company_card_wrapper name="contacts" v-if="contacts != ''">
    <v-card-title> Contact information </v-card-title>
    <v-card-text v-if="contacts.includes('https://')">
      <a :href="contacts"> {{ contacts }} </a>
    </v-card-text>
    <v-card-text v-else>{{ contacts }} ({{ contact_email }}) </v-card-text>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

const props = defineProps<{
  contacts: string;
  contact_email: string;
}>();
</script>
