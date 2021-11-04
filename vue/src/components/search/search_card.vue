<template>
  <v-container>
    <v-row>
      <v-text-field prepend-icon="mdi-magnify" v-model="query" />
    </v-row>
    <v-row>
      <v-spacer />
      <v-col cols="5">
        <tagSelector
          @change="(v) => (selected_tags.divisions = v)"
          :tags="tags.divisions"
          label="Programs"
        />
        <tagSelector
          @change="(v) => (selected_tags.business_areas = v)"
          :tags="tags.business_areas"
          label="Business area"
        />
      </v-col>
      <v-col cols="5">
        <tagSelector
          @change="(v) => (selected_tags.looking_for = v)"
          :tags="tags.looking_for"
          label="Looking for"
        />
        <tagSelector
          @change="(v) => (selected_tags.offering = v)"
          :tags="tags.offering"
          label="Offering"
        />
      </v-col>
      <v-spacer />
    </v-row>
    <v-btn @click="search">Search </v-btn>
  </v-container>
</template>

<script>
import tagSelector from "@/components/search/tag_selector";

import { mapGetters } from "vuex";
export default {
  name: "Filters",
  components: { tagSelector },
  data() {
    return {
      query: "",
      selected_tags: {
        divisions: new Array(),
        looking_for: [],
        business_areas: [],
        offering: [],
      },
    };
  },
  computed: {
    ...mapGetters({
      companies: "companies/companies",
      allTags: "tags/tags",
      filteredCompanies: "filter/getFilteredCompanies",
    }),
    tags() {
      return {
        divisions: this.allTags
          .filter((t) => t.division)
          .map((t) => ({ text: t.name, icon: t.icon, value: t.id })),
        looking_for: this.allTags
          .filter((t) => t.looking_for)
          .map((t) => ({ text: t.name, icon: t.icon, value: t.id })),
        business_areas: this.allTags
          .filter((t) => t.business_area)
          .map((t) => ({ text: t.name, icon: t.icon, value: t.id })),
        offering: this.allTags
          .filter((t) => t.offering)
          .map((t) => ({ text: t.name, icon: t.icon, value: t.id })),
      };
    },
  },
  methods: {
    updateSelected(key, event) {
      console.log(event);
      this.selected_tags[key] = event;
    },
    search() {
      this.$store.dispatch("filter/setFilters", {
        query: this.query,
        tags: this.selected_tags,
      });
      this.$store.dispatch("filter/filterCompanies");
    },
  },
};
</script>
