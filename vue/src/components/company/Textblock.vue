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

<script>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

export default {
  name: "Company_text",
  props: ["body", "title"],
  components: {
    company_card_wrapper,
  },
};
</script>
