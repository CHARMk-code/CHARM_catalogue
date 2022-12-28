<template>
  <v-card>
    <v-card-title>Prepages</v-card-title>
    <v-card-text>
      <Table
        @saveRow="(p) => prepagesStore.modifyPrepage(p)"
        @deleteRow="(p) => prepagesStore.deletePrepage(p)"
        name="Prepage"
        :tableColumns="headers"
        :rows="Array.from(prepagesStore.prepages)"
        :colMeta="colMeta"
        :editable="true"
      >
        <template #col(active)="{ value }">
          <template v-if="value"
            ><v-icon color="primary">mdi-eye</v-icon></template
          >
          <template v-else><v-icon color="grey">mdi-eye-off</v-icon></template>
        </template>
        <template #actions="{ item }">
          <v-btn
            variant="plain"
            size="small"
            icon="mdi-book-open"
            @click="router.push('/prepages/' + 1)"
          ></v-btn>
        </template>
      </Table>
    </v-card-text>
  </v-card>
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
