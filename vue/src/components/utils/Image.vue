<template>
  <q-img :src="base_URL + encodeURIComponent(imageName)" v-bind="$props" />
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
const base_URL = axios.defaults.baseURL + "/manage/image/";

defineProps<{
  imageName: string;
  position?: string;
  ratio?: string | number;
  width?: string;
  height?: string;
  fit?: "cover" | "fill" | "contain" | "none" | "scale-down";
  draggable?: boolean;
}>();
</script>
