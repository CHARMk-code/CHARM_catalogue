<template>
  <q-page class="row">
    <template
      v-if="$q.screen.gt.sm"
      v-for="prepage in prepagesStore.pageGroups[$route.params.page].pages"
    >
      <Image
        :class="{ prepage: true, 'col-6': pagesToShow === 2 }"
        fit="contain"
        position="50% top"
        :draggable="false"
        :imageName="prepage.image"
        v-touch-swipe="handleSwipe"
      />
    </template>
    <template v-if="$q.screen.lt.md && mobilePrepage">
      <Image
        v-if="mobilePrepage.mobile"
        class="prepage"
        fit="contain"
        position="50% top"
        :draggable="false"
        :imageName="mobilePrepage.image"
        v-touch-swipe="handleSwipe"
      />
    </template>
  </q-page>
</template>

<script lang="ts" setup>
import Image from "@/components/utils/Image.vue";
import axios from "@/plugins/axios";
import { useCompaniesStore } from "@/stores/modules/companies";
import { useFilterStore } from "@/stores/modules/filter";
import { usePrepagesStore, type Prepage } from "@/stores/modules/prepages";
import { useSite_settingsStore } from "@/stores/modules/site_settings";
import { useQuasar } from "quasar";
import {
  onMounted,
  onUnmounted,
  computed,
  onBeforeMount,
  watch,
  watchEffect,
  ref,
} from "vue";
import { useRoute, useRouter } from "vue-router";

const companiesStore = useCompaniesStore();
const filtersStore = useFilterStore();
const prepagesStore = usePrepagesStore();
const router = useRouter();
const route = useRoute();

const mobilePrepage: Prepage = computed(() => {
  const pageGroupPages: Prepage[] =
    prepagesStore.pageGroups[route.params.page].pages;
  if (route.query.p) {
    const mobilePages = pageGroupPages.filter((p) => p.mobile);

    return mobilePages[route.query.p];
  } else {
    return pageGroupPages[0];
  }
});

const pagesToShow = computed(
  () => prepagesStore.pageGroups[route.params.page].pages.length
);

onBeforeMount(() => {
  prepagesStore.getPrepages();
});

onMounted(() => {
  window.addEventListener("keydown", arrowKeyHandler);
  setNextRoute();
  setPrevRoute();
});

watch(
  () => route.params.page,
  (name) => {
    if (name) {
      setNextRoute();
      setPrevRoute();
    }
  }
);
watch(
  () => route.query.p,
  (name) => {
    if (name) {
      setNextRoute();
      setPrevRoute();
    }
  }
);

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
  if (route.params.page) return parseInt(route.params.page) - 1;
  else return 0;
});

const settingsStore = useSite_settingsStore();
const $q = useQuasar();
function setNextRoute() {
  if ($q.screen.lt.md) {
    var p = 0;
    if (route.query.p) {
      p = parseInt(route.query.p);
    }
    const pageGroupPages = prepagesStore.pageGroups[route.params.page].pages;

    const mobilePages = pageGroupPages.filter((p) => p.mobile);

    if (mobilePages.length - 1 > p) {
      settingsStore.settings.navigation.next =
        "/prepage/" + route.params.page + "?p=" + (p + 1);
      return;
    }
  }
  if (page.value + 1 >= Object.values(prepagesStore.pageGroups).length) {
    filtersStore.filterCompanies();
    if (filtersStore.filteredCompanies.length > 0) {
      settingsStore.settings.navigation.next =
        "/company/" + filtersStore.filteredCompanies[0].name;
    }
  } else {
    settingsStore.settings.navigation.next = "/prepage/" + (page.value + 2);
  }
}
function setPrevRoute() {
  if ($q.screen.lt.md) {
    var p = 0;
    if (route.query.p) {
      p = parseInt(route.query.p);
    }

    if (p > 0) {
      settingsStore.settings.navigation.prev =
        "/prepage/" + route.params.page + "?p=" + (p - 1);
      return;
    }
  }
  if (page.value > 0) {
    settingsStore.settings.navigation.prev = "/prepage/" + page.value;
  }
}

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
