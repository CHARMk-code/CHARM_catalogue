<template>
  <v-main>
    <v-sheet class="pa-6">
      <v-img
        v-if="maps.length > 0"
        contain
        justify="center"
        :src="base_URL + maps[0].image"
      />
    </v-sheet>
  </v-main>
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
import { useMapsStore } from "@/stores/modules/maps";
import { computed } from "vue";
import { useRoute } from "vue-router";

const base_URL = axios.defaults.baseURL + "/manage/image/";

const mapsStore = useMapsStore();

const route = useRoute();

const maps = computed(() => {
  return Array.from(mapsStore.maps.values()).filter(
    (m) => m.name === route.params.page
  );
});
</script>
