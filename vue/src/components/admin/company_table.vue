<template>
  <v-container>
    <Table
      @save_edit="saveCompany"
      name="Companies"
      :headers="headers"
      :data="modified_companies"
      :row_meta="row_meta"
      :editable="true"
    >
      <template v-slot:item.active="{ item }">
        <v-simple-checkbox
          disabled
          on-icon="mdi-eye"
          off-icon="mdi-eye-off"
          v-model="item.active"
        ></v-simple-checkbox>
      </template>

      <template v-slot:extra_actions="{ item }">
        <v-icon class="mr-2" @click="viewCompany(item)"> mdi-book-open </v-icon>
      </template>

      <template v-slot:item.website="{ item }">
        <a :href="item.website">{{ item.website }}</a>
      </template>

      <template v-slot:item.tags="{ item }">
        <template v-for="tag in tagsFromIDs(item.tags)">
          <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
            tag.name
          }}</v-chip>
        </template>
      </template>
      <template v-slot:item.divisions="{ item }">
        <template v-for="tag in tagsFromIDs(item.divisions)">
          <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
            tag.name
          }}</v-chip>
        </template>
      </template>
      <template v-slot:item.looking_for="{ item }">
        <template v-for="tag in tagsFromIDs(item.looking_for)">
          <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
            tag.name
          }}</v-chip>
        </template>
      </template>
      <template v-slot:item.offering="{ item }">
        <template v-for="tag in tagsFromIDs(item.offering)">
          <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
            tag.name
          }}</v-chip>
        </template>
      </template>
      <template v-slot:item.business_area="{ item }">
        <template v-for="tag in tagsFromIDs(item.business_area)">
          <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
            tag.name
          }}</v-chip>
        </template>
      </template>
    </Table>
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
        { text: "Programs", value: "divisions" },
        { text: "Business areas", value: "business_area" },
        { text: "Looking for", value: "looking_for" },
        { text: "offering", value: "offering" },
        { text: "Other tags", value: "" },
        { text: "Active", value: "active", align: "center", width: 110 },
        {
          text: "Actions",
          value: "actions",
          align: "center",
          width: 100,
          sortable: false,
        },
      ],
    };
  },
  computed: {
    ...mapGetters({ companies: "companies/companies", tags: "tags/tags" }),
    modified_companies() {
      const companies = Array.from(this.companies);
      const modified = companies.map((c) => ({
        ...c,
        divisions: c.tags
          .map((id) => this.$store.getters["tags/getTagFromId"](id))
          .filter((t) => t.division)
          .map((t) => t.id),
        looking_for: c.tags
          .map((id) => this.$store.getters["tags/getTagFromId"](id))
          .filter((t) => t.looking_for)
          .map((t) => t.id),
        offering: c.tags
          .map((id) => this.$store.getters["tags/getTagFromId"](id))
          .filter((t) => t.offering)
          .map((t) => t.id),
        business_area: c.tags
          .map((id) => this.$store.getters["tags/getTagFromId"](id))
          .filter((t) => t.business_area)
          .map((t) => t.id),
        tags: c.tags
          .map((id) => this.$store.getters["tags/getTagFromId"](id))
          .filter(
            (t) =>
              !(t.divisions || t.looking_for || t.offering || t.business_area)
          )
          .map((t) => t.id),
      }));
      return modified;
    },
    row_meta() {
      return [
        {
          type: "checkbox",
          model: "active",
          on_icon: "mdi-eye",
          off_icon: "mdi-eye-off",
          label: "Active (required for row to be visible)",
        },
        { type: "text", model: "name", label: "Company name" },
        { type: "file", model: "logo", label: "Company Logo" },
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
          model: "divisions",
          items: this.tags
            .filter((t) => t.division)
            .map((t) => ({ text: t.name, value: t.id })),
          label: "Divisions",
          hint: "Programs the company are interested in",
        },
        {
          type: "select",
          model: "looking_for",
          items: this.tags
            .filter((t) => t.looking_for)
            .map((t) => ({ text: t.name, value: t.id })),
          label: "Looking For",
          hint: "Which level of education the company is looking for",
        },
        {
          type: "select",
          model: "business_area",
          items: this.tags
            .filter((t) => t.business_area)
            .map((t) => ({ text: t.name, value: t.id })),
          label: "Business areas",
          hint: "The companys business areas",
        },
        {
          type: "select",
          model: "offering",
          items: this.tags
            .filter((t) => t.offering)
            .map((t) => ({ text: t.name, value: t.id })),
          label: "offering",
          hint: "Which type of jobs the company is offering",
        },
      ];
    },
  },
  methods: {
    saveCompany(company) {
      this.$store.dispatch("companies/modifyCompany", company);
    },
    viewCompany(company) {
      this.$router.push("/company/" + company.name);
    },
    tagsFromIDs(tags) {
      return tags.map((id) => {
        return this.$store.getters["tags/getTagFromId"](id);
      });
    },
  },
};
</script>
