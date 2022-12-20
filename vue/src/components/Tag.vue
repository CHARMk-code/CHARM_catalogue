<template>
  <template v-if="!tag.icon || tag.icon == ''">
    <v-chip size="small" class="ma-1">
      {{ tag.name }}
    </v-chip>
  </template>
  <template v-else>
    <v-tooltip bottom>
      <template v-slot:activator="{ props }">
        <v-avatar density="compact" size="small" v-bind="props">
          <v-img :src="base_URL + tag.icon" cover />
        </v-avatar>
      </template>
      <span>
        {{ tag.name }}
      </span>
    </v-tooltip>
  </template>
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
import type { Tag } from "@/stores/modules/tags";

const base_URL = axios.defaults.baseURL + "/manage/image/";

const props = defineProps<{
  tag: Tag;
}>();
</script>
