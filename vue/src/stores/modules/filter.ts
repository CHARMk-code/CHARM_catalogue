import { defineStore } from "pinia";
import type { LocationQuery } from "vue-router";
import { useCompaniesStore, type Company } from "./companies";
import { useFavoritesStore } from "./favorites";
import { useTagsStore } from "./tags";

interface Tags {
  divisions: number[];
  looking_for: number[];
  business_areas: number[];
  offerings: number[];
  languages: number[];
  [key: string]: number[];
}

interface Filters {
  query: string;
  tags: Tags;
  favorites: boolean;
  charmtalk: boolean;
  sweden: boolean;
}

interface State {
  filters: Filters;
  filteredCompanies: Company[];
}

interface Route_query {
  q?: string;
  tags?: string;
  favorites?: string;
  charmtalk?: string;
  sweden?: string;
  [key: string]: string | undefined;
}

export const useFilterStore = defineStore("filter", {
  state: (): State => ({
    filters: {
      query: "",
      tags: {
        divisions: [],
        looking_for: [],
        business_areas: [],
        offerings: [],
        languages: [],
        fair_areas: [],
      },
      favorites: false,
      charmtalk: false,
      sweden: false,
    },
    filteredCompanies: [],
  }),
  actions: {
    resetFilter() {
      this.filters = {
        query: "",
        tags: {
          divisions: [],
          looking_for: [],
          business_areas: [],
          offerings: [],
          languages: [],
          fair_areas: [],
        },
        favorites: false,
        charmtalk: false,
        sweden: false,
      };
    },
    filterCompanies() {
      return new Promise<void>((resolve) => {
        const companiesStore = useCompaniesStore();
        let filteredCompanies = Array.from(companiesStore.companies.values());

        // Filter all non active companies
        filteredCompanies = filteredCompanies.filter((t) => t.active);

        // Filter on query (company name)
        if (this.filters.query != "") {
          filteredCompanies = filteredCompanies.filter((c) =>
            c.name.toLowerCase().includes(this.filters.query.toLowerCase())
          );
        }

        // Filter on tags
        for (var key in this.filters.tags) {
          if (this.filters.tags[key].length == 0) {
            continue;
          }

          filteredCompanies = filteredCompanies.filter((c) => {
            return this.filters.tags[key].some((tag: number) =>
              c.tags.has(tag)
            );
          });
        }

        // Filter on attendance to charmtalks
        if (this.filters.charmtalk) {
          filteredCompanies = filteredCompanies.filter(
            (c: Company) => c.charmtalk
          );
        }

        // Filter on favorites
        if (this.filters.favorites) {
          const favoritesStore = useFavoritesStore();
          filteredCompanies = filteredCompanies.filter((c: Company) =>
            favoritesStore.favorites.has(c.id)
          );
        }
        this.filteredCompanies = filteredCompanies;
        // Filter on in sweden (no sweden attribute left)
        // if (state.filters.sweden) {
        //   filteredCompanies = filteredCompanies.filter((t: Company) => t.sweden);
        // }
        // state.filteredCompanies = companies;

        this.sortCompanies().then(() => {
          resolve();
        });
      });
    },
    sortCompanies() {
      return new Promise<void>((resolve) => {
        const strategy = (a: any, b: any): number =>
          ("" + a.name).localeCompare(b.name);
        this.filteredCompanies.sort(strategy);
        resolve();
      });
    },
    setFiltersFromRouteQuery(rQuery: LocationQuery) {
      this.resetFilter();

      if (rQuery.q && typeof rQuery.q == "string") {
        this.filters.query = rQuery.q;
      }
      if (rQuery.tags && typeof rQuery.tags == "string") {
        const tagsStore = useTagsStore();
        const allTags = new Set(rQuery.tags.split(",").map((s) => parseInt(s)))

        console.log(allTags)
        this.filters.tags.divisions = tagsStore
          .getTagsByCategoryFromIds("Division", allTags)
          .map((t) => t.id);

        this.filters.tags.looking_for = tagsStore
          .getTagsByCategoryFromIds("Looking For",allTags)
          .map((t) => t.id);

        this.filters.tags.business_areas = tagsStore
          .getTagsByCategoryFromIds("Business Area",allTags)
          .map((t) => t.id);

        this.filters.tags.languages = tagsStore
          .getTagsByCategoryFromIds("Language",allTags)
          .map((t) => t.id);

        this.filters.tags.offerings = tagsStore
          .getTagsByCategoryFromIds("Offering",allTags)
          .map((t) => t.id);

        this.filters.tags.fair_areas = tagsStore
          .getTagsByCategoryFromIds("Fair Area",allTags)
          .map((t) => t.id);
      }

      if (rQuery.favorites) {
        this.filters.favorites = true;
      }

      if (rQuery.charmtalk) {
        this.filters.charmtalk = true;
      }

      if (rQuery.sweden) {
        this.filters.sweden = true;
      }
      this.filterCompanies();
    },
    generateSearchRouteQuery() {
      const filter = this.filters;
      const rQuery: LocationQuery = {};

      if (filter.query.length > 0) rQuery.q = filter.query;

      if (
        filter.tags.divisions.length > 0 ||
        filter.tags.business_areas.length > 0 ||
        filter.tags.looking_for.length > 0 ||
        filter.tags.languages.length > 0 ||
        filter.tags.offerings.length > 0 ||
        filter.tags.fair_areas.length > 0
      ) {
        rQuery.tags = [
          filter.tags.business_areas,
          filter.tags.looking_for,
          filter.tags.languages,
          filter.tags.divisions,
          filter.tags.offerings,
          filter.tags.fair_areas,
        ]
          .reduce((res, tags) => res.concat(tags), [])
          .toString();
      }

      if (filter.favorites) rQuery.favorites = "true";
      if (filter.charmtalk) rQuery.charmtalk = "true";
      if (filter.sweden) rQuery.sweden = "true";

      return rQuery;
    },
  },
});
