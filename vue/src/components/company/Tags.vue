<template>
  <q-card >
    <q-card-section>
      <div class="text-h6">{{ title }}</div>
      <div>
        <template v-if="tags.length === 0">
          <span>No {{ title }}</span>
        </template>
        <template v-else>
          <TagGroup :tags="tags"></TagGroup>
        </template>
      </div>
    </q-card-section>
  </q-card>
</template>

<script lang="ts" setup>
import type { Tag } from "@/stores/modules/tags"
import TagGroup from "../Tag_group.vue"

defineProps<{
  title: string
  tags: Tag[]
}>()
</script>
