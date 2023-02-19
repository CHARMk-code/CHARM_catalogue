<template>
  <tbody>
    <tr
      v-for="(row, row_index) in rows"
      :key="row_index"
      @click="$emit('click-row', row)"
    >
      <template v-for="(column, col_index) in tableColumns" :key="col_index">
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

          <v-dialog v-model="show_popup" persistent max-width="500px">
            <template #activator="{ props }">
              <v-btn
                v-bind="props"
                variant="plain"
                size="small"
                icon="mdi-delete"
              ></v-btn>
            </template>
            <tablePopup
              :name="row.name ? row.name.toString() : props.name"
              :title="props.name"
              @close-popup="show_popup = false"
              @delete="
                () => {
                  $emit('delete-row', row);
                  show_popup = false;
                }
              "
            />
          </v-dialog>
        </template>
        <slot name="actions" :item="row"></slot>
      </td>
    </tr>
  </tbody>
  <v-dialog v-if="editable" v-model="dialog" max-width="900px">
    <tableEditDialog
      :name="props.name"
      :row="updatedRow"
      :col-meta="colMeta"
      :new-row="false"
      @close-dialog="closeDialog()"
      @save-row="$emit('save-row', updatedRow)"
    ></tableEditDialog>
  </v-dialog>
</template>

<script lang="ts" setup>
import tableEditDialog from "@/components/admin/table_edit_dialog.vue";
import tablePopup from "@/components/admin/table_popup.vue";
import { computed, ref, useSlots, type Ref } from "vue";
import type { TableColMeta } from "./admin/table_edit_dialog.vue";
import type { TableRow } from "./table.vue";

const props = defineProps<{
  name: string;
  editable: boolean;
  tableColumns: any[];
  rows: Iterable<TableRow>;
  colMeta: TableColMeta[];
}>();

defineEmits<{
  (e: "save-row", updatedRow: TableRow): void;
  (e: "delete-row", row: TableRow): void;
  (e: "click-row", row: TableRow): void;
}>();

const dialog: Ref<boolean> = ref(false);
const show_popup: Ref<boolean> = ref(false);
// var search = ""
const updatedRow: Ref<TableRow> = ref({});
const hasActions = computed(() => useSlots().actions || props.editable);

function editRow(row: TableRow) {
  updatedRow.value = row;
  dialog.value = true;
}

function closeDialog() {
  dialog.value = false;
  updatedRow.value = {};
}
</script>
