<template>
  <v-table>
    <thead>
      <tr>
        <th v-for="column in tableColumns" class="text-left">
          {{ column.name }}
        </th>
        <th v-if="hasActions" class="text-right">Actions</th>
      </tr>
    </thead>
    <suspense>
      <template #fallback>
        <tr>
          <td>
            <v-card flat>
              <v-card-text>Waiting for Data...</v-card-text>
            </v-card>
          </td>
        </tr>
      </template>

      <tableBody
        :name="name"
        :editable="editable"
        :tableColumns="tableColumns"
        :rows="rows"
        :colMeta="colMeta"
      >
        <template v-for="(_, name) in $slots" v-slot:[name]="slotData">
          <slot :name="name" v-bind="slotData" />
        </template>
      </tableBody>
    </suspense>
  </v-table>
</template>

<script lang="ts" setup>
import { defineAsyncComponent } from "vue";
import { computed, useSlots } from "vue";
import type { TableColMeta } from "@/components/admin/table_edit_dialog.vue";

export type TableRow = any;

export interface TableColumns {
  name: string; //display name
  value: string; //behind the scenes name
}

const tableBody = defineAsyncComponent(
  () => import("@/components/table_body.vue")
);

defineEmits<{
  (e: "saveRow", updatedRow: TableRow): void;
  (e: "deleteRow", row: TableRow): void;
  (e: "clickRow", row: TableRow): void;
}>();

const props = defineProps<{
  name: string;
  editable: boolean;
  tableColumns: TableColumns[];
  rows: Iterable<TableRow>;
  colMeta: TableColMeta[];
}>();

var hasActions = computed(() => useSlots().actions || props.editable);

function log(a: any) {
  console.log(a);
}
</script>
