<template>
  <q-img
  :src="base_URL + encodeURIComponent(imageName)"
  v-bind="$props"
  />
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
const base_URL = axios.defaults.baseURL + "/manage/image/";
const props = defineProps<{
  imageName: string;
}>();
</script>
