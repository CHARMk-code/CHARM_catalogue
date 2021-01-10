<template>
  <v-app id="app">
    <Header :search_disabled="search_disabled"/>
    <v-main class="grey lighten-3">
      <router-view/>
    </v-main>
  </v-app>
</template>

<script>
import Header from '@/components/Header'
import {mapActions} from 'vuex'

export default {
  name: 'App',
  components: {
    Header
  },
  watch: {
    $route (to, from) {
      if (to.name === 'Search') {
        this.search_disabled = true
        console.log('true')
      } else {
        this.search_disabled = false
        console.log('false')
      }
    }
  },
  created () {
    this.retrieveCompanies()
    this.retrieveTags()
  },
  data () {
    return {
      search_disabled: false
    }
  },
  methods: {
    ...mapActions([
      'retrieveCompanies',
      'retrieveTags'
    ])
  }
}
</script>

<style>
* {
  font-family: Open Sans Condensed;
}
body {
  margin: 0px;
}
svg {
  color: red;
}
</style>
