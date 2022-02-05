<template>
  <company_card_wrapper name="website" v-if="website != ''">
    <v-card-title> Website </v-card-title>
    <v-card-text>
      <a :href="website">
        {{ website }}
      </a>
    </v-card-text>
  </company_card_wrapper>
</template>

<script>
import company_card_wrapper from "@/components/company/card_wrapper";

export default {
  name: "Company_website",
  props: ["website"],
  components: {
    company_card_wrapper,
  },
};
</script>
