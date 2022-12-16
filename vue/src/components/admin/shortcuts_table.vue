<template>
  <v-card>
    <v-card-title>Shortcuts</v-card-title>
    <v-card-text>
      <Table
        @saveRow="(s) => shortcutsStore.modifyShortcut(s)"
        @deleteRow="(s) => shortcutsStore.deleteShortcut(s)"
        name="Shortcut"
        :tableColumns="headers"
        :colMeta="colMeta"
        :rows="shortcutsStore.shortcuts"
        :editable="true"
      >
      </Table>
    </v-card-text>
  </v-card>
</template>

<script lang="ts" setup>
import Table, { type TableRow } from "@/components/table.vue";
import type { Layout } from "@/stores/modules/layouts";
import { useShortcutsStore, type Shortcut } from "@/stores/modules/shortcuts";
import type { TableColMeta } from "./table_edit_dialog.vue";

const shortcutsStore = useShortcutsStore();
// shortcutsStore.modifyShortcut({"name": "test", "desc": "testarmycket", "link": "https://coollink.se", "icon": "mdi-search"})
const headers = [
  { name: "Id", value: "id" },
  { name: "Title", value: "name" },
  { name: "Icon", value: "icon" },
  { name: "Description", value: "desc" },
  { name: "Link", value: "link" },
];

const colMeta: TableColMeta[] = [
  { type: "text", model: "name", label: "Title" },
  { type: "text", model: "desc", label: "Description" },
  { type: "text", model: "icon", label: "Icon" },
  { type: "text", model: "link", label: "Link" },
];
</script>
