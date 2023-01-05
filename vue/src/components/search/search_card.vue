<template>
  <q-card>
    <q-card-section>
      <q-input
        filled
        v-model="filterStore.filters.query"
        clearable
        @clear="search()"
        @blur="search()"
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
            label="Search"
            color="primary"
            noCaps
            @click="search()"
          ></q-btn>
        </template>
      </q-input>

      <q-slide-transition>
        <div v-show="expanded">
          <tagSelector
            v-if="isVisible('tag_divisions')"
            :tags="tagsStore.divisions"
            v-model:selected_tags="selectedDivisions"
            label="Programs"
            :maxShown="8"
          />
          <tagSelector
            v-if="isVisible('tag_business_areas')"
            :tags="tagsStore.business_areas"
            v-model:selected_tags="selectedBusiness_areas"
            label="Business area"
            :maxShown="3"
          />
          <tagSelector
            v-if="isVisible('tag_looking_for')"
            :tags="tagsStore.looking_for"
            v-model:selected_tags="selectedLooking_for"
            label="Looking for"
            :maxShown="3"
          />
          <tagSelector
            v-if="isVisible('tag_offering')"
            :tags="tagsStore.offering"
            v-model:selected_tags="selectedOfferings"
            label="Offering"
            :maxShown="3"
          />
          <tagSelector
            v-if="isVisible('language')"
            :tags="tagsStore.languages"
            v-model:selected_tags="selectedLanguages"
            label="Language"
            :maxShown="2"
          />
          <q-option-group
            v-model="checkboxes"
            :options="checkbox_options"
            type="checkbox"
          >
          </q-option-group>
        </div>
      </q-slide-transition>
    </q-card-section>
  </q-card>
</template>

<script lang="ts" setup>
import tagSelector from "@/components/search/tag_selector.vue";
import { useCompaniesStore } from "@/stores/modules/companies";
import { useFilterStore } from "@/stores/modules/filter";
import { useSite_settingsStore } from "@/stores/modules/site_settings";
import { useTagsStore, type Tag } from "@/stores/modules/tags";
import { computed, onBeforeMount, onMounted, ref, watch, type Ref } from "vue";
import { useRoute, useRouter } from "vue-router";

const filterStore = useFilterStore();

console.log(filterStore.filters);
const companiesStore = useCompaniesStore();
const tagsStore = useTagsStore();
const site_settingsStore = useSite_settingsStore();

const router = useRouter();
const route = useRoute();

const checkboxes: Ref<string[]> = ref([]);
const isVisible = (name: string) => visibleCards.some((c) => c.name === name);

const tmp = [];
if (filterStore.filters.favorites && isVisible("favorites"))
  tmp.push("favorites");
if (filterStore.filters.charmtalk && isVisible("charmtalk"))
  tmp.push("charmtalk");
if (filterStore.filters.sweden && isVisible("sweden")) tmp.push("sweden");
checkboxes.value = tmp;

let expanded = ref(false);
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
  console.log("search");
  filterStore.filterCompanies().then(() => {
    filterStore.sortCompanies();
    console.log("generatedSRQ", filterStore.generateSearchRouteQuery());
    router.replace({
      path: "/search",
      query: { ...filterStore.generateSearchRouteQuery(), g: null },
    });
  });
}

const visibleCards = site_settingsStore.settings.company_view.cards.filter(
  (card) => card.active
);

const selectedDivisions = computed({
  get() {
    console.log("selectedDivisions", filterStore.filters);
    return tagsStore.getDivisionsFromIds(filterStore.filters.tags.divisions);
  },
  set(v) {
    console.log("settingDivisions", v);
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
</script>
