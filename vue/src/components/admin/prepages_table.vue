<template>
  <v-container>
    <Table
      @saveRow="(p) => prepagesStore.modifyPrepage(p)"
      @deleteRow="(p) => prepagesStore.deletePrepage(p)"
      name="Prepage"
      :tableColumns="headers"
      :rows="Array.from(prepagesStore.prepages)"
      :colMeta="colMeta"
      :editable="true"
    >
      <template v-slot:item.name="{ item }">
        {{ item.name }}
      </template>
      <template v-slot:item.order="{ item }">
        {{ item.order }}
      </template>
      <template #actions="{ item }">
        <v-btn
          variant="plain"
          size="small"
          icon="mdi-book-open"
          @click="router.push('/prepages/' + 1)"
        ></v-btn>
      </template>
      <template v-slot:item.active="{ item }">
        <v-simple-checkbox
          disabled
          on-icon="mdi-eye"
          off-icon="mdi-eye-off"
          v-model="item.active"
        ></v-simple-checkbox>
      </template>
    </Table>
  </v-container>
</template>

<script lang="ts" setup>
import Table from "@/components/table.vue";
import { usePrePagesStore } from "@/stores/modules/prepages";
import { useRouter } from "vue-router";
import type { TableColMeta } from "./table_edit_dialog.vue";

const prepagesStore = usePrePagesStore();

const router = useRouter();

const headers = [
  { name: "Name", value: "name" },
  { name: "Order", value: "order" },
  { name: "Active", value: "active" },
];
const colMeta: TableColMeta[] = [
  { type: "image", model: "image", label: "page image" },
  { type: "checkbox", model: "active", label: "Active" },
  { type: "text", model: "order", label: "Order" },
  { type: "text", model: "name", label: "Name" },
];
</script>
