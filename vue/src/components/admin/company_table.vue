<template>
  <v-container>
    <Table
      @save_edit="saveCompany"
      @delete_row="deleteCompany"
      name="Companies"
      :headers="headers"
      :data="companiesStore.companies"
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
        <template> {{ completed(item) }} </template>
      </template>
    </Table>
  </v-container>
</template>

<script lang="ts" setup>
import Table from "@/components/table.vue";
import axios from "@/plugins/axios"
import { computed } from "vue";
import dayjs from "dayjs";
import { useTagsStore } from "@/stores/modules/tags";
import { useCompaniesStore, type Company } from "@/stores/modules/companies";
import { useMapsStore } from "@/stores/modules/maps";
import { useRouter } from "vue-router";

const base_URL = axios.defaults.baseURL + "/manage/image/"

const headers = [
  { text: "Name", value: "name" },
  { text: "Booth", value: "booth_number" },
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
]

const tagsStore = useTagsStore()
const companiesStore = useCompaniesStore()
const mapsStore = useMapsStore()

// const modified_companies = computed(() => {
//   const companies = Array.from(companiesStore.companies.values());
//   return companies.map((c) => ({
//     ...c,
//     divisions: tagsStore.getDivisionsFromIds(c.tags),
//     languages: tagsStore.getLanguagesFromIds(c.tags),
//     looking_for: tagsStore.getLookingForFromIds(c.tags),
//     offering: tagsStore.getOfferingsFromIds(c.tags),
//     business_area: tagsStore.getBusinessAreasFromIds(c.tags),
//     last_updated: dayjs(c.last_updated).format("YYYY-MM-DD, HH:mm:ss"),
//     tags: tagsStore.getTagsFromIds(c.tags),
//     map_image: mapsStore.getMapFromId(c.map_image)
//   }) )
// });

const saveCompany = (company: Company) => {
  companiesStore.updateCompany(company)
}
const deleteCompany = (company: Company) => {
  companiesStore.removeCompany(company);
}
const viewCompany = (company: Company) => {
  useRouter().push("/company/" + company.name);
}

const hasNonValidValue = (value: any): boolean => {
  return value === null
    || value === ""
    || value.length === 0
    || value === -1
}


const completed = (company: Company) => {
  let missing = 0;
  let total = 0;
  for (const key in Object.keys(company)) {
    if (hasNonValidValue(company[key])) {
      missing += 1;
    }
    total += 1;
  }

  return total - missing + "/" + total;
},
const row_meta = computed(() => [
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
  {
    type: "number",
    model: "booth_number",
    label: "Booth number"
  },
  { type: "image", model: "logo", label: "Company Logo" },
  {
    type: "textarea",
    model: "description",
    label: "Company description",
  },
  {
    type: "textarea",
    model: "unique_selling_point",
    label: "What is the unique selling points",
  },
  {
    type: "textarea",
    model: "summer_job_description",
    label: "Description of summer job",
  },
  {
    type: "text",
    model: "summer_job_link",
    label: "Link to summer job application",
  },
  {
    type: "textarea",
    model: "talk_to_us_about",
    label: "Talk to us about",
  },
  { type: "text", model: "contacts", label: "Contacts" },
  { type: "text", model: "contact_email", label: "Contacts email" },
  { type: "text", model: "website", label: "Website" },
  {
    type: "number",
    model: "employees_world",
    label: "Number of Employees in the whole world",
  },
  {
    type: "number",
    model: "employees_sweden",
    label: "Number of Employees in Sweden",
  },
  {
    type: "single_select",
    model: "map_image",
    items: Array.from(mapsStore.maps.values()).concat([{ name: "No Map", id: -1, image: "", ref: -1 }]),
    label: "Map",
    hint: "Map for company location",
  },
  {
    type: "select",
    model: "divisions",
    items: tagsStore.divisions,
    label: "Divisions",
    hint: "Programs the company are interested in",
  },
  {
    type: "select",
    model: "looking_for",
    items: tagsStore.looking_for,
    label: "Looking For",
    hint: "Which level of education the company is looking for",
  },
  {
    type: "select",
    model: "business_area",
    items: tagsStore.business_areas,
    label: "Business areas",
    hint: "The companys' business areas",
  },
  {
    type: "select",
    model: "offering",
    items: tagsStore.offering,
    label: "Offering",
    hint: "Which type of jobs the company is offering",
  },
  {
    type: "select",
    model: "languages",
    items: tagsStore.languages,
    label: "Languages",
    hint: "Which languages does the company want",
  },
]);
</script>
