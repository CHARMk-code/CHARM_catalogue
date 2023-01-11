<template>
  <q-page padding>
    <q-card>
      <q-card-section>
        <span class="block text-h6">Search</span>
        <searchCard />
      </q-card-section>
      <q-card-section>
        <span class="block text-h6">Results</span>
        <q-table
          flat
          :rows="filterStore.filteredCompanies"
          :columns="columns"
          wrap-cells
          @row-click="
            (_event, row, _index) => router.push('/company/' + row.name)
          "
        >
          <template #body-cell-Programs="props">
            <q-td :props="props">
              <TagGroup :tags="props.value"></TagGroup
            ></q-td>
          </template>

          <template #body-cell-Business_areas="props">
            <q-td :props="props">
              <TagGroup :tags="props.value"></TagGroup
            ></q-td>
          </template>

          <template #body-cell-Looking_for="props">
            <q-td :props="props">
              <TagGroup :tags="props.value"></TagGroup
            ></q-td>
          </template>

          <template #body-cell-Offering="props">
            <q-td :props="props">
              <TagGroup :tags="props.value"></TagGroup
            ></q-td>
          </template>

          <template #body-cell-Fair_area="props">
            <q-td :props="props">
              <TagGroup :tags="props.value"></TagGroup
            ></q-td>
          </template>

          <template #body-cell-Favorites="props">
            <q-td :props="props">
              <q-icon
                v-if="props.value"
                size="sm"
                name="mdi-star"
                color="primary"
              ></q-icon>
              <q-icon
                v-else
                size="sm"
                name="mdi-star-outline"
                color="grey"
              ></q-icon>
            </q-td>
          </template>
        </q-table>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script lang="ts" setup>
import searchCard from "@/components/search/search_card.vue";
import axios from "@/plugins/axios";
import { useSite_settingsStore } from "@/stores/modules/site_settings";
import { useFilterStore } from "@/stores/modules/filter";
import { onBeforeRouteUpdate, useRouter } from "vue-router";
import { useFavoritesStore } from "@/stores/modules/favorites";
import type { Company } from "@/stores/modules/companies";
import { computed } from "vue";
import { useTagsStore } from "@/stores/modules/tags";
import TagGroup from "@/components/Tag_group.vue";

const site_settingsStore = useSite_settingsStore();
const filterStore = useFilterStore();
const favoritesStore = useFavoritesStore();
const tagsStore = useTagsStore();

const base_URL = axios.defaults.baseURL + "/manage/image/";

const visibleCards = site_settingsStore.settings.company_view.cards.filter(
  (card) => card.active
);

function isVisible(name: string): boolean {
  return visibleCards.some((c) => c.name === name);
}
const columns = [
  { name: "Name", label: "Name", field: "name", align: "left" },
  {
    name: "Programs",
    label: "Programs",
    field: (row: Company) => tagsStore.getDivisionsFromIds(row.tags),
    align: "left",
  },
  {
    name: "Business_areas",
    label: "Business areas",
    field: (row: Company) => tagsStore.getBusinessAreasFromIds(row.tags),
    align: "left",
  },
  {
    name: "Looking_for",
    label: "Looking for",
    field: (row: Company) => tagsStore.getLookingForFromIds(row.tags),
    align: "left",
  },
  {
    name: "Offering",
    label: "Offering",
    field: (row: Company) => tagsStore.getOfferingsFromIds(row.tags),
    align: "left",
  },
  {
    name: "Fair_area",
    label: "Fair Area",
    field: (row: Company) => tagsStore.getFairAreasFromIds(row.tags),
    align: "left",
  },
  {
    name: "Favorites",
    label: "Favorites",
    field: (row: Company) => favoritesStore.favorites.has(row.id),
    align: "center",
  },
];
const router = useRouter();
</script>
