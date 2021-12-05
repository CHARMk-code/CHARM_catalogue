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
      <template v-slot:item.completion="{ item }">
        <template> {{ item.completion }}/11 </template>
      </template>
    </Table>
  </v-container>
</template>

<script>
import Table from "@/components/table";
import { mapGetters } from "vuex";
import Vue from "vue";
import dayjs from "dayjs";

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
        { text: "Completion", value: "completion", width: 120 },
        { text: "Active", value: "active", width: 100 },
        { text: "Last Updated", value: "last_updated", width: 170 },
        {
          text: "Actions",
          value: "actions",
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
      languages: "tags/languages",
      offerings: "tags/offers",
    }),
    modified_companies() {
      const companies = Array.from(this.companies);
      let modified = companies.map((c) => ({
        ...c,
        divisions: this.$store.getters["tags/getDivisionsFromIds"](c.tags),
        languages: this.$store.getters["tags/getLanguagesFromIds"](c.tags),
        looking_for: this.$store.getters["tags/getLookingForFromIds"](c.tags),
        offering: this.$store.getters["tags/getOffersFromIds"](c.tags),
        business_area: this.$store.getters["tags/getBusinessAreasFromIds"](
          c.tags
        ),
        last_updated: dayjs(c.last_updated).format("YYYY-MM-DD, HH:mm:ss"),
        tags: this.$store.getters["tags/getTagsFromIds"](c.tags),
      }));

      modified.forEach((c) => (c["completion"] = this.completionCompany(c)));
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
          type: "checkbox",
          model: "charmtalk",
          label: "On CHARMtalks",
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
        {
          type: "textarea",
          model: "talk_to_us_about",
          label: "Talk to us about",
        },
        { type: "number", model: "founded", label: "Founded" },
        { type: "text", model: "Contacts", label: "Contacts" },
        { type: "text", model: "website", label: "Website" },
        {
          type: "number",
          model: "employees_sweden",
          label: "Number of Employees in Sweden",
        },
        {
          type: "number",
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
          label: "Offering",
          hint: "Which type of jobs the company is offering",
        },
        {
          type: "select",
          model: "languages",
          items: this.languages,
          label: "Languages",
          hint: "Which languages does the company want",
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
    completionCompany(company) {
      let missing = 0;
      if (company.name == "") missing++;
      if (company.logo == "") missing++;
      if (company.tags == []) missing++;
      if (company.talk_to_us_about == "") missing++;
      if (company.trivia == "") missing++;
      if (company.website == "") missing++;
      if (company.founded == -1) missing++;
      if (company.employees_world == -1) missing++;
      if (company.employees_sweden == -1) missing++;
      if (company.description == "") missing++;
      if (company.contacts == "") missing++;

      return 11 - missing;
    },
  },
};
</script>
