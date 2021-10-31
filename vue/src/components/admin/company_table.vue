<template>
  <v-container>
    <Table
      @save_edit="saveCompany"
      name="Companies"
      :headers="headers"
      :data="Array.from(this.companies)"
      :row_meta="row_meta"
      :editable="true"
    />
  </v-container>
</template>

<script>
import Table from "@/components/table";
import { mapGetters } from "vuex";

export default {
  name: "companies_table",
  components: {
    Table,
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
      //make sure to get from store instead
      allTags: ["a", "b", "c", "d", "e", "f", "h", "i", "k", "m"],
      // Logo?
      row_meta: [
        {
          type: "checkbox",
          model: "active",
          on_icon: "mdi-eye",
          off_icon: "mdi-eye-off",
          label: "Active (required for row to be visible)",
        },
        { type: "text", model: "name", label: "Company name" },
        {
          type: "textarea",
          model: "description",
          label: "Company description",
        },
        { type: "text", model: "founded", label: "Founded" },
        { type: "text", model: "Contacts", label: "Contacts" },
        { type: "text", model: "website", label: "Website" },
        {
          type: "text",
          model: "employees_sweden",
          label: "Number of Employees in Sweden",
        },
        {
          type: "text",
          model: "employees_world",
          label: "Number of Employees in the whole world",
        },
        { type: "text", model: "trivia", label: "Trivia" },
        {
          type: "select",
          model: "tags",
          items: this.allTags,
          label: "Tags",
          hint: "Active tags for Company",
        },
      ],
    };
  },
  computed: {
    ...mapGetters({ companies: "companies/companies" }),
  },
  methods: {
    saveCompany(company) {
      this.$store.dispatch("companies/modifyCompany", company);
    },
  },
};
</script>
