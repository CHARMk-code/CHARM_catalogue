<template>
  <v-responsive max-width="360" >
          <v-text-field
            prepend-inner-icon='mdi-magnify'
            dense
            flat
            hide-details
            solo-inverted
            v-on:input="update_search()"
            v-on:keyup.enter="goto_search()"
            v-model="search_text"
            ></v-text-field>

  </v-responsive>
</template>

<script>
import {mapActions as mapSearchActions, mapGetters as mapSearchGetters, getterTypes, actionTypes} from 'vuex-search'

export default {
  name: 'Search-bar',
  data () {
    return {
      search_text: ''
    }
  },
  computed: {
    ...mapSearchGetters('companies', {
      resultIds: getterTypes.result,
      isLoading: getterTypes.isSearching
    })

  },
  created () {
    this.$store.dispatch('retrieveCompanies')
  },
  methods: {
    goto_search () {
      this.$router.push({name: 'Search', params: {search: this.search_text}})
    },
    update_search () {
      console.log(this.search_text)
      this.searchCompanies(this.search_text)
    },
    ...mapSearchActions('companies', {
      searchCompanies: actionTypes.search
    })
  }
}
</script>
