<template>
  <v-container>
    <Table
      @saveRow="(t) => tagsStore.modifyTag(t)"
      @deleteRow="(t) => tagsStore.deleteTag(t)"
      name="Tags"
      :tableColumns="headers"
      :rows="Array.from(tagsStore.tags)"
      :colMeta="colMeta"
      :editable="true"
    >
      <template v-slot:item.business_area="{ item }">
        <v-simple-checkbox disabled v-model="item.business_area" />
      </template>
      <template v-slot:item.division="{ item }">
        <v-simple-checkbox disabled v-model="item.division" />
      </template>
      <template v-slot:item.looking_for="{ item }">
        <v-simple-checkbox disabled v-model="item.looking_for" />
      </template>
      <template v-slot:item.offering="{ item }">
        <v-simple-checkbox disabled v-model="item.offering" />
      </template>
      <template v-slot:item.language="{ item }">
        <v-simple-checkbox disabled v-model="item.language" />
      </template>

      <template v-slot:item.icon="{ item }">
        <v-avatar>
          <v-img
            max-height="36px"
            max-width="36px"
            :src="base_URL + item.icon"
          />
        </v-avatar>
      </template>
    </Table>
  </v-container>
</template>

<script lang="ts" setup>
import Table from "@/components/table.vue";
import axios from "@/plugins/axios";
import { useTagsStore } from "@/stores/modules/tags";
import type { TableColMeta } from "./table_edit_dialog.vue";

const tagsStore = useTagsStore();

const base_URL = axios.defaults.baseURL + "/manage/image/";

const headers = [
  { name: "Icon", value: "icon" },
  { name: "Name", value: "name" },
  { name: "Business Area", value: "business_area" },
  { name: "Division", value: "division" },
  { name: "Looking for", value: "looking_for" },
  { name: "Offering", value: "offering" },
  { name: "Language", value: "language" },
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
