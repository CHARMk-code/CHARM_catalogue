<template>
  <q-table
    v-model:pagination="tablePagination"
    flat
    :rows="rows"
    :columns="editable || $slots.actions ? tableColumnsWAction : tableColumns"
    :filter="rowFilter"
    wrap-cells
  >
    <template #top-left>
      <q-input
        v-model="rowFilter"
        filled
        dense
        debounce="300"
        placeholder="Search"
      >
        <template #append>
          <q-icon name="search" />
        </template>
      </q-input>
    </template>
    <template #top-right>
      <q-btn
        v-if="editable"
        class="q-ml-md"
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
            @click="editRow(props.row.id)"
          >
          </q-btn>
          <q-btn
            flat
            round
            size="sm"
            icon="mdi-delete"
            @click="deleteRow(props.row.id)"
          >
          </q-btn>
        </div>
      </q-td>
    </template>

    <template v-for="(_, name) in $slots" #[name]="slotData">
      <slot v-if="!name.startsWith('edit-')" :name="name" v-bind="slotData" />
    </template>
  </q-table>
  <q-dialog v-model="editDialog" full-width full-height>
    <tableEditDialog
      v-model:row="clickedRow.row"
      :name="name"
      :col-meta="colMeta"
      :new-row="newRow"
      :meta-row="clickedRow.meta"
      :meta-model-callback="metaModelCallback"
      @save-row="
        () => {
          editDialog = false;
          $emit('save-row', clickedRow.row);
        }
      "
    >
      <template v-for="(_, name) in $slots" #[name]="slotData">
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
        <q-btn v-close-popup flat label="Cancel" />
        <q-btn
          v-close-popup
          flat
          label="Delete"
          color="primary"
          @click="$emit('delete-row', clickedRow.row)"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts" setup>
import { ref, type Ref } from "vue";
import { computed } from "vue";
import type { TableColMeta } from "@/components/admin/table_edit_dialog.vue";
import tableEditDialog from "@/components/admin/table_edit_dialog.vue";
import { useSite_settingsStore } from "@/stores/modules/site_settings";

export type TableRow = any;

defineEmits<{
  (e: "save-row", updatedRow: TableRow): void;
  (e: "delete-row", row: TableRow): void;
  (e: "click-row", row: TableRow): void;
}>();

const props = defineProps<{
  name: string;
  editable: boolean;
  tableColumns: any[];
  rows: TableRow[];
  metaRows?: any[];
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

const siteSettingsStore = useSite_settingsStore();

const tablePagination = computed({
  get() {
    return siteSettingsStore.getTablePagination();
  },
  set(v) {
    siteSettingsStore.session_settings.tables.rowsPerPage = v.rowsPerPage;
  },
});

const clickedRow: Ref<{ row: TableRow; meta: any }> = ref({
  row: {},
  meta: {},
});
const rowFilter = ref("");
const editDialog = ref(false);
const deleteDialog = ref(false);
const newRow = ref(false);

function editRow(id: number) {
  const rowIndex = props.rows.findIndex((row) => row.id === id);
  clickedRow.value.row = props.rows[rowIndex]
  clickedRow.value.meta = props.metaRows ? props.metaRows[rowIndex] : {};
  newRow.value = false;
  editDialog.value = true;
}

function deleteRow(id: number) {
  const rowIndex = props.rows.findIndex((row) => row.id === id);
  clickedRow.value.row = props.rows[rowIndex];
  clickedRow.value.meta = props.metaRows ? props.metaRows[rowIndex] : {};
  deleteDialog.value = true;
}

function createRow() {
  clickedRow.value = { row: {}, meta: {} };
  newRow.value = true;
  editDialog.value = true;
}
</script>
