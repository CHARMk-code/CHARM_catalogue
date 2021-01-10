<template>
  <v-app-bar app flat>
    <v-container class="py-0 fill-height">
      <img class='logo' src="@/assets/CHARM_logo.png">

      <v-spacer></v-spacer>

      <v-btn
        v-for="link in links"
        :key="link.name"
        text
        >
        <router-link :to="link.route" >{{ link.name }}</router-link>
      </v-btn>

        <v-spacer></v-spacer>

        <SearchBar :goto_bool="true" search_resource="companies" v-if="!search_disabled"/>

    </v-container>
  </v-app-bar>
</template>

<script>
import SearchBar from '@/components/Search-bar'

export default {
  name: 'Header',
  components: {
    SearchBar
  },
  props: {
    search_disabled: {
      type: Boolean,
      default: false
    }
  },
  data () {
    return {
      links: [
        {name: 'Home page', route: '/'},
        {name: 'Catalogue', route: '/company/1'}
      ]
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.header-inner {
  background: black;
  padding-left: 80px;
  padding-right: 80px;
  padding-top: 15px;
}

.logo {
  height: 60px;
  padding: 6.5px 1px;
}

header > div {
  display: flex;
  justify-content: space-between;
}
p {
  color: white;
  font-size: 2em;
  margin: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
