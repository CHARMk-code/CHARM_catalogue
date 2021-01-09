<template>
  <v-container>
    <v-row class="pa-6 justify-center">
      <v-responsive max-width="550" class="d-flex justify-center">
        <v-sheet rounded class="pa-6">
          <v-row>
            <v-col cols="9" >
              <SearchBar :search='search_text'/>
            </v-col>
            <v-col cols="1">
              <v-btn>Search
              </v-btn>
            </v-col>
          </v-row>
        </v-sheet>
      </v-responsive>
    </v-row>
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

            </v-list-item>
          </v-list>
          </v-sheet>
      </v-col>

    </v-row>
  </v-container>
</template>

<script>
import SearchBar from '@/components/Search-bar'
import {mapGetters as mapSearchGetters, getterTypes} from 'vuex-search'

export default {
  name: 'Search',
  components: {
    SearchBar
  },
  data () {
    return {
      search_text: ''
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
      this.$router.push({name: 'Company', params: {company: id}})
    }

  }
}
</script>

<style scoped>
.navigation {
  text-decoration: none;
  margin: 20px;
  position:absolute;
  top: 50%;
}

.navigation > * {
  top: -50%;
}

.next {
  right: 0%;
}
<
