<template>
  <q-card>
    <q-card-section class="text-h5">Companies</q-card-section>
    <q-card-section>
      <Table
        @saveRow="(c) => companiesStore.updateCompany(c)"
        @deleteRow="(c) => companiesStore.removeCompany(c)"
        name="Companies"
        :tableColumns="headers"
        :rows="rows"
        :metaRows="metaRows"
        :colMeta="colMeta"
        :editable="true"
        :metaModelCallback="updateWithMetaModels"
      >
        <template #body-cell-Active="props">
          <q-td :props="props">
            <q-icon
              v-if="props.value"
              size="sm"
              color="primary"
              name="mdi-eye"
            />
            <q-icon
              v-if="!props.value"
              size="sm"
              color="grey"
              name="mdi-eye-off"
            />
          </q-td>
        </template>

        <template #body-cell-Programs="props">
          <q-td :props="props">
            <template
              v-if="
                !props.value ||
                tagsStore.getDivisionsFromIds(props.value).length < 1
              "
            >
              None
            </template>
            <template v-else>
              <TagGroup
                :tags="tagsStore.getDivisionsFromIds(props.value)"
              ></TagGroup>
            </template>
          </q-td>
        </template>
        <!-- <template #col(completion)="{ item }">
          {{ completed(item) }}
        </template> -->
        <template #actions="{ row }">
          <q-btn
            v-if="row.active"
            round
            flat
            size="sm"
            icon="mdi-book-open"
            :to="'/company/' + row.name"
          ></q-btn>
        </template>

        <!-- <template #edit-divisions="{ row, colMeta }">
          {{ log(row) }}
          <q-select
            filled
            multiple
            :model-value="tagsStore.getDivisionsFromIds(row.tags)"
            @update:model-value="
              (divisions) =>
                updateTagsFromSelect(
                  row,
                  new Set(tagsStore.getDivisionsFromIds(row.tags)),
                  new Set(divisions)
                )
            "
            :options="colMeta.items"
            :label="colMeta.label"
            :hint="colMeta.hint"
          >
            <template #option="{ opt, itemProps }">
              <q-item v-bind="itemProps">
                <q-item-section avatar v-if="opt.icon && opt.icon.length > 0">
                  <TagGroup :tags="[opt]"></TagGroup>
                </q-item-section>
                <q-item-section> {{ opt.name }}</q-item-section>
              </q-item>
            </template>

            <template #selected-item="{ index, opt }">
              <TagGroup :tags="[opt]"></TagGroup>
            </template>
          </q-select>
        </template> -->
      </Table>
    </q-card-section>
  </q-card>
</template>

<script lang="ts" setup>
import Table, { type TableRow } from "@/components/table.vue";
import TagGroup from "@/components/Tag_group.vue";
import axios from "@/plugins/axios";
import { computed, onMounted, reactive, ref, watch, type Ref } from "vue";
import dayjs from "dayjs";
import { useTagsStore, type Tag } from "@/stores/modules/tags";
import { useCompaniesStore, type Company } from "@/stores/modules/companies";
import { useMapsStore, type Company_Map } from "@/stores/modules/maps";
import { useRouter } from "vue-router";
import type { TableColMeta } from "./table_edit_dialog.vue";

const base_URL = axios.defaults.baseURL + "/manage/image/";

const headers = [
  { name: "Name", label: "Name", field: "name", align: "left", sortable: true },
  {
    name: "Booth",
    label: "Booth #",
    field: "booth_number",
    align: "left",
    sortable: true,
  },
  {
    name: "Programs",
    label: "Programs",
    field: (row) => row.tags,
    align: "left",
  },
  // { name: "Completion", label: "Completion", field: "completion", align: "left" },
  {
    name: "Active",
    label: "active",
    field: "active",
    align: "left",
    sortable: true,
  },
  {
    name: "Last Updated",
    label: "Last Updated",
    field: "last_updated",
    align: "left",
    sortable: true,
  },
];

const tagsStore = useTagsStore();
const companiesStore = useCompaniesStore();
const mapsStore = useMapsStore();

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

const rows = Array.from(companiesStore.companies.values());

type metaRow = {
  divisions: selectedTag[];
  looking_for: selectedTag[];
  business_areas: selectedTag[];
  offering: selectedTag[];
  language: selectedTag[];
  fair_area: selectedTag[];
  meta_map_image: { label: Company_Map; value: number };
};

const metaRows: metaRow[] = Array.from(companiesStore.companies.values()).map(
  (row) => {
    const map_image_obj = mapsStore.getMapFromId(row.map_image);
    const meta_map_image = map_image_obj
      ? { label: map_image_obj, value: map_image_obj.id }
      : { label: { name: "None" }, value: -1 };
    return reactive({
      meta_map_image,
      divisions: tagsStore
        .getDivisionsFromIds(row.tags)
        .map((t) => ({ value: t.id, label: t })),
      looking_for: tagsStore
        .getLookingForFromIds(row.tags)
        .map((t) => ({ value: t.id, label: t })),
      business_areas: tagsStore
        .getBusinessAreasFromIds(row.tags)
        .map((t) => ({ value: t.id, label: t })),
      offering: tagsStore
        .getOfferingsFromIds(row.tags)
        .map((t) => ({ value: t.id, label: t })),
      language: tagsStore
        .getLanguagesFromIds(row.tags)
        .map((t) => ({ value: t.id, label: t })),
      fair_area: tagsStore
        .getFairAreasFromIds(row.tags)
        .map((t) => ({ value: t.id, label: t })),
    });
  }
);

type selectedTag = {
  label: Tag;
  value: number;
};

function updateWithMetaModels(meta: metaRow, row: Company) {
  var allTags: number[] = [];
  if (
    meta.divisions ||
    meta.looking_for ||
    meta.business_areas ||
    meta.offering ||
    meta.language
  ) {
    if (meta.divisions) {
      var allTags = allTags.concat(meta.divisions.map((v) => v.value));
    }
    if (meta.looking_for) {
      var allTags = allTags.concat(meta.looking_for.map((v) => v.value));
    }
    if (meta.business_areas) {
      var allTags = allTags.concat(meta.business_areas.map((v) => v.value));
    }
    if (meta.offering) {
      var allTags = allTags.concat(meta.offering.map((v) => v.value));
    }
    if (meta.language) {
      var allTags = allTags.concat(meta.language.map((v) => v.value));
    }
    if (meta.fair_area) {
      var allTags = allTags.concat(meta.fair_area.map((v) => v.value));
    }
    row.tags = new Set(allTags);
  }
  row.map_image = meta.meta_map_image.value;
}

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
    type: "external-link",
    model: "summer_job_link",
    label: "Link to summer job application",
  },
  {
    type: "textarea",
    model: "talk_to_us_about",
    label: "Talk to us about",
  },
  { type: "text", model: "contacts", label: "Contacts" },
  {
    type: "text",
    model: "contact_email",
    label: "Contacts email",
  },
  { type: "external-link", model: "website", label: "Website" },
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
    model: "meta_map_image",
    items: [{ label: { name: "None" }, value: -1 }].concat(
      Array.from(mapsStore.maps.values()).map((m) => ({
        label: m,
        value: m.id,
      }))
    ),
    label: "Map",
    hint: "Map for company location",
    meta: true,
  },
  {
    type: "multiple-select",
    model: "divisions",
    items: tagsStore.divisions.map((t) => ({ value: t.id, label: t })),
    label: "Programs",
    hint: "Programs the company are interested in",
    meta: true,
  },
  {
    type: "multiple-select",
    model: "looking_for",
    items: tagsStore.looking_for.map((t) => ({ value: t.id, label: t })),
    label: "Looking For",
    hint: "Which level of education the company is looking for",
    meta: true,
  },
  {
    type: "multiple-select",
    model: "business_areas",
    items: tagsStore.business_areas.map((t) => ({ value: t.id, label: t })),
    label: "Business areas",
    hint: "The company's business areas",
    meta: true,
  },
  {
    type: "multiple-select",
    model: "offering",
    items: tagsStore.offering.map((t) => ({ value: t.id, label: t })),
    label: "Offering",
    hint: "Which type of jobs the company is offering",
    meta: true,
  },
  {
    type: "multiple-select",
    model: "language",
    items: tagsStore.languages.map((t) => ({ value: t.id, label: t })),
    label: "Languages",
    hint: "Which languages does the company want",
    meta: true,
  },
  {
    type: "multiple-select",
    model: "fair_area",
    items: tagsStore.fair_areas.map((t) => ({ value: t.id, label: t })),
    label: "Fair Area",
    hint: "Which Fair Area is the company on",
    meta: true,
  },
];
</script>
