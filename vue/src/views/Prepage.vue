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
import { useSite_settingsStore } from "@/stores/modules/site_settings";
import {
  onMounted,
  onUnmounted,
  computed,
  onBeforeMount,
  watch,
  watchEffect,
} from "vue";
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
  if (route.params.page) return parseInt(route.params.page[0]) - 1;
  else return 0;
});

const settingsStore = useSite_settingsStore();

watchEffect(() => {
  if (page.value + 1 >= prepagesStore.active_prepages.length) {
    filtersStore.filterCompanies();
    settingsStore.settings.navigation.next =
      "/company/" + filtersStore.filteredCompanies[0].name;
  } else {
    settingsStore.settings.navigation.next = "/prepage/" + (page.value + 2);
  }

  if (page.value > 0) {
    settingsStore.settings.navigation.prev = "/prepage/" + page.value;
  }
});

function handleSwipe({ direction }) {
  if (direction === "right") {
    prev();
  } else if (direction === "left") {
    next();
  }
}

function next() {
  const maybeNext: string | undefined = settingsStore.consumeNext();
  if (maybeNext) router.push(maybeNext);
}

function prev() {
  const maybePrev: string | undefined = settingsStore.consumePrev();
  if (maybePrev) router.push(maybePrev);
}
</script>

<style scoped lang="sass">
.prepage
  height: calc(100vh - 53px)
</style>
