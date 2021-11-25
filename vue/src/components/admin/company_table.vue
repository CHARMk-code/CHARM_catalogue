<template>
  <v-container>
    <Table
      @save_edit="saveCompany"
      @delete_row="deleteCompany"
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

      <template v-slot:item.divisions="{ item }">
        <template v-for="tag in item.divisions">
          <template v-if="tag.icon != ''">
            <v-avatar size="24px" class="ma-1" x-small :key="tag.id">
              <v-img
                max-height="32px"
                max-width="32px"
                :src="base_URL + tag.icon"
              />
            </v-avatar>
          </template>
          <template v-else>
            <v-chip small :key="tag.id">
              {{ item.name }}
            </v-chip>
          </template>
        </template>
      </template>
      <template v-slot:item.looking_for="{ item }">
        <template v-for="tag in item.looking_for">
          <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
            tag.name
          }}</v-chip>
        </template>
      </template>
      <template v-slot:item.offering="{ item }">
        <template v-for="tag in item.offering">
          <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
            tag.name
          }}</v-chip>
        </template>
      </template>
      <template v-slot:item.business_area="{ item }">
        <template v-for="tag in item.business_area">
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
import Vue from "vue";
export default {
  name: "companies_table",
  components: {
    Table,
  },
  data() {
    return {
      headers: [
        { text: "Name", value: "name" },
        { text: "Programs", value: "divisions" },
        { text: "Business areas", value: "business_area" },
        { text: "Looking for", value: "looking_for" },
        { text: "offering", value: "offering" },
        { text: "Active", value: "active", align: "center", width: 110 },
        {
          text: "Actions",
          value: "actions",
          align: "center",
          width: 130,
          sortable: false,
        },
      ],
    };
  },
  computed: {
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },
    ...mapGetters({
      companies: "companies/companies",
      tags: "tags/tags",
      divisions: "tags/divisions",
      looking_for: "tags/looking_fors",
      business_areas: "tags/business_areas",
      offerings: "tags/offers",
    }),
    modified_companies() {
      const companies = Array.from(this.companies);
      const modified = companies.map((c) => ({
        ...c,
        divisions: this.$store.getters["tags/getDivisionsFromIds"](c.tags),
        looking_for: this.$store.getters["tags/getLookingForFromIds"](c.tags),
        offering: this.$store.getters["tags/getOffersFromIds"](c.tags),
        business_area: this.$store.getters["tags/getBusinessAreasFromIds"](
          c.tags
        ),
        tags: this.$store.getters["tags/getTagsFromIds"](c.tags),
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
        {
          type: "text",
          model: "name",
          label: "Company name",
          displayname: true,
        },
        { type: "image", model: "logo", label: "Company Logo" },
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
          items: this.divisions,
          label: "Divisions",
          hint: "Programs the company are interested in",
        },
        {
          type: "select",
          model: "looking_for",
          items: this.looking_for,
          label: "Looking For",
          hint: "Which level of education the company is looking for",
        },
        {
          type: "select",
          model: "business_area",
          items: this.business_areas,
          label: "Business areas",
          hint: "The companys' business areas",
        },
        {
          type: "select",
          model: "offering",
          items: this.offerings,
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
    deleteCompany(company) {
      this.$store.dispatch("companies/deleteCompany", company);
    },
    viewCompany(company) {
      this.$router.push("/company/" + company.name);
    },
  },
};
</script>
