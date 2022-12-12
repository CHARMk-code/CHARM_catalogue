import { defineStore } from "pinia";
import { useCompaniesStore, type Company } from "./companies";
import { useFavoritesStore } from "./favorites";

interface Tags {
  divisions: number[],
  looking_for: number[],
  business_areas: number[],
  offerings: number[],
  languages: number[],
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
  state: () => ({
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
    setFilters(filters: Filters) {
      this.filters = {
        ...this.filters,
        ...filters,
        tags: { ...this.filters.tags, ...filters.tags },
      };
    },
    filterCompanies() {
      return new Promise<void>((resolve) => {
        const companiesStore = useCompaniesStore();
        var filteredCompanies = companiesStore.companies;

        // Filter all non active companies
        filteredCompanies = filteredCompanies.filter((t) => t.active);

        // Filter on query (company name)
        if (this.filters.query != "") {
          filteredCompanies = filteredCompanies.filter((c) =>
            c.name.toLowerCase().includes(this.filters.query.toLowerCase())
          );
        }

        // Filter on tags
        for (const key in Object.keys(this.filters.tags)) {
          const filterTags = this.filters.tags[key];

          if (filterTags.length > 0) {
            filteredCompanies = filteredCompanies.filter((c) => {
              return c.tags.some((t) =>
                filterTags.some((filterTag: number) => t == filterTag)
              );
            });
          }
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

        // Filter on in sweden (no sweden attribute left)
        // if (state.filters.sweden) {
        //   filteredCompanies = filteredCompanies.filter((t: Company) => t.sweden);
        // }
        // state.filteredCompanies = companies;
        resolve();
      });
    },
    sortCompanies(strategy: ((a: any, b: any) => number)) {
      strategy = (a: any, b: any): number => ("" + a.name).localeCompare(b.name);
      this.filteredCompanies.sort(strategy);
    },
  },
  getters: {
    filteredCompanies: (state) => {
      return state.filteredCompanies;
    },
    getFilter: (state) => {
      return state.filters;
    },
  },
});
