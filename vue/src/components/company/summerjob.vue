<template>
  <company_card_wrapper name="summerjob">
    <v-card-title> Summer Jobs</v-card-title>

    <v-card-text> {{ desc }}</v-card-text>
    <v-card-text> Apply before: {{deadline.getFullYear()}}-{{ deadline.getMonth() }}-{{ deadline.getDate() }}</v-card-text>

    <v-card-actions>
      <v-btn :href="link"> Apply here </v-btn>
    </v-card-actions>
  </company_card_wrapper>
</template>

<script>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

export default {
  name: "summer-job",
  props: ["desc", "link", "deadline"],
  components: {
    company_card_wrapper,
  },
};
</script>
