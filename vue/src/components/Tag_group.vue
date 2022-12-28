<template>
  <template v-for="tag in props.tags">
    <template v-if="tag.icon && tag.icon.length > 0">
      <v-tooltip bottom>
        <template v-slot:activator="{ props }">
          <v-avatar
            style="margin: 2px"
            density="compact"
            size="small"
            v-bind="props"
          >
            <img
              :src="base_URL + tag.icon"
              style="
                object-fit: cover;
                position: absolute;
                width: 100%;
                height: 100%;
              "
            />
          </v-avatar>
        </template>
        <span>
          {{ tag.name }}
        </span>
      </v-tooltip>
    </template>
    <template v-else>
      <v-chip size="small" class="ma-1">
        {{ tag.name }}
      </v-chip>
    </template>
  </template>
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
import type { Tag } from "@/stores/modules/tags";

const base_URL = axios.defaults.baseURL + "/manage/image/";

const props = defineProps<{
  tags: Tag[];
}>();
</script>
