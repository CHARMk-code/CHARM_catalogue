
<template>
  <div>
      <div>
        <template v-if="show()">
          Filters:
          <q-chip size="sm" class="ma-1" v-if=showQuery()>
            dasd
          </q-chip>
          <TagGroup :tags="tags"></TagGroup>
          <q-chip size="sm" class="ma-1" v-if=filterStore.filters.favorites>
            Favorites
          </q-chip>
          <q-chip size="sm" class="ma-1" v-if=filterStore.filters.sweden>
            In Sweden
          </q-chip>
          <q-chip size="sm" class="ma-1" v-if=filterStore.filters.sweden>
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
const grouped_tags = filterStore.filters.tags;
const tags:[Tag] = tagsStore.getTagsFromIds([grouped_tags.business_areas,
  grouped_tags.divisions,
  grouped_tags.fair_areas,
  grouped_tags.languages,
  grouped_tags.looking_for,
  grouped_tags.offerings].flat());

function show() {
  return showQuery()
    || tags.length > 0
    || filterStore.filters.favorites
    || filterStore.filters.sweden
    || filterStore.filters.charmtalk;
}

function showQuery() {
  return filterStore.filters.query != ""
}
</script>
