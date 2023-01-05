<template>
  <q-page>
    <q-img
      class="prepage"
      fit="contain"
      width="100%"
      position="50% top"
      :draggable="false"
      :src="base_URL + prepagesStore.active_prepages[page].image"
      v-touch-swipe="handleSwipe"
    />
  </q-page>
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
import { useCompaniesStore } from "@/stores/modules/companies";
import { useFilterStore } from "@/stores/modules/filter";
import { usePrepagesStore } from "@/stores/modules/prepages";
import { onMounted, onUnmounted, computed, onBeforeMount } from "vue";
import { useRoute, useRouter } from "vue-router";

const companiesStore = useCompaniesStore();
const filtersStore = useFilterStore();
const prepagesStore = usePrepagesStore();
const router = useRouter();
const route = useRoute();

const base_URL = axios.defaults.baseURL + "/manage/image/";

onBeforeMount(() => {
  prepagesStore.getPrepages();
});

onMounted(() => {
  window.addEventListener("keydown", arrowKeyHandler);
});

onUnmounted(() => {
  window.removeEventListener("keydown", arrowKeyHandler);
});

const arrowKeyHandler = (e: { key: string }) => {
  if (e.key == "ArrowRight") {
    next();
  } else if (e.key == "ArrowLeft") {
    prev();
  }
};

const page = computed(() => {
  return parseInt(route.params.page[0]) - 1;
});

function handleSwipe({ direction }) {
  if (direction === "right") {
    prev();
  } else if (direction === "left") {
    next();
  }
}

const next = () => {
  if (page.value + 1 >= prepagesStore.active_prepages.length) {
    filtersStore.filterCompanies();
    router.push("/company/" + filtersStore.filteredCompanies[0].name);
  } else {
    router.push("/prepage/" + (page.value + 1));
  }
};
const prev = () => {
  const next_index = page.value - 1;
  if (next_index >= 0) {
    router.push("/prepage/" + next_index);
  }
};
</script>

<style scoped lang="sass">
.prepage
  height: calc(100vh - 53px)
</style>
