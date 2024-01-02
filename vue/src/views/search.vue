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
          v-model:pagination="searchPagination"
          flat
          :rows="filterStore.filteredCompanies"
          :columns="columns"
          wrap-cells
          @row-click="
            (_event, row, _index) => router.push('/company/' + row.name)
          "
        >
          <template #body-cell-Logo="props">
            <q-td :props="props" auto-width>
              <Image
                :ratio="4 / 3"
                :image-name="props.value"
                fit="contain"
                :height="$q.screen.lt.md ? '80px' : '60px'"
                :width="$q.screen.lt.md ? '120px' : '80px'"
              ></Image>
            </q-td>
          </template>

          <template #body-cell-Programs="props">
            <q-td :props="props">
              <TagGroup :tags="props.value"></TagGroup>
            </q-td>
          </template>

          <template #body-cell-Looking_for="props">
            <q-td :props="props">
              <TagGroup :tags="props.value"></TagGroup>
            </q-td>
          </template>

          <template #body-cell-Offering="props">
            <q-td :props="props">
              <TagGroup :tags="props.value"></TagGroup>
            </q-td>
          </template>

          <template #body-cell-Fair_area="props">
            <q-td :props="props">
              <TagGroup :tags="props.value"></TagGroup>
            </q-td>
          </template>

          <template #body-cell-Favorites="props">
            <q-td :props="props" auto-width>
              <q-icon
                v-if="props.value"
                size="sm"
                name="mdi-star"
                color="primary"
                @click.stop="favoritesStore.removeFavorite(props.row.id)"
              ></q-icon>
              <q-icon
                v-else
                size="sm"
                name="mdi-star-outline"
                color="grey"
                @click.stop="favoritesStore.addFavorite(props.row.id)"
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
import { useFilterStore } from "@/stores/modules/filter";
import { useRouter } from "vue-router";
import { useFavoritesStore } from "@/stores/modules/favorites";
import type { Company } from "@/stores/modules/companies";
import { computed } from "vue";
import { useTagsStore } from "@/stores/modules/tags";
import TagGroup from "@/components/Tag_group.vue";
import Image from "@/components/utils/Image.vue";
import { useQuasar } from "quasar";
import { useSite_settingsStore } from "@/stores/modules/site_settings";

const filterStore = useFilterStore();
const favoritesStore = useFavoritesStore();
const tagsStore = useTagsStore();
const siteSettingsStore = useSite_settingsStore();

const searchPagination = computed({
  get() {
    return siteSettingsStore.getTablePagination();
  },
  set(v) {
    siteSettingsStore.session_settings.tables.rowsPerPage = v.rowsPerPage;
  },
});

const $q = useQuasar();
const columns = computed(() => {
  const tempCols = [
    {
      name: "Logo",
      label: "Logo",
      field: "logo",
      align: "left",
      mobile: true,
    },
    { name: "Name", label: "Name", field: "name", align: "left", mobile: true },
    {
      name: "Programs",
      label: "Programs",
      field: (row: Company) => tagsStore.getTagsByCategoryFromIds("division", row.tags),
      align: "left",
    },
    {
      name: "Looking_for",
      label: "Looking for",
      field: (row: Company) => tagsStore.getTagsByCategoryFromIds("looking_for", row.tags),
      align: "left",
    },
    {
      name: "Offering",
      label: "Offering",
      field: (row: Company) => tagsStore.getTagsByCategoryFromIds("offering", row.tags),
      align: "left",
    },
    {
      name: "Fair_area",
      label: "Fair Area",
      field: (row: Company) => tagsStore.getTagsByCategoryFromIds("fair_area", row.tags),
      align: "left",
    },
    {
      name: "Favorites",
      label: "Favorites",
      field: (row: Company) => favoritesStore.favorites.has(row.id),
      align: "center",
      mobile: true,
    },
  ];
  if ($q.screen.lt.md) {
    return tempCols.filter((col) => col.mobile);
  } else {
    return tempCols;
  }
});
const router = useRouter();
</script>
