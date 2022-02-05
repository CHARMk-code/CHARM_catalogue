<template>
  <v-container>
    <company_cards />
    <layouts_table />
  </v-container>
</template>

<script>
import layouts_table from "@/components/admin/layout_table";
import company_cards from "@/components/admin/company_cards";

export default {
  name: "Layout",
  components: {
    layouts_table,
    company_cards,
  },
};
</script>
