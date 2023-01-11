<template>
  <q-card>
    <q-card-section class="text-h5">Feedback</q-card-section>
    <q-card-section>
      <Table
        name="Tags"
        :tableColumns="headers"
        :rows="feedbackStore.feedback"
        :colMeta="colMeta"
        :editable="false"
      >
      </Table>
    </q-card-section>
  </q-card>
</template>

<script lang="ts" setup>
import Table from "@/components/table.vue";
import axios from "@/plugins/axios";
import { useFeedbackStore } from "@/stores/modules/feedback";
import type { TableColMeta } from "./table_edit_dialog.vue";

const feedbackStore = useFeedbackStore();

const headers = [
  {
    name: "Title",
    label: "Title",
    field: "title",
    align: "left",
    sortable: true,
  },
  {
    name: "Text",
    label: "Text",
    field: "text",
    align: "left",
  },
  {
    name: "Received",
    label: "Received",
    field: "received",
    align: "right",
    sortable: true,
  },
];

const colMeta: TableColMeta[] = [];
</script>
