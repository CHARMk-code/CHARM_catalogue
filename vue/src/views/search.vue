<template>
  <sideLayout>
    <v-container>
      <searchCard />
      <Table
        noSearch="true"
        name="Results"
        :headers="headers"
        :data="filterStore.filteredCompanies"
        :editable="false"
        @click_row="onRowClick"
      >
        <template v-slot:item.tags="{ item }">
          <template v-for="tag in item.tags" :key="tag.id">
            <v-chip class="mr-1 mb-1" x-small>{{
              tag.name
            }}</v-chip>
          </template>
        </template>
        <template v-slot:item.divisions="{ item }">
          <template v-for="tag in item.divisions">
            <template v-if="tag.icon != ''">
              <v-avatar size="24px" class="ma-1" x-small :key="tag.id">
                <v-img
                  max-height="32px"
                  max-width="32px"
                  :src="base_URL + tag.icon"
                />
              </v-avatar>
            </template>
            <template v-else>
              <v-chip small :key="tag.id">
                {{ item.name }}
              </v-chip>
            </template>
          </template>
        </template>
        <template v-slot:item.looking_for="{ item }">
          <template v-for="tag in item.looking_for" :key="tag.id">
            <v-chip class="mr-1 mb-1" x-small>{{
              tag.name
            }}</v-chip>
          </template>
        </template>
        <template v-slot:item.offering="{ item }">
          <template v-for="tag in item.offerings" :key="tag.id">
            <v-chip class="mr-1 mb-1" x-small>{{
              tag.name
            }}</v-chip>
          </template>
        </template>
        <template v-slot:item.business_area="{ item }">
          <template v-for="tag in item.business_area" :key="tag.id">
            <v-chip class="mr-1 mb-1" x-small>{{
              tag.name
            }}</v-chip>
          </template>
        </template>
        <template v-slot:item.id="{ item }">
          <v-checkbox
            :input-value="favoritesStore.favorites.has(item.id)"
            disabled
            on-icon="mdi-star"
            off-icon="mdi-star-outline"
          />
          <!--  TODO: Should make start clickable to changes status-->
        </template>
      </Table>
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

const site_settingsStore = useSite_settingsStore();
const filterStore = useFilterStore();
const favoritesStore = useFavoritesStore();

const base_URL = axios.defaults.baseURL + "/manage/image/";


const visibleCards = site_settingsStore.settings.company_view.cards.filter((card) => card.active)

const isVisible = (name: string) => visibleCards.some((c) => c.name === name)

const headers = [
    isVisible("name") ? { text: "Name", value: "name" } : null,
    isVisible("tag_divisions") ? { text: "Programs", value: "divisions" } : null,
    isVisible("tag_business_areas") ? { text: "Business areas", value: "business_area" } : null,
    isVisible("tag_looking_for") ? { text: "Looking for", value: "looking_for" } : null,
    isVisible("tag_offering") ? { text: "Offering", value: "offering" } : null,
    isVisible("name") ? { text: "Liked", value: "id", width: "80" } : null,
  ].filter(a => a !== null)

const router = useRouter();

function onRowClick(company: Company) {
  router.push("/company/" + company.name);
}
</script>
