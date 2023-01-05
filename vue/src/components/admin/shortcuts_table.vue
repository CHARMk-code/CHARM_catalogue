<template>
  <q-card>
    <q-card-section class="text-h5">Shortcuts</q-card-section>
    <q-card-section>
      <Table
        @saveRow="(s) => shortcutsStore.modifyShortcut(s)"
        @deleteRow="(s) => shortcutsStore.deleteShortcut(s)"
        name="Shortcut"
        :tableColumns="columns"
        :colMeta="colMeta"
        :rows="shortcutsStore.shortcuts"
        :editable="true"
      >
        <template #body-cell-Icon="props">
          <q-td :props="props">
            <q-icon size="sm" :name="props.value"></q-icon>
          </q-td>
        </template>
      </Table>
    </q-card-section>
  </q-card>
</template>

<script lang="ts" setup>
import Table, { type TableRow } from "@/components/table.vue";
import type { Layout } from "@/stores/modules/layouts";
import { useShortcutsStore, type Shortcut } from "@/stores/modules/shortcuts";
import type { TableColMeta } from "./table_edit_dialog.vue";

const shortcutsStore = useShortcutsStore();
const columns = [
  { name: "Icon", label: "Icon", field: "icon", align: "left" },
  {
    name: "Title",
    label: "Title",
    field: "name",
    align: "left",
    sortable: true,
  },
  {
    name: "Description",
    label: "Description",
    field: "desc",
    align: "left",
    sortable: true,
  },
  { name: "Link", label: "Link", field: "link", align: "left", sortable: true },
];

const colMeta: TableColMeta[] = [
  { type: "text", model: "name", label: "Title" },
  { type: "text", model: "desc", label: "Description" },
  { type: "icon", model: "icon", label: "Icon" },
  { type: "text", model: "link", label: "Link" },
];
</script>
