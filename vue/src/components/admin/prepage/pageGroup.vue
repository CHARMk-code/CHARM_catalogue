<template>
  <draggable
    class="row dragArea rounded-borders justify-center"
    :list="pages"
    item-key="id"
    group="page"
    :move="onMoveCallback"
    @start="dragging = true"
    @end="dragging = false"
  >
    <template #header v-if="pages.length === 0">
      <div class="q-pa-lg full-width text-center text-italic">
        Empty page group
      </div>
    </template>

    <template #item="{ index, element }">
      <q-card square flat bordered class="col-6">
        <q-card-section>
          <q-img :src="base_URL + element.image"></q-img>
        </q-card-section>
        <q-card-actions>
          <q-space />
          <q-btn
            flat
            round
            size="sm"
            icon="mdi-pencil"
            @click="editPrepage(index)"
          >
          </q-btn>
          <q-btn
            flat
            round
            size="sm"
            icon="mdi-delete"
            @click="preDeletePrepage(index)"
          >
          </q-btn>
          <q-toggle
            class="position-center"
            size="sm"
            label="Visible on mobile"
            v-model="element.mobile"
          ></q-toggle>
          <q-space />
        </q-card-actions>
      </q-card>
    </template>
  </draggable>
  <q-dialog full-width full-height v-model="editDialog">
    <tableEditDialog
      name="Prepage"
      v-model:row="clickedRow"
      :colMeta="colMeta"
      :newRow="false"
      @saveRow="
        (rawRow) => {
          editDialog = false;
          savePrepage(rawRow);
        }
      "
    >
    </tableEditDialog>
  </q-dialog>
  <q-dialog v-model="deleteDialog">
    <q-card>
      <q-card-section class="row items-center">
        Are you sure you want to delete?
      </q-card-section>

      <q-card-actions :align="'right'">
        <q-btn flat label="Cancel" v-close-popup />
        <q-btn flat label="Delete" color="primary" @click="deletePrepage()" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
import { usePrepagesStore, type Prepage } from "@/stores/modules/prepages";
import { computed, ref, type Ref } from "vue";
import draggable from "vuedraggable";
import type { TableColMeta } from "../table_edit_dialog.vue";
import tableEditDialog from "@/components/admin/table_edit_dialog.vue";
import type { TableRow } from "@/components/table.vue";

const base_URL = axios.defaults.baseURL + "/manage/image/";

const headerHover = ref(false);
const dragging = ref(false);
const showEmptyPlaceholder = computed(() => {
  !(headerHover.value && dragging);
});

const clickedRow: Ref<Prepage | {}> = ref({});
const editDialog = ref(false);
const deleteDialog = ref(false);
const deleteIndex = ref(-1);

function onMoveCallback(evt) {
  if (evt.to.id === "inactivePages") return true;
  if (evt.from !== evt.to && evt.to.childElementCount >= 2) {
    console.log("Too big");
    return false;
  }
}
const props = defineProps<{
  pages: Prepage[];
}>();

const prepagesStore = usePrepagesStore();

function editPrepage(index: number) {
  clickedRow.value = props.pages[index];
  editDialog.value = true;
}
function preDeletePrepage(index: number) {
  clickedRow.value = props.pages[index];
  deleteIndex.value = index;
  deleteDialog.value = true;
}

function deletePrepage(index: number) {
  prepagesStore.deletePrepage(clickedRow.value);
  deleteDialog.value = false;
}

function savePrepage(prepage: Prepage) {
  console.log("saving");
  const prepagesStore = usePrepagesStore();
  prepagesStore.modifyPrepage(prepage);
}

const colMeta: TableColMeta[] = [
  { type: "text", model: "name", label: "Name" },
  { type: "image", model: "image", label: "Page image" },
];
</script>

<style lang="sass" scoped>
.dragArea
  background-color: $grey-3
</style>
