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
            @click="saveOrder()"
            label="Save Prepage Order"
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
                @click="addPageGroup()"
                label="Add empty page group"
              ></q-btn>
              <q-space />
            </div>
          </template>
        </draggable>
        <q-card-actions>
          <q-space />
          <q-btn
            color="primary"
            @click="saveOrder()"
            label="Save Prepage Order"
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
          style="min-height: 200px"
          class="row q-col-gutter-sm q-pr-sm q-pb-sm dragArea rounded-borders"
          id="inactivePages"
          :move="onMoveCallbackInactivePrepages"
          :list="inactivePages"
          item-key="id"
          group="page"
        >
          <template #item="{ element }">
            <div class="col-6">
              <q-card>
                <Image :imageName="element.image" />
              </q-card>
            </div>
          </template>
        </draggable>
        <div class="row q-mt-md">
          <q-space />
          <q-btn @click="editDialog = true" label="Create new Prepage"></q-btn>
          <q-space />
        </div>
      </q-card-section>
    </q-card>
    <q-space />
  </div>
  <q-dialog full-width full-height v-model="editDialog">
    <tableEditDialog
      name="Prepage"
      v-model:row="newPrepage"
      :colMeta="colMeta"
      :newRow="true"
      @saveRow="
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
import { computed, reactive, readonly, ref, type Ref } from "vue";
import draggable from "vuedraggable";
import pageGroupDraggable from "./pageGroup.vue";
import axios from "@/plugins/axios";
import tableEditDialog from "@/components/admin/table_edit_dialog.vue";
import Image from "@/components/utils/Image.vue";

const prepagesStore = usePrepagesStore();

function onMoveCallbackInactivePrepages(evt) {
  if (evt.to.id === "inactivePages") return true;
  if (evt.from !== evt.to && evt.to.childElementCount >= 2) {
    console.log("Too big");
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
  console.log("saving");
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
    console.log(pageIndex);
    pageGroup.pages.forEach((prepage, index) => {
      prepage.side = index === 0 ? "left" : "right";
      prepage.page = pageIndex + 1;
      prepage.active = true;

      console.log(prepage);
      prepagesStore.modifyPrepage(prepage);
    });
  });
  inactivePages.forEach((prepage) => {
    prepage.active = false;
    console.log(prepage);
    prepagesStore.modifyPrepage(prepage);
  });
}
</script>

<style lang="sass" scoped>
.dragArea
  background-color: $grey-3
</style>
