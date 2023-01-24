<template>
  <q-page class="row">
    <template
      v-if="$q.screen.gt.sm"
      v-for="(prepage, index) in prepagesStore.pageGroups[$route.params.page]
        .pages"
    >
      <Image
        :class="{ prepage: true, 'col-6': pagesToShow === 2 }"
        fit="contain"
        :position="prepagePositioning(pagesToShow, index)"
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
        position="center top"
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

const mobilePrepage: Ref<Prepage> = computed(() => {
  const pageGroupPages: Prepage[] =
    prepagesStore.pageGroups[route.params.page].pages;
  const mobilePages = pageGroupPages.filter((p) => p.mobile);
  if (route.query.p) {
    return mobilePages[route.query.p];
  } else {
    return mobilePages[0];
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

function prepagePositioning(amount: number, index: number) {
  if (amount === 2) {
    if (index === 0) {
      return "right top";
    } else {
      return "left top";
    }
  } else {
    return "center top";
  }
}

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
  if (route.params.page) return parseInt(route.params.page);
  else return 0;
});

const settingsStore = useSite_settingsStore();
const $q = useQuasar();
function setNextRoute() {
  var nextPage, nextP;
  if ($q.screen.lt.md) {
    //if mobile
    var p = 0;
    if (route.query.p) {
      p = parseInt(route.query.p);
    }

    const pageGroupPages = prepagesStore.pageGroups[route.params.page].pages;
    const mobilePages = pageGroupPages.filter((p) => p.mobile);

    if (mobilePages.length - 1 > p) {
      nextPage = page.value;
      nextP = p + 1;
    } else {
      nextPage = page.value + 1;
      nextP = 0;
    }
  } else {
    //if desktop
    nextPage = page.value + 1;
    nextP = 0;
  }

  // determine if nextPage is company or prepage
  if (nextPage > Object.values(prepagesStore.pageGroups).length) {
    filtersStore.filterCompanies();
    if (filtersStore.filteredCompanies.length < 1) return; //go nowhere if there are no companypages

    settingsStore.settings.navigation.next =
      "/company/" + filtersStore.filteredCompanies[0].name;
  } else {
    settingsStore.settings.navigation.next = `/prepage/${nextPage}${
      $q.screen.lt.md ? "?p=" + nextP : ""
    }`;
  }
}
function setPrevRoute() {
  var prevPage, prevP;
  if (page.value <= 1) return;
  if ($q.screen.lt.md) {
    //if mobile
    var p = 0;
    if (route.query.p) {
      p = parseInt(route.query.p);
    }

    const prevPageGroupPages =
      prepagesStore.pageGroups[parseInt(route.params.page) - 1].pages;
    const prevMobilePages = prevPageGroupPages.filter((p) => p.mobile);

    if (p > 0) {
      prevPage = page.value;
      prevP = p - 1;
    } else {
      prevPage = page.value - 1;
      if (prevMobilePages.length > 1) {
        prevP = prepagesStore.pageGroups[prevPage].pages.length - 1;
      } else {
        prevP = 0;
      }
    }
  } else {
    //if desktop
    prevPage = page.value - 1;
    prevP = 0;
  }

  settingsStore.settings.navigation.prev = `/prepage/${prevPage}${
    $q.screen.lt.md ? "?p=" + prevP : ""
  }`;
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
