<template>
      <MapViewer/>
</template>

<script lang="ts" setup>
import { useMapsStore } from "@/stores/modules/maps";
import { computed } from "vue";
import { useRoute } from "vue-router";
import MapViewer from "@/components/map/MapViewer.vue";

const mapsStore = useMapsStore();

const route = useRoute();

const maps = computed(() => {
  return Array.from(mapsStore.maps.values()).filter(
    (m) => m.name === route.params.page
  );
});
</script>
