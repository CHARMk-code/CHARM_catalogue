<template>
  <q-page class="row">
    <template v-if="$q.screen.gt.sm">
      <template
        v-for="(prepage, index) in prepagesStore.pageGroups[$route.params.page]
          .pages"
        :key="index"
      >
        <Image
          v-touch-swipe="handleSwipe"
          :class="{ prepage: true, 'col-6': pagesToShow === 2 }"
          fit="contain"
          :position="prepagePositioning(pagesToShow, index)"
          :draggable="false"
          :image-name="prepage.image"
        />
      </template>
    </template>
    <template v-if="$q.screen.lt.md && mobilePrepage">
      <Image
        v-if="mobilePrepage.mobile"
        v-touch-swipe="handleSwipe"
        class="prepage"
        fit="contain"
        position="center top"
        :draggable="false"
        :image-name="mobilePrepage.image"
      />
    </template>
  </q-page>
</template>

<script lang="ts" setup>
import Image from "@/components/utils/Image.vue";
import SwipeOverlay from "@/components/company/SwipeOverlay.vue";
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
  type Ref,
} from "vue";
import { useRoute, useRouter } from "vue-router";

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
  let nextPage, nextP;
  if ($q.screen.lt.md) {
    //if mobile
    let p = 0;
    if (route.query.p) {
      p = parseInt(route.query.p);
    }

    const pageGroupPages = prepagesStore.pageGroups[route.params.page].pages;
    const mobilePages = pageGroupPages.filter((p: Prepage) => p.mobile);

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

    settingsStore.session_settings.navigation.next =
      "/company/" + filtersStore.filteredCompanies[0].name;
  } else {
    settingsStore.session_settings.navigation.next = `/prepage/${nextPage}${
      $q.screen.lt.md ? "?p=" + nextP : ""
    }`;
  }
}
function setPrevRoute() {
  let prevPage, prevP;
  if (page.value <= 1) return;
  if ($q.screen.lt.md) {
    //if mobile
    let p = 0;
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

  settingsStore.session_settings.navigation.prev = `/prepage/${prevPage}${
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

// Logic for swipe instructions on mobile
if ($q.screen.lt.md && !(localStorage.getItem("swipeInstructions") || false)) {
  $q.dialog({ component: SwipeOverlay });
}
</script>

<style scoped lang="sass">
.prepage
  height: calc(100vh - 53px)
</style>
