<template>
  <v-container class="px-0">
    <v-sheet class="pa-0" fluid elevation="1">
      <v-card-title> Search </v-card-title>

      <v-card-text>
        <v-row class="px-2">
          <v-text-field
            prepend-icon="mdi-magnify"
            v-model="filter.query"
            @input="search"
          />
          <v-btn x-large @click="expand = !expand" class="ml-6 mt-2" icon
            ><v-icon large
              >{{ expand ? "mdi-chevron-up" : "mdi-chevron-down" }}
            </v-icon></v-btn
          >
        </v-row>
        <v-expand-transition>
          <v-row
            v-show="expand"
            class="mx-2 mb-1 flex align-content-space-around"
            style="gap: 16px"
          >
            <tagSelector
              v-if="isVisible('tag_divisions')"
              @change="
                (tags: number[]) => {
                  filter.tags.divisions = tags;
                  search();
                }
              "
              :tags="tagsStore.divisions"
              :selected_tags="filter.tags.divisions"
              label="Programs"
            />
            <tagSelector
              v-if="isVisible('tag_business_areas')"
              @change="
                (tags: number[]) => {
                  filter.tags.business_areas = tags;
                  search();
                }
              "
              :tags="tagsStore.business_areas"
              :selected_tags="filter.tags.business_areas"
              label="Business area"
            />
            <tagSelector
              v-if="isVisible('tag_looking_for')"
              @change="
                (tags: number[]) => {
                  filter.tags.looking_for = tags;
                  search();
                }
              "
              :tags="tagsStore.looking_for"
              :selected_tags="filter.tags.looking_for"
              label="Looking for"
            />
            <tagSelector
              v-if="isVisible('tag_offering')"
              @change="
                (tags: number[]) => {
                  filter.tags.offerings = tags;
                  search();
                }
              "
              :tags="tagsStore.offering"
              :selected_tags="filter.tags.offerings"
              label="Offering"
            />
            <tagSelector
              v-if="isVisible('language')"
              @change="
                (tags: number[]) => {
                  filter.tags.languages = tags;
                  search();
                }
              "
              :tags="tagsStore.languages"
              :selected_tags="filter.tags.languages"
              label="Language"
            />
            <v-row>
              <v-checkbox
                v-if="isVisible('name')"
                class="ml-2 mr-4"
                @change="search()"
                v-model="filter.favorites"
                label="Only Favorites"
              />
              <v-checkbox
                v-if="isVisible('CHARMtalks')"
                class="ml-2 mr-4"
                @change="search()"
                v-model="filter.charmtalk"
                label="Participating in CHARMtalks"
              />
            </v-row>
            <v-btn @click="filterStore.resetFilter"> Clear filter </v-btn>
          </v-row>
        </v-expand-transition>
      </v-card-text>
    </v-sheet>
  </v-container>
</template>

<script lang="ts" setup>
import tagSelector from "@/components/search/tag_selector.vue";
import { useCompaniesStore } from "@/stores/modules/companies";
import { useFilterStore } from "@/stores/modules/filter";
import { useSite_settingsStore } from "@/stores/modules/site_settings";
import { useTagsStore, type Tag } from "@/stores/modules/tags";
import { useRoute, useRouter } from "vue-router";

const filterStore = useFilterStore();
const companiesStore = useCompaniesStore();
const tagsStore = useTagsStore();
const site_settingsStore = useSite_settingsStore();

const router = useRouter();
const route = useRoute();

const filter = filterStore.filters;

const expand = false;

interface Route_query {
  q?: string
  tags?: string
  favorites?: string
  charmtalk?: string
  sweden?: string
  [key: string]: string | undefined
}

function search() {
  filterStore.filterCompanies()
    .then(() => filterStore.sortCompanies())

  let query: Route_query = {}

  filter.query.length > 0 && (query.q = filter.query);

  if (filter.tags.divisions.length > 0
    || filter.tags.business_areas.length > 0
    || filter.tags.looking_for.length > 0
    || filter.tags.languages.length > 0
    || filter.tags.offerings.length > 0
  ) {
    query.tags = [
      filter.tags.business_areas,
      filter.tags.looking_for,
      filter.tags.languages,
      filter.tags.divisions,
      filter.tags.offerings
    ]
    .reduce((res, tags) => res.concat(tags), [])
    .toString()
  }

  filter.favorites && (query.favorites = "true");
  filter.charmtalk && (query.charmtalk = "true");
  filter.sweden && (query.sweden = "true");

  router.replace({
    path: "/search",
    query,
  });
}

function beforeCreate() {
  const urlQuery: Route_query = route.query;

  if (Object.keys(urlQuery).length == 0) return;

  filterStore.resetFilter();

  if (typeof urlQuery.q) {
    filter.query = urlQuery.q || "";
  }

  if (urlQuery.tags) {
    let allTags = urlQuery.tags.split(',').map(parseInt)

    filter.tags.divisions = tagsStore
      .getDivisionsFromIds(allTags).map(t => t.id);

    filter.tags.looking_for = tagsStore
      .getLookingForFromIds(allTags).map(t => t.id);

    filter.tags.business_areas = tagsStore
      .getBusinessAreasFromIds(allTags).map(t => t.id);

    filter.tags.languages = tagsStore
      .getLanguagesFromIds(allTags).map(t => t.id);

    filter.tags.offerings = tagsStore
      .getOfferingsFromIds(allTags).map(t => t.id);
  }

  if (urlQuery.favorites) {
    filter.favorites = true;
  }

  if (urlQuery.charmtalk) {
    filter.charmtalk = true;
  }

  if (urlQuery.sweden) {
    filter.sweden = true;
  }

  filterStore.filterCompanies()
    .then(() => filterStore.sortCompanies() )
}

const visibleCards = site_settingsStore.settings.company_view.cards.filter((card) => card.active)

const isVisible = (name: string) => visibleCards.some((c) => c.name === name)
</script>
