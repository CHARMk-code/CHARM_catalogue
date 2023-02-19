<template>
  <q-card>
    <q-card-section>
      <div class="text-h5">Graphics</div>
    </q-card-section>
    <q-card-section>
      <Table
        name="Layout"
        :table-columns="headers"
        :col-meta="colMeta"
        :rows="layoutsStore.layouts"
        :editable="true"
        @save-row="(l: Layout) => layoutsStore.modifyLayout(l)"
        @delete-row="(l: Layout) => layoutsStore.deleteLayout(l)"
      >
        <template #body-cell-Placement="props">
          <q-td :props="props">
            <template v-if="props.value == 0">On Company Page</template>
            <template v-else-if="props.value == 1">Left side</template>
            <template v-else-if="props.value == 2">Right side</template>
          </q-td>
        </template>

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
      </Table>
    </q-card-section>
  </q-card>
</template>

<script lang="ts" setup>
import Table from "@/components/table.vue";
import { useLayoutsStore } from "@/stores/modules/layouts";
import type { TableColMeta } from "./table_edit_dialog.vue";

const layoutsStore = useLayoutsStore();
const headers = [
  {
    name: "Placement",
    label: "Placement",
    field: "placement",
    align: "left",
    sortable: true,
  },
  {
    name: "Image",
    label: "Image",
    field: "image",
    align: "left",
    sortable: true,
  },
  {
    name: "Active",
    label: "Active",
    field: "active",
    align: "left",
    sortable: true,
  },
];

const colMeta: TableColMeta[] = [
  { type: "image", model: "image", label: "image" },
  { type: "checkbox", model: "active", label: "Active" },
  {
    type: "radio",
    model: "placement",
    label: "Placement",
    items: [
      { label: "Company page", value: 0 },
      { label: "Page Left", value: 1 },
      { label: "Page Right", value: 2 },
    ],
  },
];
</script>
