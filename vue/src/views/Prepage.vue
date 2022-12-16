<template>
  <sideLayout
    v-on:next="next"
    v-on:prev="prev"
    v-bind:button_left="page > 0"
    v-bind:button_right="true"
  >
    <v-sheet
      class="prepage-sheet"
      v-touch="{ right: () => prev(), left: () => next() }"
    >
      <v-img
        class="prepage"
        contain
        max-width="100%"
        max-height="90vh"
        :src="base_URL + prepagesStore.active_prepages[page].image"
      />
    </v-sheet>
  </sideLayout>
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
import { useCompaniesStore } from "@/stores/modules/companies";
import { useFilterStore } from "@/stores/modules/filter";
import { usePrePagesStore } from "@/stores/modules/prepages";
import sideLayout from "@/views/sideLayout.vue";
import { onMounted, onUnmounted, computed, onBeforeMount } from "vue"
import { useRoute, useRouter } from "vue-router";

const companiesStore = useCompaniesStore();
const filtersStore = useFilterStore();
const prepagesStore = usePrePagesStore();
const router = useRouter();
const route = useRoute();

const base_URL = axios.defaults.baseURL + "/manage/image/"

onBeforeMount(() => {
  prepagesStore.getPrepages();
})

onMounted(() => {
  window.addEventListener("keydown", arrowKeyHandler);
})

onUnmounted(() => {
  window.removeEventListener("keydown", arrowKeyHandler);
})

const arrowKeyHandler = (e: { key: string; }) => {
  if (e.key == "ArrowRight") {
    next();
  } else if (e.key == "ArrowLeft") {
    prev();
  }
}

const page = computed(() => { return parseInt(route.params.page[0])})

const next = () => {
  if (page.value + 1 >= prepagesStore.active_prepages.length) {
    companiesStore.getCompanies();
    filtersStore.filterCompanies();
    router.push("/company/" + filtersStore.filteredCompanies[0].name);
  } else {
    router.push("/prepages/" + (page.value + 1));
  }
}
const prev = () => {
  const next_index = page.value - 1;
  if (next_index >= 0) {
    router.push("/prepages/" + next_index);
  }
}
</script>