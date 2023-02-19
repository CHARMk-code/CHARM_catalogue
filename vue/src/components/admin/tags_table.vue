<template>
  <q-card>
    <q-card-section class="text-h5">Tags</q-card-section>
    <q-card-section>
      <Table
        name="Tags"
        :table-columns="headers"
        :rows="Array.from(tagsStore.tags.values())"
        :col-meta="colMeta"
        :editable="true"
        @save-row="(t) => tagsStore.updateTag(t)"
        @delete-row="(t) => tagsStore.removeTag(t)"
      >
        <template #body-cell-Icon="props">
          <q-td :props="props">
            <Tags :tags="[props.value]"></Tags>
          </q-td>
        </template>

        <template #body-cell-Categories="props">
          <q-td :props="props">
            {{ props.value }}
          </q-td>
        </template>
      </Table>
    </q-card-section>
  </q-card>
</template>

<script lang="ts" setup>
import Table from "@/components/table.vue";
import Tags from "@/components/Tag_group.vue";
import { useTagsStore, type Tag } from "@/stores/modules/tags";
import type { TableColMeta } from "./table_edit_dialog.vue";

const tagsStore = useTagsStore();

const headers = [
  { name: "Icon", label: "Icon", field: (row: any) => row, align: "left" },
  { name: "Name", label: "Name", field: "name", align: "left", sortable: true },
  {
    name: "Categories",
    label: "Categories",
    field: (row: Tag) => {
      const categories = [];
      if (row.business_area) categories.push("Business Area");
      if (row.division) categories.push("Program");
      if (row.looking_for) categories.push("Looking for");
      if (row.offering) categories.push("Offering");
      if (row.language) categories.push("Language");
      if (row.fair_area) categories.push("Fair Area");
      return categories.join(", ");
    },
    align: "left",
    sortable: true,
  },
];

const colMeta: TableColMeta[] = [
  { type: "image", model: "icon", label: "tag icon" },
  { type: "text", model: "name", label: "Tag name" },
  { type: "checkbox", model: "business_area", label: "Business area" },
  { type: "checkbox", model: "division", label: "Division" },
  { type: "checkbox", model: "looking_for", label: "Looking for" },
  { type: "checkbox", model: "offering", label: "Offering" },
  { type: "checkbox", model: "language", label: "Language" },
  { type: "checkbox", model: "fair_area", label: "Fair Area" },
];
</script>
