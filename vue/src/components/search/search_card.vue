<template>
  <q-card flat>
    <q-card-section>
      <q-input
        v-model="filterStore.filters.query"
        filled
        clearable
        @update:model-value="search()"
      >
        <template #prepend> <q-icon name="mdi-magnify"></q-icon> </template>
        <template #after>
          <q-btn
            flat
            round
            dense
            :icon="expanded ? 'mdi-chevron-up' : 'mdi-chevron-down'"
            size="lg"
            @click="expanded = !expanded"
          ></q-btn>
          <q-btn
            class="q-ml-sm"
            label="Search"
            color="primary"
            no-caps
            @click="search()"
          ></q-btn>
        </template>
      </q-input>

      <q-slide-transition>
        <div v-show="expanded">
          <tagSelector
            v-if="isVisible('tag_divisions')"
            v-model:selectedTags="selectedDivisions"
            :tags="tagsStore.divisions"
            label="Programs"
            :max-shown="8"
          />
          <tagSelector
            v-if="isVisible('tag_business_areas')"
            v-model:selectedTags="selectedBusiness_areas"
            :tags="tagsStore.business_areas"
            label="Business areas"
            :max-shown="3"
          />
          <tagSelector
            v-if="isVisible('tag_looking_for')"
            v-model:selectedTags="selectedLooking_for"
            :tags="tagsStore.looking_for"
            label="Looking for"
            :max-shown="3"
          />
          <tagSelector
            v-if="isVisible('tag_offering')"
            v-model:selectedTags="selectedOfferings"
            :tags="tagsStore.offering"
            label="Offering"
            :max-shown="3"
          />
          <tagSelector
            v-if="isVisible('language')"
            v-model:selectedTags="selectedLanguages"
            :tags="tagsStore.languages"
            label="Language"
            :max-shown="2"
          />
          <tagSelector
            v-if="isVisible('fair_area')"
            v-model:selectedTags="selectedFairAreas"
            :tags="tagsStore.fair_areas"
            label="Fair Areas"
            :max-shown="2"
          />
          <q-option-group
            v-model="checkboxes"
            :options="checkbox_options"
            type="checkbox"
          >
          </q-option-group>
          <!-- <q-btn
            label="Clear filters"
            noCaps
            @click="filterStore.resetFilter()"
          ></q-btn> -->
          <q-btn
            label="Apply filters"
            no-caps
            color="primary"
            class="q-mt-sm q-ml-sm"
            @click="search()"
          >
          </q-btn>
        </div>
      </q-slide-transition>
    </q-card-section>
  </q-card>
</template>

<script lang="ts" setup>
import tagSelector from "@/components/search/tag_selector.vue";
import { useFilterStore } from "@/stores/modules/filter";
import { useSite_settingsStore } from "@/stores/modules/site_settings";
import { useTagsStore, type Tag } from "@/stores/modules/tags";
import { computed, ref, watch, type Ref } from "vue";
import { useRouter } from "vue-router";

const filterStore = useFilterStore();

const tagsStore = useTagsStore();
const site_settingsStore = useSite_settingsStore();

const router = useRouter();

const checkboxes: Ref<string[]> = ref([]);
const isVisible = (name: string) => visibleCards.some((c) => c.name === name);

const tmp = [];
if (filterStore.filters.favorites && isVisible("favorites"))
  tmp.push("favorites");
if (filterStore.filters.charmtalk && isVisible("charmtalk"))
  tmp.push("charmtalk");
if (filterStore.filters.sweden && isVisible("sweden")) tmp.push("sweden");
checkboxes.value = tmp;

const expanded = ref(false);
watch(checkboxes, async (vs: string[]) => {
  const tmp = {
    ...filterStore.filters,
    favorites: false,
    charmtalk: false,
    sweden: false,
  };
  for (const v of vs) {
    switch (v) {
      case "favorites":
        tmp.favorites = true;
        break;
      case "charmtalk":
        tmp.charmtalk = true;
        break;
      case "sweden":
        tmp.sweden = true;
        break;
    }
  }
  filterStore.filters = tmp;
});

const checkbox_options = computed(() => {
  const tmp = [];
  if (!isVisible("favorites"))
    tmp.push({ label: "Favorites", value: "favorites" });
  if (!isVisible("charmtalk"))
    tmp.push({ label: "Attending CHARMtalks", value: "charmtalk" });
  if (!isVisible("sweden")) tmp.push({ label: "Sweden", value: "sweden" });
  return tmp;
});

function search() {
  filterStore.filterCompanies().then(() => {
    const searchQuery = filterStore.generateSearchRouteQuery();
    router.replace({
      path: "/search",
      query: searchQuery,
    });
  });
}

const visibleCards = site_settingsStore.server_settings.company_view.cards.filter(
  (card) => card.active
);

const selectedDivisions = computed({
  get() {
    return tagsStore.getDivisionsFromIds(filterStore.filters.tags.divisions);
  },
  set(v) {
    if (v) {
      filterStore.filters.tags.divisions = v.map((t: Tag) => t.id);
    } else {
      filterStore.filters.tags.divisions = [];
    }
  },
});
const selectedBusiness_areas = computed({
  get() {
    return tagsStore.getBusinessAreasFromIds(
      filterStore.filters.tags.business_areas
    );
  },
  set(v) {
    if (v) {
      filterStore.filters.tags.business_areas = v.map((t: Tag) => t.id);
    } else {
      filterStore.filters.tags.business_areas = [];
    }
  },
});
const selectedLooking_for = computed({
  get() {
    return tagsStore.getLookingForFromIds(filterStore.filters.tags.looking_for);
  },
  set(v) {
    if (v) {
      filterStore.filters.tags.looking_for = v.map((t: Tag) => t.id);
    } else {
      filterStore.filters.tags.looking_for = [];
    }
  },
});
const selectedOfferings = computed({
  get() {
    return tagsStore.getOfferingsFromIds(filterStore.filters.tags.offerings);
  },
  set(v) {
    if (v) {
      filterStore.filters.tags.offerings = v.map((t: Tag) => t.id);
    } else {
      filterStore.filters.tags.offerings = [];
    }
  },
});
const selectedLanguages = computed({
  get() {
    return tagsStore.getLanguagesFromIds(filterStore.filters.tags.languages);
  },
  set(v) {
    if (v) {
      filterStore.filters.tags.languages = v.map((t: Tag) => t.id);
    } else {
      filterStore.filters.tags.languages = [];
    }
  },
});
const selectedFairAreas = computed({
  get() {
    return tagsStore.getFairAreasFromIds(filterStore.filters.tags.fair_areas);
  },
  set(v) {
    if (v) {
      filterStore.filters.tags.fair_areas = v.map((t: Tag) => t.id);
    } else {
      filterStore.filters.tags.fair_areas = [];
    }
  },
});
</script>
