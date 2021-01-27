<template>
  <v-container>

    <v-responsive max-width="750" style="margin: auto;">
      <v-sheet rounded class="pt-6 pa-0">
        <v-row>
          <v-col class="ml-6 mr-6">
            <SearchBar style="margin: auto;" :search='search_text'/>
          </v-col>
        </v-row>

        <v-row>
          <v-col>
          <Filtering />
          </v-col>
        </v-row>

      </v-sheet>
    </v-responsive>
    <v-row class="pa-6 justify-content">
      <v-col>
          <v-sheet class="pa-2" rounded>
            <v-banner>Found {{matching_companies.length}} matching results</v-banner>
          <v-list>
            <v-list-item
              v-for="company in matching_companies"
              :key="company.id"
              v-on:click="goto_result(company.id)"
              >
              <v-list-item-title>{{ company.name }}</v-list-item-title>
              <v-list-item-avatar
                v-for="tag in companyTags(company.id)"
                :key="tag.id"
                class="ml-0 d-none d-md-flex"
                >
                <v-img
                  contain
                  :src="require('@/assets/divisions/' + tag.name.toLowerCase() + '.svg')"
                  max-height="48"
                  max-width="48"
                  class="pa-0"
                  style="overflow: hidden;"
                  />
              </v-list-item-avatar>

            </v-list-item>
          </v-list>
          </v-sheet>
      </v-col>

    </v-row>
  </v-container>
</template>

<script>
import SearchBar from '@/components/Search-bar'
import Filtering from '@/components/Filtering'
import {mapGetters as mapSearchGetters, getterTypes} from 'vuex-search'

export default {
  name: 'Search',
  components: {
    SearchBar,
    Filtering
  },
  data () {
    return {
      search_text: '',
      programs: [
        {id: 0, name: 'a'},
        {id: 1, name: 'ae'},
        {id: 2, name: 'bt'},
        {id: 3, name: 'd'},
        {id: 4, name: 'e'},
        {id: 5, name: 'f'},
        {id: 6, name: 'gs'},
        {id: 7, name: 'i'},
        {id: 8, name: 'it'},
        {id: 9, name: 'k'},
        {id: 10, name: 'kf'},
        {id: 11, name: 'm'},
        {id: 12, name: 'mt'},
        {id: 13, name: 'sjo'},
        {id: 14, name: 'td'},
        {id: 15, name: 'tm'},
        {id: 16, name: 'v'},
        {id: 17, name: 'z'}
      ]
    }
  },
  computed: {
    ...mapSearchGetters('companies', {
      search_result_ids: getterTypes.result,
      isLoading: getterTypes.isSearching
    }),
    matching_companies () {
      const actualResult = []
      for (var i = 0; i < this.search_result_ids.length; i++) {
        actualResult.push(this.$store.getters.getCompanyById(this.search_result_ids[i]))
      }
      return actualResult
    }
  },
  methods: {
    goto_result (id) {
      this.$router.push({name: 'Page', params: {page: id}})
    },
    companyTags (id) {
      return this.$store.getters.getTagsByCompanyId(id)
    }

  }
}
</script>

<style scoped>
.v-avatar {
  margin-left: 0;
}
</style>
