<template>
  <v-container class="px-0">
    <v-sheet class="pa-0" fluid elevation="1">
      <v-card-title> Search </v-card-title>

      <v-card-text>
        <v-row class="px-2">
          <v-text-field
            prepend-icon="mdi-magnify"
            v-model="query"
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
              @change="
                (v) => {
                  selected_tags.divisions = v;
                  search();
                }
              "
              :tags="tag_divisions"
              :selected_tags="selected_tags.divisions"
              label="Programs"
            />
            <tagSelector
              @change="
                (v) => {
                  selected_tags.business_areas = v;
                  search();
                }
              "
              :tags="tag_business_areas"
              :selected_tags="selected_tags.business_areas"
              label="Business area"
            />
            <tagSelector
              @change="
                (v) => {
                  selected_tags.looking_for = v;
                  search();
                }
              "
              :tags="tag_looking_for"
              :selected_tags="selected_tags.looking_for"
              label="Looking for"
            />
            <tagSelector
              @change="
                (v) => {
                  selected_tags.offerings = v;
                  search();
                }
              "
              :tags="tag_offerings"
              :selected_tags="selected_tags.offerings"
              label="Offering"
            />
            <tagSelector
              @change="
                (v) => {
                  selected_tags.languages = v;
                  search();
                }
              "
              :tags="tag_languages"
              :selected_tags="selected_tags.languages"
              label="Langague"
            />
            <v-row>
              <v-checkbox
                class="ml-2 mr-4"
                @change="search()"
                v-model="favorites"
                label="Only Favorites"
              />
              <v-checkbox
                class="ml-2 mr-4"
                @change="search()"
                v-model="charmtalk"
                label="Participating in CHARMtalks"
              />
            </v-row>
            <!--
                <v-checkbox
                @change="search()"
                v-model="sweden"
                label="In Sweden"
              />
              -->
            <v-btn @click="clearFilter"> Clear filter </v-btn>
          </v-row>
        </v-expand-transition>
      </v-card-text>
    </v-sheet>
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
      expand: false,
      query: "",
      selected_tags: {
        divisions: [],
        looking_for: [],
        business_areas: [],
        offerings: [],
        languages: [],
      },
      favorites: false,
      charmtalk: false,
      sweden: false,
    };
  },
  computed: {
    ...mapGetters({
      companies: "companies/companies",
      allTags: "tags/tags",
      filteredCompanies: "filter/getFilteredCompanies",
      tag_divisions: "tags/divisions",
      tag_business_areas: "tags/business_areas",
      tag_looking_for: "tags/looking_fors",
      tag_offerings: "tags/offers",
      tag_languages: "tags/languages",
    }),
    tags() {
      return {
        divisions: this.tag_divisions,
        looking_for: this.tag_looking_for,
        business_areas: this.tag_business_areas,
        offerings: this.tag_offerings,
        languages: this.tag_languages,
      };
    },
  },
  methods: {
    updateSelected(key, event) {
      this.selected_tags[key] = event;
    },
    search() {
      this.$store
        .dispatch("filter/setFilters", {
          query: this.query,
          tags: this.selected_tags,
          favorites: this.favorites,
          charmtalk: this.charmtalk,
          sweden: this.sweden,
        })
        .then(() => {
          this.$store.dispatch("filter/filterCompanies");
        });
      let query = {};
      this.query.length > 0 ? (query.q = this.query) : null;
      if (this.selected_tags.divisions.length > 0) {
        query.divisions = this.selected_tags.divisions
          .map((t) => t.id.toString())
          .toString();
      }
      if (this.selected_tags.looking_for.length > 0) {
        query.looking_for = this.selected_tags.looking_for
          .map((t) => t.id.toString())
          .toString();
      }
      if (this.selected_tags.business_areas.length > 0) {
        query.business_areas = this.selected_tags.business_areas
          .map((t) => t.id.toString())
          .toString();
      }
      if (this.selected_tags.offerings.length > 0) {
        query.offerings = this.selected_tags.offerings
          .map((t) => t.id.toString())
          .toString();
      }
      if (this.selected_tags.languages.length > 0) {
        query.languages = this.selected_tags.languages
          .map((t) => t.id.toString())
          .toString();
      }
      this.favorites && (query.favorites = true);
      this.charmtalk && (query.charmtalk = true);
      this.sweden && (query.sweden = true);

      this.$router.replace({
        path: "/search",
        query,
      });
    },
    clearFilter() {
      this.query = "";
      this.selected_tags.divisions = [];
      this.selected_tags.business_areas = [];
      this.selected_tags.offerings = [];
      this.selected_tags.looking_for = [];
      this.selected_tags.languages = [];
      this.favorites = false;
      this.charmtalk = false;
      this.search();
    },
  },
  mounted() {
    const stored_filter = this.$store.getters["filter/getFilter"];
    this.query = stored_filter.query;
    this.selected_tags = stored_filter.tags;
    this.favorites = stored_filter.favorites;
    this.charmtalk = stored_filter.charmtalk;
    this.sweden = stored_filter.sweden;
  },
};
</script>
