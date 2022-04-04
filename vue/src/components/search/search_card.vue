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
              v-if="isVisible('tag_divisions')"
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
              v-if="isVisible('tag_business_areas')"
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
              v-if="isVisible('tag_looking_for')"
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
              v-if="isVisible('tag_offering')"
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
              v-if="isVisible('language')"
              @change="
                (v) => {
                  selected_tags.languages = v;
                  search();
                }
              "
              :tags="tag_languages"
              :selected_tags="selected_tags.languages"
              label="Language"
            />
            <v-row>
              <v-checkbox
                v-if="isVisible('name')"
                class="ml-2 mr-4"
                @change="search()"
                v-model="favorites"
                label="Only Favorites"
              />
              <v-checkbox
                v-if="isVisible('CHARMtalks')"
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
      filter: "filter/getFilter",
      getTagsFromIds: "tags/getTagsFromIds",
      getDivisionsFromIds: "tags/getDivisionsFromIds",
      getBusinessAreasFromIds: "tags/getBusinessAreasFromIds",
      getLookingForFromIds: "tags/getLookingForFromIds",
      getOffersFromIds: "tags/getOffersFromIds",
      getLanguagesFromIds: "tags/getLanguagesFromIds",
      tag_divisions: "tags/divisions",
      tag_business_areas: "tags/business_areas",
      tag_looking_for: "tags/looking_fors",
      tag_offerings: "tags/offers",
      tag_languages: "tags/languages",
      visibleCards: "site_settings/getCompanyCards",
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
          this.$store.dispatch("filter/sortCompanies", "!");
        });
      let query = {};
      this.query.length > 0 && (query.q = this.query);
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
    isVisible(name) {
      return this.visibleCards.some((c) =>
        c.name === name ? c.active : false
      );
    },
  },
  async created() {
    const urlQuery = this.$route.query;
    if (Object.keys(urlQuery).length == 0) return;
    this.clearFilter();
    const newFilter = { tags: {} };
    console.log(urlQuery);
    if (typeof urlQuery.q !== "undefined" && urlQuery.q.length > 0) {
      newFilter.query = urlQuery.q;
    }
    if (
      typeof urlQuery.divisions !== "undefined" &&
      urlQuery.divisions.length > 0
    ) {
      newFilter.tags.divisions = this.getDivisionsFromIds(
        urlQuery.divisions.split(",").map((t) => parseInt(t))
      );
    }
    if (
      typeof urlQuery.looking_for !== "undefined" &&
      urlQuery.looking_for.length > 0
    ) {
      newFilter.tags.looking_for = this.getLookingForFromIds(
        urlQuery.looking_for.split(",").map((t) => parseInt(t))
      );
    }
    if (
      typeof urlQuery.business_areas !== "undefined" &&
      urlQuery.business_areas.length > 0
    ) {
      newFilter.tags.business_areas = this.getBusinessAreasFromIds(
        urlQuery.business_areas.split(",").map((t) => parseInt(t))
      );
    }
    if (
      typeof urlQuery.offerings !== "undefined" &&
      urlQuery.offerings.length > 0
    ) {
      newFilter.tags.offerings = this.getOffersFromIds(
        urlQuery.offerings.split(",").map((t) => parseInt(t))
      );
    }
    if (
      typeof urlQuery.languages !== "undefined" &&
      urlQuery.languages.length > 0
    ) {
      newFilter.tags.languages = this.getLanguagesFromIds(
        urlQuery.languages.split(",").map((t) => parseInt(t))
      );
    }
    if (
      typeof urlQuery.favorites !== "undefined" &&
      urlQuery.favorites.length > 0
    ) {
      newFilter.favorites = true;
    }
    if (
      typeof urlQuery.charmtalk !== "undefined" &&
      urlQuery.charmtalk.length > 0
    ) {
      newFilter.charmtalk = true;
    }
    if (typeof urlQuery.sweden !== "undefined" && urlQuery.sweden.length > 0) {
      newFilter.sweden = true;
    }
    this.$store.dispatch("filter/setFilters", newFilter);
    this.$store.dispatch("filter/filterCompanies", newFilter);
    this.$store.dispatch("filter/sortCompanies", "!");

    const stored_filter = this.$store.getters["filter/getFilter"];
    this.query = stored_filter.query;
    this.selected_tags = stored_filter.tags;
    this.favorites = stored_filter.favorites;
    this.charmtalk = stored_filter.charmtalk;
    this.sweden = stored_filter.sweden;

    this.expand =
      Object.values(this.selected_tags).some((v) => v.length > 0) ||
      this.favorites ||
      this.charmtalk ||
      this.sweden;
  },
};
</script>
