<template>
  <v-card>
    <v-card-title>Maps</v-card-title>
    <v-card-text>
      <Table
        @saveEdit="(m) => mapsStore.updateMap(m)"
        @deleteRow="(m) => mapsStore.removeMap(m)"
        name="Map (Do NOT modify this table it will corrupt the data)"
        :tableColumns="headers"
        :rows="Array.from(mapsStore.maps.values())"
        :colMeta="colMeta"
        :editable="true"
      >
        <template #col(ref)="{ value }">
          {{ (mapsStore.getMapFromId(value) || { name: "None" }).name }}
        </template>
      </Table>
    </v-card-text>
  </v-card>
</template>

<script lang="ts" setup>
import Table from "@/components/table.vue";
import { useMapsStore } from "@/stores/modules/maps";
import type { TableColMeta } from "./table_edit_dialog.vue";

const mapsStore = useMapsStore();

const headers = [
  { name: "Name", value: "name" },
  { name: "Image", value: "image" },
  { name: "Parent", value: "ref" },
];

const colMeta: TableColMeta[] = [
  { type: "image", model: "image", label: "Map image" },
  {
    type: "single-select",
    model: "ref",
    label: "Parent",
    items: Array.from(mapsStore.maps)
      .map(([_, m]) => {
        return { title: m.name, value: m.id };
      })
      .concat([{ title: "No Goto", value: -1 }]),
  },
  { type: "text", model: "name", label: "Name" },
];
</script>
