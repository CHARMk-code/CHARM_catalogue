<template>
  <div class="row">
    <q-space></q-space>
    <q-card class="col-8">
      <q-card-section>
        <span class="text-h5"> Prepages </span>
        <q-card-actions>
          <q-space />
          <q-btn
            color="primary"
            label="Save Prepage Order"
            @click="saveOrder()"
          ></q-btn>
          <q-space />
        </q-card-actions>
        <draggable :list="pageGroups" item-key="id" group="pageGroup">
          <template #item="{ index, element }">
            <div>
              <div class="text-h6 q-my-md text-center">
                Page {{ index + 1 }}
              </div>
              <pageGroupDraggable :pages="element.pages"></pageGroupDraggable>
            </div>
          </template>
          <template #footer>
            <div class="row q-pa-md">
              <q-space />
              <q-btn
                label="Add empty page group"
                @click="addPageGroup()"
              ></q-btn>
              <q-space />
            </div>
          </template>
        </draggable>
        <q-card-actions>
          <q-space />
          <q-btn
            color="primary"
            label="Save Prepage Order"
            @click="saveOrder()"
          ></q-btn>
          <q-space />
        </q-card-actions>
      </q-card-section>
    </q-card>
    <q-space />
    <div class="col-3"></div>
    <q-card class="fixed-right col-3">
      <q-card-section class="q-mt-lg"></q-card-section>
      <q-card-section>
        <div class="text-h6 text-center">Inactive prepages</div>
      </q-card-section>
      <q-card-section>
        <div class="text-center q-pa-sm">
          Drop active pages here to inactivate them
        </div>
        <draggable
          id="inactivePages"
          style="min-height: 200px"
          class="row q-col-gutter-sm q-pr-sm q-pb-sm dragArea rounded-borders"
          :move="onMoveCallbackInactivePrepages"
          :list="inactivePages"
          item-key="id"
          group="page"
        >
          <template #item="{ element }">
            <div class="col-6">
              <q-card>
                <Image :image-name="element.image" />
              </q-card>
            </div>
          </template>
        </draggable>
        <div class="row q-mt-md">
          <q-space />
          <q-btn label="Create new Prepage" @click="editDialog = true"></q-btn>
          <q-space />
        </div>
      </q-card-section>
    </q-card>
    <q-space />
  </div>
  <q-dialog v-model="editDialog" full-width full-height>
    <tableEditDialog
      v-model:row="newPrepage"
      name="Prepage"
      :col-meta="colMeta"
      :new-row="true"
      @save-row="
        (rawRow) => {
          editDialog = false;
          createPrepage(rawRow);
        }
      "
    >
    </tableEditDialog>
  </q-dialog>
</template>

<script lang="ts" setup>
import {
  usePrepagesStore,
  type Prepage,
  type PrepageGroup,
} from "@/stores/modules/prepages";
import { ref, type Ref } from "vue";
import draggable from "vuedraggable";
import pageGroupDraggable from "./pageGroup.vue";
import tableEditDialog, {
  type TableColMeta,
} from "@/components/admin/table_edit_dialog.vue";
import Image from "@/components/utils/Image.vue";

const prepagesStore = usePrepagesStore();

function onMoveCallbackInactivePrepages(evt: any) {
  if (evt.to.id === "inactivePages") return true;
  if (evt.from !== evt.to && evt.to.childElementCount >= 2) {
    return false;
  }
}

const editDialog = ref(false);
const newPrepage = {
  name: "",
  image: undefined,
  active: false,
  mobile: true,
  side: "left",
  page: 0,
};

const colMeta: TableColMeta[] = [
  { type: "text", model: "name", label: "Name" },
  { type: "image", model: "image", label: "Page image" },
];

const pageGroups: Ref<PrepageGroup[]> = ref(
  Object.values(prepagesStore.pageGroups)
);
const inactivePages: Prepage[] = prepagesStore.inactivePrepages;

function createPrepage(prepage: Prepage) {
  const prepagesStore = usePrepagesStore();
  prepagesStore.modifyPrepage(prepage);
}

function addPageGroup() {
  prepagesStore.pageGroups[pageGroups.value.length + 1] = {
    id: pageGroups.value.length + 1,
    pages: [],
  };
  pageGroups.value = Object.values(prepagesStore.pageGroups);
}

function saveOrder() {
  pageGroups.value.forEach((pageGroup, pageIndex) => {
    pageGroup.pages.forEach((prepage, index) => {
      prepage.side = index === 0 ? "left" : "right";
      prepage.page = pageIndex + 1;
      prepage.active = true;

      prepagesStore.modifyPrepage(prepage);
    });
  });
  inactivePages.forEach((prepage) => {
    prepage.active = false;
    prepagesStore.modifyPrepage(prepage);
  });
}
</script>

<style lang="sass" scoped>
.dragArea
  background-color: $grey-3
</style>
