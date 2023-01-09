<template>
  <div class="">
    <template v-for="(tag, index) in  data.mutable_tags" >
      <template v-if="tag.icon && tag.icon.length > 0">
        <q-avatar size="md" v-bind="props" clickable @click=removeTag(tag)>
          <img :src="base_URL + tag.icon" cover />
          <q-tooltip> {{ tag.name }} </q-tooltip>
        </q-avatar>
      </template>

      <template v-else>
        <q-chip size="sm" class="ma-1" clickable @click=removeTag(tag)>
          {{ tag.name }}
        </q-chip>
      </template>
    </template>
  </div>
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
import type { Tag } from "@/stores/modules/tags";
import { useFilterStore } from "@/stores/modules/filter";
import { reactive } from "vue";
const base_URL = axios.defaults.baseURL + "/manage/image/";
const filterStore = useFilterStore();
const emit = defineEmits()

const props = defineProps<{
  tags: Tag[];
  mutable: boolean;
}>();

const data = reactive({mutable_tags: props.tags});

function removeTag(tag: Tag){
  if (props.mutable !== true) { return; }

  data.mutable_tags = data.mutable_tags.filter((t) => t.id != tag.id);

  const pre_tags = filterStore.filters.tags;
  filterStore.filters.tags.divisions = pre_tags.divisions.filter((t) => t != tag.id);
  filterStore.filters.tags.business_areas = pre_tags.business_areas.filter((t) => t != tag.id);
  filterStore.filters.tags.fair_areas = pre_tags.fair_areas.filter((t) => t != tag.id);
  filterStore.filters.tags.offerings = pre_tags.offerings.filter((t) => t != tag.id);
  filterStore.filters.tags.languages = pre_tags.languages.filter((t) => t != tag.id);
  filterStore.filters.tags.looking_for = pre_tags.looking_for.filter((t) => t != tag.id);
  filterStore.filterCompanies();
  emit("removed");
}
</script>
