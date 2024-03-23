<template>
  <q-card>
    <q-card-section class="text-h5">Tags</q-card-section>
    <q-card-section>
      <Table
        name="Tags"
        :table-columns="headers"
        :rows="Array.from(tagsStore.tags.values())"
        :meta-rows="metaRows"
        :col-meta="colMeta"
        :editable="true"
        :meta-model-callback="updateWithMetaModels"
        @save-row="(t) => tagsStore.updateTag(t)"
        @delete-row="(t) => tagsStore.removeTag(t)"
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
import { useTagsStore, type Tag } from "@/stores/modules/tags"
import type { TableColMeta } from "./table_edit_dialog.vue"
import { useTagCategoriesStore } from "@/stores/modules/tag_category"
import { reactive } from "vue"

const tagsStore = useTagsStore()
const tagCategoriesStore = useTagCategoriesStore()

interface metaRow {
  category: { label: { name: string}, value: number}
}

const headers = [
  { name: "Icon", label: "Icon", field: (row: any) => row, align: "left" },
  { name: "Name", label: "Name", field: "name", align: "left", sortable: true },
  {
    name: "Categories",
    label: "Categories",
    field: (row: Tag) =>
      tagCategoriesStore.tag_categories.get(row.category)?.name,
    align: "left",
    sortable: true,
  },
]

const metaRows: metaRow[] = Array.from(tagsStore.tags.values()).map((row) => {
  const category = tagCategoriesStore.tag_categories.get(row.category)

  return reactive({
    category: category
      ? { label: { name: category.name }, value: category.id }
      : { label: { name: "ERROR!!!!" }, value: -1 },
  })
})

function updateWithMetaModels(meta: metaRow, row: Tag) {
  row.category = meta.category.value
}

console.log(
  [...tagCategoriesStore.tag_categories.values()].map((category) => ({
    label: category.name,
    value: category.id,
  })),
)
const colMeta: TableColMeta[] = [
  { type: "image", model: "icon", label: "Tag Icon" },
  { type: "text", model: "name", label: "Tag Name" },
  {
    type: "single-select",
    model: "category",
    label: "Tag Category",
    items: [...tagCategoriesStore.tag_categories.values()].map((category) => ({
      label: { name: category.name },
      value: category.id,
    })),
    meta: true,
  },
]
</script>
