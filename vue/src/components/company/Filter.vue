
<template>
  <div>
      <div>
        <template v-if="showFilter">
          Current Filter:
          <q-chip size="sm" class="ma-1" v-if=showQuery() clickable @click=clearQuery()>
            {{filterStore.filters.query}}
          </q-chip>
          <TagGroup :tags="tags" :mutable="true" @removed="update()"></TagGroup>
          <q-chip size="sm" class="ma-1" v-if=filterStore.filters.favorites clickable @click=clearFavorit()>
            Favorites
          </q-chip>
          <q-chip size="sm" class="ma-1" v-if=filterStore.filters.sweden  clickable @click=clearSweden()>
            In Sweden
          </q-chip>
          <q-chip size="sm" class="ma-1" v-if=filterStore.filters.charmtalk  clickable @click=clearCharmtalk()>
            CHARMtalk
          </q-chip>
        </template>
      </div>
  </div>
</template>

<script lang="ts" setup>
import type { Tag } from "@/stores/modules/tags";
import { useFilterStore } from "@/stores/modules/filter";

import { useTagsStore } from "@/stores/modules/tags";
import TagGroup from "../Tag_group.vue";
const filterStore = useFilterStore();

const tagsStore = useTagsStore();

var tags:[Tag] = [];
var showFilter = false;
update();
function update() {
  const grouped_tags = filterStore.filters.tags;
  tags = tagsStore.getTagsFromIds([grouped_tags.business_areas,
  grouped_tags.divisions,
  grouped_tags.fair_areas,
  grouped_tags.languages,
  grouped_tags.looking_for,
  grouped_tags.offerings].flat());
  showFilter =  showQuery()
    || tags.length > 0
    || filterStore.filters.favorites
    || filterStore.filters.sweden
    || filterStore.filters.charmtalk;
}

function showQuery() {
  return filterStore.filters.query != ""
}

function clearQuery() {
  filterStore.filters.query = "";
  filterStore.filterCompanies();
}

function clearFavorit() {
  filterStore.filters.favorites = false;
  filterStore.filterCompanies();
}

function clearSweden() {
    filterStore.filters.sweden = false;
    filterStore.filterCompanies();
}

function clearCharmtalk() {
  filterStore.filters.charmtalk = false;
  filterStore.filterCompanies();
}

function removeTag(tag: Tag){
  this.mutable_tags = this.mutable_tags.filter((t) => t.id != tag.id);

  const pre_tags = filterStore.filters.tags;
  filterStore.filters.tags.divisions = pre_tags.divisions.filter((t) => t != tag.id);
  filterStore.filters.tags.business_areas = pre_tags.business_areas.filter((t) => t != tag.id);
  filterStore.filters.tags.fair_areas = pre_tags.fair_areas.filter((t) => t != tag.id);
  filterStore.filters.tags.offerings = pre_tags.offerings.filter((t) => t != tag.id);
  filterStore.filters.tags.languages = pre_tags.languages.filter((t) => t != tag.id);
  filterStore.filters.tags.looking_for = pre_tags.looking_for.filter((t) => t != tag.id);
  filterStore.filterCompanies();
}
</script>
