<template>
  <div class="">
    <template v-for="(tag, index) in props.tags">
      <template v-if="tag.icon && tag.icon.length > 0">
        <q-avatar
          :key="index"
          size="md"
          v-bind="props"
          clickable
          @click="tagClick(tag)"
        >
          <img :src="base_URL + tag.icon" cover />
          <q-tooltip> {{ tag.name }} </q-tooltip>
        </q-avatar>
      </template>

      <template v-else>
        <q-chip
          :key="index"
          :icon-right="removeable ? 'mdi-close-circle' : ''"
          size="sm"
          class="ma-1"
          clickable
          @click="tagClick(tag)"
        >
          {{ tag.name }}
        </q-chip>
      </template>
    </template>
  </div>
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
import type { Tag } from "@/stores/modules/tags";
const base_URL = axios.defaults.baseURL + "/v2/image/";
const emit = defineEmits<{
  (e: "tag-click", tag: Tag): void;
}>();

const props = withDefaults(
  defineProps<{
    tags: Tag[];
    removeable?: boolean;
  }>(),
  { removeable: false }
);

function tagClick(tag: Tag) {
  emit("tag-click", tag);
}
</script>
