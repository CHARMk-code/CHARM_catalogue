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
import {mapActions as mapSearchActions, actionTypes} from 'vuex-search'

export default {
  name: 'Search-bar',
  prop: {
    search_resource: String,
    goto_bool: {type: Boolean, default: true}
  },
  data () {
    return {
      search_text: ''
    }
  },
  methods: {
    goto_search () {
      if (this.$attrs.goto_bool) {
        this.$router.push({name: 'Search', params: {search: this.search_text}})
      }
    },
    update_search () {
      this.searchCompanies(this.search_text)
      this.searchTags(this.search_text)
    },
    ...mapSearchActions('companies', {
      searchCompanies: actionTypes.search
    }),
    ...mapSearchActions('tags', {
      searchTags: actionTypes.search
    })
  }
}
</script>
