<template>
  <div class="">
    <template v-for="(tag, index) in props.tags" >
      <template v-if="tag.icon && tag.icon.length > 0">
        <q-avatar size="md" v-bind="props" clickable @click=tagClick(tag)>
          <img :src="base_URL + tag.icon" cover />
          <q-tooltip> {{ tag.name }} </q-tooltip>
        </q-avatar>
      </template>

      <template v-else>
        <q-chip :icon-right="removeable ? 'mdi-close-circle': ''" size="sm" class="ma-1" clickable @click=tagClick(tag)>
          {{ tag.name }}
        </q-chip>
      </template>
    </template>
  </div>
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
import type { Tag } from "@/stores/modules/tags";
const base_URL = axios.defaults.baseURL + "/manage/image/";
const emit = defineEmits<{
  (e: "tagClick", tag:Tag): void;
}>()

const props = defineProps<{
  tags: Tag[];
  removeable: boolean;
}>();


function tagClick(tag: Tag){

  emit("tagClick", tag);
}
</script>
