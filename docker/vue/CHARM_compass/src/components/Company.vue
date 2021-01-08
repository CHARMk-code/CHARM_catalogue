<template>
  <div class='company-container'>
    <div class='company-center'>
      <div class='nav'>
        <router-link class='prev navigation' :to="'/company/' + (parseFloat(this.company) - 1)">
          <font-awesome-icon :icon="['fas', 'arrow-left']" size="3x"/>
        </router-link>
        <router-link class='next navigation' :to="'/company/' + (parseFloat(this.company) + 1)">
          <font-awesome-icon :icon="['fas', 'arrow-left']" size="3x" rotation="180"/>
        </router-link>
      </div>
      <img class='company-page' v-bind:src="require('@/assets/companies/' + this.company + '.png')"/>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Company',
  data () {
    return {
      company: 0
    }
  },
  watch: {
    $route (to, from) {
      console.log(to)
      this.getCompany(to.params.company)
    }
  },
  created () {
    this.getCompany(this.$route.params.company)
    this.$forceUpdate()
  },
  methods: {
    getCompany (company) {
      this.company = company
    }
  }
}
</script>

<style scoped>

.company-container {
  display: flex;
  justify-content: center;
}
.company-center {
  display: flex;
  flex-direction: column;
}
.nav {
  display: flex;
  padding: 25px;
  justify-content: space-evenly;
}
.company-page {
  flex-shrink: 3;
}
.navigation {
  flex-shrink: 1;
}

</style>
