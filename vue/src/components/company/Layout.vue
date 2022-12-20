<template>
  <v-img v-if="layout != undefined" :src="base_URL + layout.image" contain />
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
import { useLayoutsStore } from "@/stores/modules/layouts";

const base_URL = axios.defaults.baseURL + "/manage/image/";

const layoutsStore = useLayoutsStore();
const layout = layoutsStore.getCenter;
</script>
