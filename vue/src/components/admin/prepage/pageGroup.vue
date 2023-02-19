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
    <template v-if="pages.length === 0" #header>
      <div class="q-pa-lg full-width text-center text-italic">
        Empty page group
      </div>
    </template>

    <template #item="{ index, element }">
      <q-card square flat bordered class="col-6">
        <q-card-section>
          <Image :image-name="element.image" />
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
            v-model="element.mobile"
            class="position-center"
            size="sm"
            label="Visible on mobile"
          ></q-toggle>
          <q-space />
        </q-card-actions>
      </q-card>
    </template>
  </draggable>
  <q-dialog v-model="editDialog" full-width full-height>
    <tableEditDialog
      v-model:row="clickedRow"
      name="Prepage"
      :col-meta="colMeta"
      :new-row="false"
      @save-row="
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
        <q-btn v-close-popup flat label="Cancel" />
        <q-btn flat label="Delete" color="primary" @click="deletePrepage()" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script lang="ts" setup>
import { usePrepagesStore, type Prepage } from "@/stores/modules/prepages";
import { ref, type Ref } from "vue";
import draggable from "vuedraggable";
import type { TableColMeta } from "../table_edit_dialog.vue";
import tableEditDialog from "@/components/admin/table_edit_dialog.vue";
import Image from "@/components/utils/Image.vue";

const dragging = ref(false);

const clickedRow: Ref<Prepage | {}> = ref({});
const editDialog = ref(false);
const deleteDialog = ref(false);
const deleteIndex = ref(-1);

function onMoveCallback(evt) {
  if (evt.to.id === "inactivePages") return true;
  if (evt.from !== evt.to && evt.to.childElementCount >= 2) return false;
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

function deletePrepage() {
  prepagesStore.deletePrepage(clickedRow.value);
  deleteDialog.value = false;
}

function savePrepage(prepage: Prepage) {
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
