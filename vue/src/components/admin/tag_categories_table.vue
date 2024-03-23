<template>
  <q-card>
    <q-card-section class="text-h5">Tag Categories</q-card-section>
    <q-card-section>
      <Table
        name="Tag Categories"
        :table-columns="headers"
        :rows="Array.from(tagCategoriesStore.tag_categories.values())"
        :col-meta="colMeta"
        :editable="true"
        @save-row="(t) => tagCategoriesStore.updateTagCategory(t)"
        @delete-row="(t) => tagCategoriesStore.removeTagCategory(t)"
      >
        <template #body-cell-Icon="props">
          <q-td :props="props">
            <Tags :tags="[props.value]"></Tags>
          </q-td>
        </template>

        <template #body-cell-Categories="props">
          <q-td :props="props">
            {{ props.value }}
          </q-td>
        </template>
      </Table>
    </q-card-section>
  </q-card>
</template>

<script lang="ts" setup>
import Table from "@/components/table.vue"
import Tags from "@/components/Tag_group.vue"
import type { TableColMeta } from "./table_edit_dialog.vue"
import { useTagCategoriesStore } from "@/stores/modules/tag_category"

const tagCategoriesStore = useTagCategoriesStore()

const headers = [
  { name: "Name", label: "Name", field: "name", align: "left", sortable: true },
]

const colMeta: TableColMeta[] = [
  { type: "text", model: "name", label: "Tag Category Name" },
]
</script>
