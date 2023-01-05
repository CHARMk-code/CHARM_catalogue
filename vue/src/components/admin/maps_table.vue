<template>
  <q-card>
    <q-card-section class="text-h5">Maps</q-card-section>
    <q-card-section>
      <Table
        @saveEdit="(m) => mapsStore.updateMap(m)"
        @deleteRow="(m) => mapsStore.removeMap(m)"
        name="Map (Do NOT modify this table it will corrupt the data)"
        :tableColumns="headers"
        :rows="Array.from(mapsStore.maps.values())"
        :colMeta="colMeta"
        :editable="true"
      >
        <template #body-cell-Parent="props">
          <q-td :props="props">
            {{ (mapsStore.getMapFromId(props.value) || { name: "None" }).name }}
          </q-td>
        </template>
      </Table>
    </q-card-section>
  </q-card>
</template>

<script lang="ts" setup>
import Table from "@/components/table.vue";
import { useMapsStore } from "@/stores/modules/maps";
import type { TableColMeta } from "./table_edit_dialog.vue";

const mapsStore = useMapsStore();

const headers = [
  { name: "Name", label: "Name", field: "name", align: "left", sortable: true },
  {
    name: "Image",
    label: "Image",
    field: "image",
    align: "left",
    sortable: true,
  },
  {
    name: "Parent",
    label: "Parent",
    field: "ref",
    align: "left",
    sortable: true,
  },
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
