<template>
  <q-card>
    <q-card-section class="text-h5">Prepages</q-card-section>
    <q-card-section>
      <Table
        @saveRow="(p) => prepagesStore.modifyPrepage(p)"
        @deleteRow="(p) => prepagesStore.deletePrepage(p)"
        name="Prepage"
        :tableColumns="headers"
        :rows="prepagesStore.prepages"
        :colMeta="colMeta"
        :editable="true"
      >
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
        <!-- <template #body-cell-All="props">
          <q-td :props="props">
            {{ props.value }}
          </q-td>
        </template> -->
        <template #actions="{ row }">
          <q-btn
            v-if="row.active"
            round
            flat
            size="sm"
            icon="mdi-book-open"
            :to="'/prepage/' + row.order"
          ></q-btn>
        </template>
      </Table>
    </q-card-section>
  </q-card>
</template>

<script lang="ts" setup>
import Table from "@/components/table.vue";
import { usePrepagesStore } from "@/stores/modules/prepages";
import type { TableColMeta } from "./table_edit_dialog.vue";

const prepagesStore = usePrepagesStore();

const headers = [
  { name: "Name", label: "Name", field: "name", align: "left", sortable: true },
  {
    name: "Order",
    label: "Order",
    field: "order",
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
  // { name: "All", label: "All", field: (row: any) => row },
];
const colMeta: TableColMeta[] = [
  { type: "text", model: "name", label: "Name" },
  { type: "text", model: "order", label: "Order" },
  { type: "checkbox", model: "active", label: "Active" },
  { type: "image", model: "image", label: "page image" },
];
</script>
