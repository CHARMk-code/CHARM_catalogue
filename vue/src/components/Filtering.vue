<template>
  <v-expansion-panels flat class="pa-0" style="">
    <v-expansion-panel>
      <v-expansion-panel-header> Filters </v-expansion-panel-header>
      <v-expansion-panel-content>
        <v-row justify="center">
          <v-col v-for="prg in programs" :key="prg.id" max-width="48">
            <p style="width: 48px" class="text-center ma-0 pa-0">
              {{ prg.name.toUpperCase() }}
            </p>
            <v-btn-toggle v-model="activeFilters[prg.id]">
              <v-btn class="pa-0" @click="filterCompanies(prg.id)">
                <v-img
                  contain
                  :src="require('@/assets/divisions/' + prg.name + '.svg')"
                  max-height="48"
                  max-width="48"
                  class="pa-0"
                  style="overflow: hidden"
                />
              </v-btn>
            </v-btn-toggle>
          </v-col>
        </v-row>
      </v-expansion-panel-content>
    </v-expansion-panel>
  </v-expansion-panels>
</template>

<script>
export default {
  name: "Filtering",
  data() {
    return {
      programs: [
        { id: 1, name: "a" },
        { id: 2, name: "ae" },
        { id: 3, name: "bt" },
        { id: 4, name: "d" },
        { id: 5, name: "e" },
        { id: 6, name: "f" },
        { id: 7, name: "gs" },
        { id: 8, name: "i" },
        { id: 9, name: "it" },
        { id: 10, name: "k" },
        { id: 11, name: "kf" },
        { id: 12, name: "m" },
        { id: 13, name: "mt" },
        { id: 14, name: "sjo" },
        { id: 15, name: "td" },
        { id: 16, name: "tm" },
        { id: 17, name: "v" },
        { id: 18, name: "z" },
      ],
      activeFilters: [undefined],
    };
  },
  methods: {
    filterCompanies(clicked) {
      if (this.activeFilters[clicked] === 0) {
        this.activeFilters[clicked] = undefined;
      } else {
        this.activeFilters[clicked] = 0;
      }

      if (!this.activeFilters.some((v) => v === 0)) {
        this.$store.commit("defaultFilteredCompanies");
      } else {
        const filteredCompanies = [];

        for (let i = 0; i < this.activeFilters.length; i++) {
          const filter = this.activeFilters[i];

          if (filter === 0) {
            const filterCompanies = this.$store.state.tagCompanies[i];

            filterCompanies.forEach((companyId) => {
              if (
                !filteredCompanies.some(
                  (oldCompany) => oldCompany.id === companyId
                )
              ) {
                filteredCompanies.push(
                  this.$store.getters.getCompanyById(companyId)
                );
              }
            });
          }
        }

        this.$store.commit("setFilteredCompanies", filteredCompanies);
      }
    },
  },
};
</script>

<style scoped>
.v-avatar {
  margin-left: 0;
}
</style>
