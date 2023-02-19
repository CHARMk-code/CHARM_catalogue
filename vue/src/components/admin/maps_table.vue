<template>
  <q-card>
    <q-card-section class="text-h5">Maps</q-card-section>
    <q-card-section>
      <Table
        name="Map"
        :table-columns="headers"
        :rows="rows"
        :meta-rows="metaRows"
        :col-meta="colMeta"
        :editable="true"
        :meta-model-callback="updateMapParent"
        @save-row="(m) => mapsStore.updateMap(m)"
        @delete-row="(m) => mapsStore.removeMap(m)"
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
import { useMapsStore, type Company_Map } from "@/stores/modules/maps";
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

const rows = Array.from(mapsStore.maps.values());
const metaRows = Array.from(mapsStore.maps.values()).map((map) => {
  const ref = mapsStore.getMapFromId(map.ref);
  const parent = ref
    ? { label: ref, value: ref.id }
    : { label: { name: "None" }, value: -1 };
  return {
    parent,
  };
});

function updateMapParent(
  meta: { parent: { label: Company_Map; value: number } },
  row: Company_Map
) {
  row.ref = meta.parent.value;
}

const colMeta: TableColMeta[] = [
  { type: "image", model: "image", label: "Map image" },
  {
    type: "single-select",
    model: "parent",
    label: "Parent",
    items: [{ label: { name: "None" }, value: -1 }].concat(
      Array.from(mapsStore.maps.values()).map((m) => ({
        label: m,
        value: m.id,
      }))
    ),
    meta: true,
  },
  { type: "text", model: "name", label: "Name" },
];
</script>
