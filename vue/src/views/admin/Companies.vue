<template>
  <v-container>
    <company_cards />
    <companies_table />
  </v-container>
</template>

<script>
import companies_table from "@/components/admin/company_table";
import company_cards from "@/components/admin/company_cards";

export default {
  name: "Companies",
  components: {
    companies_table,
    company_cards,
  },
  data() {
    return {
      headers: [
        { text: "Name", value: "name" },
        { text: "Website", value: "website" },
        { text: "Tags", value: "tags" },
        { text: "Active", value: "active" },
        { text: "Actions", value: "actions", sortable: false },
      ],
    };
  },
};
</script>
