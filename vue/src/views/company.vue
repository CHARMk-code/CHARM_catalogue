<template>
  <q-page v-touch-swipe.left.right="handleSwipe" padding>
    <Filter
      @filter-changed="
        () => {
          setNextRoute();
          setPrevRoute();
        }
      "
    />
    <div v-if="company != undefined && company.active" class="row">
      <Logo class="col-12 col-md-6" :src="company.logo" />

      <Name :id="company.id" class="col-12 col-md-6" :name="company.name" />
    </div>
    <div
      v-if="company != undefined && company.active"
      class="row q-col-gutter-md"
    >
      <div class="col-12 col-md-6 q-gutter-md">
        <component
          :is="comp"
          v-for="(comp, index) in component_layout.left"
          :key="index"
        />
      </div>
      <div class="col-12 col-md-6 q-gutter-md">
        <component
          :is="comp"
          v-for="(comp, index) in component_layout.right"
          :key="index"
        />
      </div>
    </div>
  </q-page>
</template>

<script lang="ts" setup>
import Filter from "@/components/company/Filter.vue";
import Logo from "@/components/company/Logo.vue";
import Name from "@/components/company/Name.vue";
import Trivia from "@/components/company/Trivia.vue";
import Contacts from "@/components/company/Contacts.vue";
import Textblock from "@/components/company/Textblock.vue";
import Website from "@/components/company/Website.vue";
import Tags from "@/components/company/Tags.vue";
import Note from "@/components/company/Note.vue";
import Map from "@/components/company/Map.vue";
import Summerjob from "@/components/company/summerjob.vue";
import Layout from "@/components/company/Layout.vue";
import SwipeOverlay from "@/components/company/SwipeOverlay.vue";
import { computed, h, onMounted, onUnmounted, ref, watch, type Ref } from "vue";
import { useFilterStore } from "@/stores/modules/filter";
import { useRoute, useRouter } from "vue-router";
import { usePrepagesStore, type Prepage } from "@/stores/modules/prepages";
import { useCompaniesStore } from "@/stores/modules/companies";
import { useTagsStore } from "@/stores/modules/tags";
import { useMapsStore } from "@/stores/modules/maps";
import { useSite_settingsStore } from "@/stores/modules/site_settings";
import { useQuasar } from "quasar";

const filterStore = useFilterStore();
const prepagesStore = usePrepagesStore();
const companiesStore = useCompaniesStore();
const tagsStore = useTagsStore();
const mapsStore = useMapsStore();
const settingsStore = useSite_settingsStore();

const route = useRoute();
const router = useRouter();

const company = computed(() => {
  return companiesStore.companyByName(route.params.name);
});

const component_layout: Ref<{ left: any; right: any }> = ref({
  left: [],
  right: [],
});

function isVisible(card_name: string) {
  const visibleCards = settingsStore.settings.company_view.cards;
  return visibleCards.some((card) =>
    card.name === card_name ? card.active : false
  );
}

function renderCompanyCards() {
  function addComponent(side: "left" | "right", comp: any) {
    component_layout.value[side].push(comp);
  }

  component_layout.value = { left: [], right: [] };

  if (company.value) {
    // Description
    if (isVisible("desc")) {
      const vnode = h(Textblock, { body: company.value.description });
      addComponent("left", vnode);
    }
    // Layout
    if (isVisible("layout") || true) {
      const vnode = h(Layout);
      addComponent("left", vnode);
    }
    // Divisions
    if (isVisible("tag_divisions")) {
      const vnode = h(Tags, {
        title: "Divisions",
        name: "tag_divisions",
        tags: tagsStore.getDivisionsFromIds(company.value.tags),
      });
      addComponent("left", vnode);
    }
    // Looking for
    if (isVisible("tag_looking_for")) {
      const vnode = h(Tags, {
        title: "Looking For",
        name: "tag_looking_for",
        tags: tagsStore.getLookingForFromIds(company.value.tags),
      });
      addComponent("left", vnode);
    }
    // Offering
    if (isVisible("tag_offering")) {
      const vnode = h(Tags, {
        title: "Offering",
        name: "tag_offering",
        tags: tagsStore.getOfferingsFromIds(company.value.tags),
      });
      addComponent("left", vnode);
    }
    // Business areas
    if (isVisible("tag_business_areas")) {
      const vnode = h(Tags, {
        title: "Business areas",
        name: "tag_business_areas",
        tags: tagsStore.getBusinessAreasFromIds(company.value.tags),
      });
      addComponent("left", vnode);
    }

    // Map
    if (isVisible("map")) {
      const vnode = h(Map, {
        map: mapsStore.getMapFromId(company.value.map_image),
        boothNumber: company.value.booth_number,
      });
      addComponent("right", vnode);
    }
    // What makes us special
    if (isVisible("desc")) {
      const vnode = h(Textblock, {
        title: "What Makes Us Special",
        body: company.value.unique_selling_point,
      });
      addComponent("right", vnode);
    }
    // Summerjob
    if (isVisible("summerjob")) {
      const vnode = h(Summerjob, {
        name: "summerjob",
        desc: company.value.summer_job_description,
        link: company.value.summer_job_link,
        deadline: company.value.summer_job_deadline,
      });
      addComponent("right", vnode);
    }
    // Website
    if (isVisible("website")) {
      const vnode = h(Website, {
        website: company.value.website,
      });
      addComponent("right", vnode);
    }
    // Contacts
    if (isVisible("contacts")) {
      const vnode = h(Contacts, {
        contacts: company.value.contacts,
        contactEmail: company.value.contact_email,
      });
      addComponent("right", vnode);
    }
    // Trivia
    if (isVisible("trivia")) {
      const vnode = h(Trivia, {
        talkToUsAbout: company.value.talk_to_us_about,
        sweden: company.value.employees_sweden,
        world: company.value.employees_world,
      });
      addComponent("right", vnode);
    }
    // Notes
    if (isVisible("notes")) {
      const vnode = h(Note, {
        id: company.value.id,
      });
      addComponent("right", vnode);
    }
  }
}

const currentIndex = computed(() => {
  return filterStore.filteredCompanies
    .map((x) => x.id)
    .indexOf(company.value.id);
});

onMounted(() => {
  window.addEventListener("keydown", arrowKeyHandler);
  setNextRoute();
  setPrevRoute();
  renderCompanyCards();
});

watch(
  () => route.params.name,
  (name) => {
    if (name) {
      setNextRoute();
      setPrevRoute();
      renderCompanyCards();
    }
  }
);

onUnmounted(() => {
  window.removeEventListener("keydown", arrowKeyHandler);
});

function arrowKeyHandler(e: any) {
  if (e.key == "ArrowRight") {
    next();
  } else if (e.key == "ArrowLeft") {
    prev();
  }
}

function handleSwipe({ direction }) {
  if (direction === "right") {
    prev();
  } else if (direction === "left") {
    next();
  }
}

function setNextRoute() {
  // Set next route
  if (currentIndex.value + 1 < filterStore.filteredCompanies.length) {
    settingsStore.settings.navigation.next =
      "/company/" + filterStore.filteredCompanies[currentIndex.value + 1].name;
  } else {
    settingsStore.settings.navigation.next = undefined;
  }
}

const $q = useQuasar();

function setPrevRoute() {
  //set prev route
  if (currentIndex.value > 0) {
    settingsStore.settings.navigation.prev =
      "/company/" + filterStore.filteredCompanies[currentIndex.value - 1].name;
  } else {
    if (Object.values(prepagesStore.pageGroups).length !== 0) {
      const pageGroups = Object.values(prepagesStore.pageGroups);
      let lastPageGroupIndex = pageGroups.length;

      if ($q.screen.lt.md) {
        let mobilePages = pageGroups[lastPageGroupIndex - 1].pages.filter(
          (p: Prepage) => p.mobile
        );
        while (mobilePages.length == 0) {
          lastPageGroupIndex -= 1;
          mobilePages = pageGroups[lastPageGroupIndex - 1].pages.filter(
            (p: Prepage) => p.mobile
          );
        }
        if (mobilePages.length > 1) {
          settingsStore.settings.navigation.prev =
            "/prepage/" + lastPageGroupIndex + "?p=" + (mobilePages.length - 1);
          return;
        }
      }
      settingsStore.settings.navigation.prev = "/prepage/" + pageGroups.length;
    } else {
      settingsStore.settings.navigation.prev = undefined;
    }
  }
}

function next() {
  const maybeNext: string | undefined = settingsStore.consumeNext();
  if (maybeNext) {
    router.push(maybeNext);
  }
}

function prev() {
  const maybePrev: string | undefined = settingsStore.consumePrev();
  if (maybePrev) {
    router.push(maybePrev);
  }
}

// Logic for swipe instructions on mobile
// has saved thing in localStorage
if ($q.screen.lt.md && !(localStorage.getItem("swipeInstructions") || false)) {
  $q.dialog({ component: SwipeOverlay });
}
</script>

<style lang="sass" scoped>
@media (max-width: $breakpoint-md-min)
  .logo
    width: 100%
</style>
