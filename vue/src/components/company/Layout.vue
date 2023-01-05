<template>
  <div v-if="layout != undefined">
    <q-img :src="base_URL + layout.image" fit="contain" />
  </div>
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
import { useLayoutsStore } from "@/stores/modules/layouts";

const base_URL = axios.defaults.baseURL + "/manage/image/";

const layoutsStore = useLayoutsStore();
const layout = layoutsStore.getCenter;
</script>
