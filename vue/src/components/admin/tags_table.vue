<template>
  <v-card>
    <v-card-title>Tags</v-card-title>
    <v-card-text>
      <Table
        @saveRow="(t) => tagsStore.updateTag(t)"
        @deleteRow="(t) => tagsStore.removeTag(t)"
        name="Tags"
        :tableColumns="headers"
        :rows="Array.from(tagsStore.tags.values())"
        :colMeta="colMeta"
        :editable="true"
      >
        <template #col(icon)="{ item }">
          <TagComp :tag="item"></TagComp>
        </template>

        <template #col(categories)="{ item }">
          {{ categories(item) }}
        </template>
      </Table>
    </v-card-text>
  </v-card>
</template>

<script lang="ts" setup>
import Table from "@/components/table.vue";
import TagComp from "@/components/Tag.vue";
import axios from "@/plugins/axios";
import { useTagsStore, type Tag } from "@/stores/modules/tags";
import type { TableColMeta } from "./table_edit_dialog.vue";

const tagsStore = useTagsStore();

const base_URL = axios.defaults.baseURL + "/manage/image/";
function categories(tag: Tag) {
  const categories = [];
  if (tag.business_area) categories.push("Business Area");
  if (tag.division) categories.push("Program");
  if (tag.looking_for) categories.push("Looking for");
  if (tag.offering) categories.push("Offering");
  if (tag.language) categories.push("Language");
  return categories.join(", ");
}
const headers = [
  { name: "Icon", value: "icon" },
  { name: "Name", value: "name" },
  { name: "Categories", value: "categories" },
];

const colMeta: TableColMeta[] = [
  { type: "image", model: "icon", label: "tag icon" },
  { type: "text", model: "name", label: "Tag name" },
  { type: "checkbox", model: "business_area", label: "Business area" },
  { type: "checkbox", model: "division", label: "Division" },
  { type: "checkbox", model: "looking_for", label: "Looking for" },
  { type: "checkbox", model: "offering", label: "Offering" },
  { type: "checkbox", model: "language", label: "Language" },
];
</script>
