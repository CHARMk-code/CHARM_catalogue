<template>
  <div id="app">
    <Header/>
    <router-view/>
  </div>
</template>

<script>
import Header from '@/components/Header'

export default {
  name: 'App',
  components: {
    Header
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
