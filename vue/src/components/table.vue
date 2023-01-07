<template>
  <q-table
    flat
    :rows="rows"
    :columns="editable || $slots.actions ? tableColumnsWAction : tableColumns"
    :filter="rowFilter"
    wrap-cells
  >
    <template #top-left>
      <q-input
        filled
        dense
        debounce="300"
        v-model="rowFilter"
        placeholder="Search"
      >
        <template v-slot:append>
          <q-icon name="search" />
        </template>
      </q-input>
    </template>
    <template #top-right>
      <q-btn
        class="q-ml-md"
        v-if="editable"
        color="primary"
        @click="createRow()"
      >
        Create new
      </q-btn>
    </template>
    <template #body-cell-Actions="props">
      <q-td style="white-space: normal !important">
        <div class="text-right">
          <slot name="actions" :row="props.value"></slot>
          <q-btn
            flat
            round
            size="sm"
            icon="mdi-pencil"
            @click="editRow(props.key - 1)"
          >
          </q-btn>
          <q-btn
            flat
            round
            size="sm"
            icon="mdi-delete"
            @click="deleteRow(props.key - 1)"
          >
          </q-btn>
        </div>
      </q-td>
    </template>

    <template v-for="(_, name) in $slots" v-slot:[name]="slotData">
      <slot v-if="!name.startsWith('edit-')" :name="name" v-bind="slotData" />
    </template>
  </q-table>
  <q-dialog full-width full-height v-model="editDialog">
    <tableEditDialog
      :name="name"
      v-model:row="clickedRow.row"
      :colMeta="colMeta"
      :newRow="newRow"
      :metaRow="clickedRow.meta"
      @saveRow="
        () => {
          editDialog = false;
          $emit('saveRow', clickedRow.row);
        }
      "
      :metaModelCallback="metaModelCallback"
    >
      <template v-for="(_, name) in $slots" v-slot:[name]="slotData">
        <slot v-if="name.startsWith('edit-')" :name="name" v-bind="slotData" />
      </template>
    </tableEditDialog>
  </q-dialog>
  <q-dialog v-model="deleteDialog">
    <q-card>
      <q-card-section class="row items-center">
        Are you sure you want to delete?
      </q-card-section>

      <q-card-actions :align="'right'">
        <q-btn flat label="Cancel" v-close-popup />
        <q-btn
          flat
          label="Delete"
          color="primary"
          @click="$emit('deleteRow', clickedRow.row)"
          v-close-popup
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts" setup>
import { ref, type Ref } from "vue";
import { computed, useSlots } from "vue";
import type { TableColMeta } from "@/components/admin/table_edit_dialog.vue";
import tableEditDialog from "@/components/admin/table_edit_dialog.vue";

export type TableRow = any;

defineEmits<{
  (e: "saveRow", updatedRow: TableRow): void;
  (e: "deleteRow", row: TableRow): void;
  (e: "clickRow", row: TableRow): void;
}>();

const props = defineProps<{
  name: string;
  editable: boolean;
  tableColumns: any[];
  rows: TableRow[];
  metaRows: any[];
  colMeta: TableColMeta[];
  metaModelCallback?: (meta: any, row: TableRow) => void;
}>();

const tableColumnsWAction = computed(() =>
  props.tableColumns.concat([
    {
      name: "Actions",
      label: "Actions",
      field: (row: any) => row,
      align: "right",
    },
  ])
);
const clickedRow: Ref<{ row: TableRow; meta: any }> = ref({
  row: {},
  meta: {},
});
const rowFilter = ref("");
const editDialog = ref(false);
const deleteDialog = ref(false);
const newRow = ref(false);

function editRow(index: number) {
  clickedRow.value.row = props.rows[index];
  clickedRow.value.meta = props.metaRows ? props.metaRows[index] : {};
  newRow.value = false;
  editDialog.value = true;
}

function deleteRow(index: number) {
  clickedRow.value.row = props.rows[index];
  clickedRow.value.meta = props.metaRows ? props.metaRows[index] : {};
  deleteDialog.value = true;
}

function createRow() {
  clickedRow.value = { row: {}, meta: {} };
  newRow.value = true;
  editDialog.value = true;
}

var hasActions = computed(() => useSlots().actions || props.editable);

function log(a: any) {
  console.log(a);
}
</script>
