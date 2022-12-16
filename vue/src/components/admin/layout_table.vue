<template>
  <v-card>
    <v-card-title>Layouts</v-card-title>
    <v-card-text>
      <Table
        @saveRow="(l: Layout) => layoutsStore.modifyLayout(l)"
        @deleteRow="(l: Layout) => layoutsStore.deleteLayout(l)"
        name="Layout"
        :tableColumns="headers"
        :colMeta="colMeta"
        :rows="layoutsStore.layouts"
        :editable="true"
      >
        <template #col(placement)="{ value }">
          <template v-if="value == 0"> Company page</template>
          <template v-else-if="value == 1"> Page Left </template>
          <template v-else-if="value == 2"> Page Right </template>
        </template>

        <template #col(active)="{ value }">
          <v-icon v-if="value">mdi-eye</v-icon>
          <v-icon v-else>mdi-eye-off</v-icon>
        </template>
      </Table>
    </v-card-text>
  </v-card>
</template>

<script lang="ts" setup>
import Table from "@/components/table.vue";
import { useLayoutsStore } from "@/stores/modules/layouts";
import type { TableColMeta } from "./table_edit_dialog.vue";

const layoutsStore = useLayoutsStore();
const headers = [
  { name: "Placement", value: "placement" },
  { name: "Image", value: "image" },
  { name: "Active", value: "active" },
];

const colMeta: TableColMeta[] = [
  { type: "image", model: "image", label: "image" },
  { type: "checkbox", model: "active", label: "Active" },
  {
    type: "radio",
    model: "placement",
    label: "Placement",
    items: [
      { title: "Company page", value: 0 },
      { title: "Page Left", value: 1 },
      { title: "Page Right", value: 2 },
    ],
  },
];
</script>
