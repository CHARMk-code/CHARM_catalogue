<template>
  <sideLayout>
    <v-container>
      <v-card>
        <searchCard />
        <v-card-subtitle>Results</v-card-subtitle>
        <Table
          name="Results"
          :tableColumns="headers"
          :rows="filterStore.filteredCompanies"
          :editable="false"
          :colMeta="[]"
          @clickRow="onRowClick"
        >
          <template #col(divisions)="{ item }">
            <div class="ma-1">
              <TagGroup
                :tags="tagsStore.getDivisionsFromIds(item.tags)"
              ></TagGroup>
            </div>
          </template>

          <template #col(business_area)="{ item }">
            <TagGroup :tags="tagsStore.getBusinessAreasFromIds(item.tags)">
            </TagGroup>
          </template>

          <template #col(looking_for)="{ item }">
            <TagGroup
              :tags="tagsStore.getLookingForFromIds(item.tags)"
            ></TagGroup>
          </template>

          <template #col(offering)="{ item }">
            <TagGroup
              :tags="tagsStore.getOfferingsFromIds(item.tags)"
            ></TagGroup>
          </template>

          <template #col(liked)="{ item }">
            <v-checkbox
              disabled
              true-icon="mdi-star"
              false-icon="mdi-star-outline"
              :value="favoritesStore.favorites.has(item.id)"
            ></v-checkbox>
          </template>
          <!--
            <template
            <template
            v-slot:item.id="{ item }"
          >
            <v-checkbox
              :input-value="favoritesStore.favorites.has(item.id)"
              disabled
              on-icon="mdi-star"
              off-icon="mdi-star-outline"
            />
          </template>
          -->
        </Table>
      </v-card>
    </v-container>
  </sideLayout>
</template>

<script lang="ts" setup>
import sideLayout from "@/views/sideLayout.vue";
import searchCard from "@/components/search/search_card.vue";
import Table from "@/components/table.vue";
import axios from "@/plugins/axios";
import { useSite_settingsStore } from "@/stores/modules/site_settings";
import { useFilterStore } from "@/stores/modules/filter";
import { useRouter } from "vue-router";
import { useFavoritesStore } from "@/stores/modules/favorites";
import type { Company } from "@/stores/modules/companies";
import { computed } from "vue";
import { useTagsStore } from "@/stores/modules/tags";
import TagGroup from "@/components/Tag_group.vue";
import TagComp from "@/components/Tag.vue";

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

const headers = computed(() => {
  const temp = [];
  if (isVisible("name")) temp.push({ name: "Name", value: "name" });
  if (isVisible("tag_divisions"))
    temp.push({ name: "Programs", value: "divisions" });
  if (isVisible("tag_business_areas"))
    temp.push({ name: "Business areas", value: "business_area" });
  if (isVisible("tag_looking_for"))
    temp.push({ name: "Looking for", value: "looking_for" });
  if (isVisible("tag_offering"))
    temp.push({ name: "Offering", value: "offering" });
  if (isVisible("name")) temp.push({ name: "Liked", value: "liked" });
  return temp;
});
const router = useRouter();

function onRowClick(company: Company) {
  router.push("/company/" + company.name);
}

function log(a, b) {
  console.log(a, b);
}
</script>
