<template>
  <v-main>
    <v-sheet class="pa-6">
      <Image
        v-if="maps.length > 0"
        contain
        justify="center"
        :imageName="maps[0].image"
      />
    </v-sheet>
  </v-main>
</template>

<script lang="ts" setup>
import { useMapsStore } from "@/stores/modules/maps";
import { computed } from "vue";
import { useRoute } from "vue-router";
import Image from "@/components/utils/Image.vue";

const mapsStore = useMapsStore();

const route = useRoute();

const maps = computed(() => {
  return Array.from(mapsStore.maps.values()).filter(
    (m) => m.name === route.params.page
  );
});
</script>
