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
    <tbody>
      <tr v-if="rows.length === 0">
        <td>No results</td>
      </tr>
      <tr v-for="row in rows" @click="$emit('clickRow', row)">
        <template v-for="column in tableColumns">
          <td>
            <slot
              :name="`col(${column.value})`"
              :value="row[column.value]"
              :item="row"
            >
              {{ row[column.value] }}
            </slot>
          </td>
        </template>
        <!-- actions -->
        <td v-if="hasActions" class="text-right">
          <template v-if="editable">
            <v-btn
              variant="plain"
              size="small"
              icon="mdi-pencil"
              @click="editRow(row)"
            >
            </v-btn>

            <v-dialog persistent v-model="show_popup" max-width="500px">
              <template v-slot:activator="{ props }">
                <v-btn
                  v-bind="props"
                  variant="plain"
                  size="small"
                  icon="mdi-delete"
                ></v-btn>
              </template>
              <tablePopup
                @closePopup="show_popup = false"
                @delete="
                  () => {
                    $emit('deleteRow', row);
                    show_popup = false;
                  }
                "
                :name="row.name ? row.name.toString() : props.name"
                :title="props.name"
              />
            </v-dialog>
          </template>
          <slot name="actions" :item="row"></slot>
        </td>
      </tr>
    </tbody>
  </v-table>

  <v-dialog v-if="editable" v-model="dialog" max-width="900px">
    <tableEditDialog
      @closeDialog="closeDialog()"
      @saveRow="$emit('saveRow', updatedRow)"
      :name="props.name"
      :row="updatedRow"
      :colMeta="colMeta"
      :newRow="false"
    ></tableEditDialog>
  </v-dialog>
</template>

<script lang="ts" setup>
// This needs to be fixed in order to pass as the types in the stores
export interface TableRow {
  [key: string]: string | number | boolean | Date | Set<number>;
}

import tableEditDialog from "@/components/admin/table_edit_dialog.vue";
import tablePopup from "@/components/admin/table_popup.vue";
import { computed, ref, useSlots, type Ref } from "vue";
import type { TableColMeta } from "@/components/admin/table_edit_dialog.vue";

export interface TableColumns {
  name: string; //display name
  value: string; //behind the scenes name
}

const props = defineProps<{
  name: string;
  editable: boolean;
  tableColumns: TableColumns[];
  rows: TableRow[];
  colMeta: TableColMeta[];
}>();

defineEmits<{
  (e: "saveRow", updatedRow: TableRow): void;
  (e: "deleteRow", row: TableRow): void;
}>();

var dialog: Ref<boolean> = ref(false);
var show_popup: Ref<boolean> = ref(false);
// var search = ""
var updatedRow: Ref<TableRow> = ref({});
var hasActions = computed(() => useSlots().actions || props.editable);

function editRow(row: TableRow) {
  updatedRow.value = row;
  dialog.value = true;
}

function closeDialog() {
  dialog.value = false;
  updatedRow.value = {};
}
</script>
