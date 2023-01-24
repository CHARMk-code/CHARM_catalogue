<template>
  <div v-if="layout != undefined">
    <Image :imageName="layout.image" fit="contain" />
  </div>
</template>

<script lang="ts" setup>
import { useLayoutsStore } from "@/stores/modules/layouts";
import Image from "../utils/Image.vue";

const layoutsStore = useLayoutsStore();
const layout = layoutsStore.getCenter;
</script>
