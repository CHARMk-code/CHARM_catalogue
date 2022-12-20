import { defineStore } from "pinia";
import { useCompaniesStore, type Company } from "./companies";
import { useFavoritesStore } from "./favorites";

interface Tags {
  divisions: number[],
  looking_for: number[],
  business_areas: number[],
  offerings: number[],
  languages: number[],
  [key: string]: number[];
}

interface Filters {
  query: string,
  tags: Tags,
  favorites: boolean,
  charmtalk: boolean,
  sweden: boolean,
}

interface State {
  filters: Filters,
  filteredCompanies: Company[],
}

export const useFilterStore = defineStore('filter', {
  state: (): State => ({
    filters: {
      query: "",
      tags: {
        divisions: [],
        looking_for: [],
        business_areas: [],
        offerings: [],
        languages: [],
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
        },
        favorites: false,
        charmtalk: false,
        sweden: false,
      }
      this.filterCompanies()
    },
    filterCompanies() {
      return new Promise<void>((resolve) => {
        const companiesStore = useCompaniesStore();
        var filteredCompanies = Array.from(companiesStore.companies.values());

        // Filter all non active companies
        filteredCompanies = filteredCompanies.filter((t) => t.active);

        // Filter on query (company name)
        if (this.filters.query != "") {
          filteredCompanies = filteredCompanies.filter((c) =>
            c.name.toLowerCase().includes(this.filters.query.toLowerCase())
          );
        }

        // Filter on tags
        var filterTags: number[] = []
        for (var key in this.filters.tags) filterTags = filterTags.concat(this.filters.tags[key]);

        if (filterTags.length > 0) {
          filteredCompanies = filteredCompanies.filter((c) => {
            return filterTags.some((filterTag: number) => c.tags.has(filterTag))
          });
        }

        // Filter on attendance to charmtalks
        if (this.filters.charmtalk) {
          filteredCompanies = filteredCompanies.filter((c: Company) => c.charmtalk);
        }

        // Filter on favorites
        if (this.filters.favorites) {
          const favoritesStore = useFavoritesStore()
          filteredCompanies = filteredCompanies.filter((c: Company) =>
            favoritesStore.favorites.has(c.id)
          );
        }
        this.filteredCompanies = filteredCompanies
        // Filter on in sweden (no sweden attribute left)
        // if (state.filters.sweden) {
        //   filteredCompanies = filteredCompanies.filter((t: Company) => t.sweden);
        // }
        // state.filteredCompanies = companies;
        resolve();
      });
    },
    sortCompanies() {
      const strategy = (a: any, b: any): number => ("" + a.name).localeCompare(b.name);
      this.filteredCompanies.sort(strategy);
    },
  },
});
