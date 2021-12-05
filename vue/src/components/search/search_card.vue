<template>
  <v-container>
    <v-sheet fluid elevation="1">
      <v-card-title> Search </v-card-title>

      <v-card-text>
        <v-row class="px-2">
          <v-text-field
            prepend-icon="mdi-magnify"
            v-model="query"
            @input="search"
          />
          <v-btn x-large class="ml-6 mt-2" icon
            ><v-icon large @click="expand = !expand"
              >{{ expand ? "mdi-chevron-up" : "mdi-chevron-down" }}
            </v-icon></v-btn
          >
        </v-row>
        <v-expand-transition>
          <v-row v-show="expand">
            <v-col cols="6">
              <tagSelector
                @change="
                  (v) => {
                    selected_tags.divisions = v;
                    search();
                  }
                "
                :tags="tag_divisions"
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
                label="Business area"
              />
            </v-col>
            <v-col cols="6">
              <tagSelector
                @change="
                  (v) => {
                    selected_tags.looking_for = v;
                    search();
                  }
                "
                :tags="tag_looking_for"
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
                label="Offering"
              />
            </v-col>
            <v-col>
              <v-checkbox
                @change="search()"
                label="Favorites"
                v-model="favorites"
              />
              <v-checkbox
                @change="search()"
                v-model="charmtalk"
                label="On CHARMtalks"
              />
              <v-checkbox
                @change="search()"
                v-model="sweden"
                label="In Sweden"
              />
            </v-col>
            <v-col>
              <tagSelector
                @change="
                  (v) => {
                    selected_tags.languages = v;
                    search();
                  }
                "
                :tags="tag_languages"
                label="Langague"
              />
            </v-col>
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
      this.$store.dispatch("filter/setFilters", {
        query: this.query,
        tags: this.selected_tags,
        favorites: this.favorites,
        charmtalk: this.charmtalk,
        sweden: this.sweden,
      });
      this.$store.dispatch("filter/filterCompanies");
    },
  },
};
</script>
