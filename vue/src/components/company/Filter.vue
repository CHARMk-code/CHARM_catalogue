<template>
  <div v-if="showFilter" class="full-width">
    <span class="text-bold"> Current Filter: </span>
    <q-chip
      v-if="showQuery"
      icon-right="mdi-close-circle"
      size="sm"
      class="ma-1"
      clickable
      @click="clearQuery()"
    >
      {{ filterStore.filters.query }}
    </q-chip>
    <q-chip
      v-if="filterStore.filters.favorites"
      icon-right="mdi-close-circle"
      size="sm"
      class="ma-1"
      clickable
      @click="clearFavorite()"
    >
      Favorites
    </q-chip>
    <q-chip
      v-if="filterStore.filters.sweden"
      icon-right="mdi-close-circle"
      size="sm"
      class="ma-1"
      clickable
      @click="clearSweden()"
    >
      In Sweden
    </q-chip>
    <q-chip
      v-if="filterStore.filters.charmtalk"
      icon-right="mdi-close-circle"
      size="sm"
      class="ma-1"
      clickable
      @click="clearCharmtalk()"
    >
      CHARMtalk
    </q-chip>
    <TagGroup :tags="tags" :removeable="true" @tag-click="removeTag"></TagGroup>
  </div>
</template>

<script lang="ts" setup>
import type { Tag } from "@/stores/modules/tags";
import { useFilterStore } from "@/stores/modules/filter";
import { ref, computed, type Ref } from "vue";
import { useTagsStore } from "@/stores/modules/tags";
import TagGroup from "../Tag_group.vue";
const filterStore = useFilterStore();

const emit = defineEmits<{
  (e: "filter-changed"): void;
}>();

const tagsStore = useTagsStore();
const grouped_tags = filterStore.filters.tags;
const tags: Ref<Tag[]> = ref(
  tagsStore.getTagsFromIds(
    [
      grouped_tags.business_areas,
      grouped_tags.divisions,
      grouped_tags.fair_areas,
      grouped_tags.languages,
      grouped_tags.looking_for,
      grouped_tags.offerings,
    ].flat()
  )
);

const showQuery = computed(() => filterStore.filters.query != "");
const showFilter = computed(
  () =>
    showQuery.value ||
    tags.value.length > 0 ||
    filterStore.filters.favorites ||
    filterStore.filters.sweden ||
    filterStore.filters.charmtalk
);

function removeTag(tag: Tag) {
  tags.value = tags.value.filter((t) => t.id != tag.id);

  const pre_tags = filterStore.filters.tags;
  filterStore.filters.tags.divisions = pre_tags.divisions.filter(
    (t) => t != tag.id
  );
  filterStore.filters.tags.business_areas = pre_tags.business_areas.filter(
    (t) => t != tag.id
  );
  filterStore.filters.tags.fair_areas = pre_tags.fair_areas.filter(
    (t) => t != tag.id
  );
  filterStore.filters.tags.offerings = pre_tags.offerings.filter(
    (t) => t != tag.id
  );
  filterStore.filters.tags.languages = pre_tags.languages.filter(
    (t) => t != tag.id
  );
  filterStore.filters.tags.looking_for = pre_tags.looking_for.filter(
    (t) => t != tag.id
  );
  filterStore.filterCompanies();
  emit("filter-changed");
}

function clearQuery() {
  filterStore.filters.query = "";
  filterStore.filterCompanies();
  emit("filter-changed");
}

function clearFavorite() {
  filterStore.filters.favorites = false;
  filterStore.filterCompanies();
  emit("filter-changed");
}

function clearSweden() {
  filterStore.filters.sweden = false;
  filterStore.filterCompanies();
  emit("filter-changed");
}

function clearCharmtalk() {
  filterStore.filters.charmtalk = false;
  filterStore.filterCompanies();
  emit("filter-changed");
}
</script>
