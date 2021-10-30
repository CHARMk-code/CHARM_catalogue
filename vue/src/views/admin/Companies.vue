<template>
  <v-container>
    <companies_table />
  </v-container>
</template>

<script>
import companies_table from "@/components/admin/company_table";

export default {
  name: "Companies",
  components: {
    companies_table,
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
  mounted() {
    this.$store.dispatch("companies/getCompanies");
  },
};
</script>
