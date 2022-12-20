<template>
  <v-card>
    <v-card-title>Companies</v-card-title>
    <v-card-text>
      <Table
        @saveRow="(c) => companiesStore.updateCompany(c)"
        @deleteRow="(c) => companiesStore.removeCompany(c)"
        name="Companies"
        :tableColumns="headers"
        :rows="Array.from(companiesStore.companies.values())"
        :colMeta="colMeta"
        :editable="true"
      >
        <template #col(active)="{ value }">
          <template v-if="value"><v-icon>mdi-eye</v-icon></template>
          <template v-else><v-icon>mdi-eye-off</v-icon></template>
        </template>

        <template #col(divisions)="{ item }">
          <template v-if="tagsStore.getDivisionsFromIds(item.tags).length < 1">
            None
          </template>
          <template
            v-else
            v-for="tag in tagsStore.getDivisionsFromIds(item.tags)"
          >
            <Tag :tag="tag"></Tag>
          </template>
        </template>

        <!-- <template #col(completion)="{ item }">
        {{ completed(item) }}
      </template> -->

        <template #actions="{ item }">
          <v-btn
            variant="plain"
            size="small"
            icon="mdi-book-open"
            @click="router.push('/company/' + item.name)"
          ></v-btn>
        </template>
      </Table>
    </v-card-text>
  </v-card>
</template>

<script lang="ts" setup>
import Table from "@/components/table.vue";
import Tag from "@/components/Tag.vue";
import axios from "@/plugins/axios";
import { computed } from "vue";
import dayjs from "dayjs";
import { useTagsStore } from "@/stores/modules/tags";
import { useCompaniesStore, type Company } from "@/stores/modules/companies";
import { useMapsStore } from "@/stores/modules/maps";
import { useRouter } from "vue-router";
import type { TableColMeta } from "./table_edit_dialog.vue";

const base_URL = axios.defaults.baseURL + "/manage/image/";

const headers = [
  { name: "Name", value: "name" },
  { name: "Booth", value: "booth_number" },
  { name: "Programs", value: "divisions" },
  // { name: "Completion", value: "completion" },
  { name: "Active", value: "active" },
  { name: "Last Updated", value: "last_updated" },
];

const tagsStore = useTagsStore();
const companiesStore = useCompaniesStore();
const mapsStore = useMapsStore();

const router = useRouter();

// const hasNonValidValue = (value: any): boolean => {
//   return value === null || value === "" || value.length === 0 || value === -1;
// };

// const completed = (company: Company) => {
//   let missing = 0;
//   let total = 0;
//   for (const key in Object.keys(company)) {
//     if (hasNonValidValue(company[key])) {
//       missing += 1;
//     }
//     total += 1;
//   }
//   return total - missing + "/" + total;
// };

const colMeta: TableColMeta[] = [
  {
    type: "checkbox",
    model: "active",
    onIcon: "mdi-eye",
    offIcon: "mdi-eye-off",
    label: "Active (required for company to be visible)",
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
  },
  {
    type: "number",
    model: "booth_number",
    label: "Booth number",
  },
  {
    type: "image",
    model: "logo",
    label: "Company Logo",
  },
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
    type: "single-select",
    model: "map_image",
    items: Array.from(mapsStore.maps)
      .map(([_, m]) => {
        return { title: m.name, value: m.id };
      })
      .concat([{ title: "No Goto", value: -1 }]),
    label: "Map",
    hint: "Map for company location",
  },
  {
    type: "multiple-select",
    model: "divisions",
    items: tagsStore.divisions.map((t) => {
      return { title: t.name, value: t.id };
    }),
    label: "Divisions",
    hint: "Programs the company are interested in",
  },
  {
    type: "multiple-select",
    model: "looking_for",
    items: tagsStore.looking_for.map((t) => {
      return { title: t.name, value: t.id };
    }),
    label: "Looking For",
    hint: "Which level of education the company is looking for",
  },
  {
    type: "multiple-select",
    model: "business_area",
    items: tagsStore.business_areas.map((t) => {
      return { title: t.name, value: t.id };
    }),
    label: "Business areas",
    hint: "The companys' business areas",
  },
  {
    type: "multiple-select",
    model: "offering",
    items: tagsStore.offering.map((t) => {
      return { title: t.name, value: t.id };
    }),
    label: "Offering",
    hint: "Which type of jobs the company is offering",
  },
  {
    type: "multiple-select",
    model: "languages",
    items: tagsStore.languages.map((t) => {
      return { title: t.name, value: t.id };
    }),
    label: "Languages",
    hint: "Which languages does the company want",
  },
];
</script>
