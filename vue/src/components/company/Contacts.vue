<template>
  <company_card_wrapper name="contacts" v-if="contacts != ''">
    <v-card-title> Contact information </v-card-title>
    <v-card-text v-if="contacts.includes('https://')">
      <a :href="contacts"> {{ contacts }} </a>
    </v-card-text>
    <v-card-text v-else>{{ contacts }} ({{ contact_email }}) </v-card-text>
  </company_card_wrapper>
</template>

<script>
import company_card_wrapper from "@/components/company/card_wrapper";

export default {
  name: "Company_Contacts",
  props: ["contacts", "contact_email"],
  components: {
    company_card_wrapper,
  },
};
</script>
