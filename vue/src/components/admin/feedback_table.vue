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
        <template #top-right>
          <q-btn
            color="primary"
            label="Save changes"
            @click="feedbackStore.sendAllFeedback()"
          />
        </template>
        <template #body-cell-Important="props">
          <q-td :props="props" auto-width>
            <q-checkbox v-model="props.row.important" />
          </q-td>
        </template>

        <template #body-cell-Archived="props">
          <q-td :props="props" auto-width>
            <q-checkbox v-model="props.row.archived" />
          </q-td>
        </template>
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
    name: "Important",
    label: "Important",
    field: "important",
    align: "left",
    sortable: true,
  },
  {
    name: "Archived",
    label: "Archived",
    field: "archived",
    align: "left",
    sortable: true,
  },
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
