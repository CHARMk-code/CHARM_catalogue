<template>
  <v-container>
    <v-row>
      <v-spacer></v-spacer>
      <v-col>
        <v-sheet
          min-height="70vh"
          rounded="lg"
          class="d-flex"
          style="position:relative;"
          >

          <router-link class='prev navigation' :to="'/company/' + (parseFloat(this.company) - 1)">
            <v-chip x-large>
              <v-icon x-large>mdi-arrow-left</v-icon>
            </v-chip>
          </router-link>

          <router-link class='next navigation' :to="'/company/' + (parseFloat(this.company) + 1)">
            <v-chip x-large>
              <v-icon x-large>mdi-arrow-right</v-icon>
            </v-chip>
          </router-link>

          <img center class='company-page' v-bind:src="require('@/assets/companies/' + this.company + '.png')"/>
        </v-sheet>
      </v-col>

      <!-- Used for tags later on -->
      <v-col>
        <v-sheet rounded="lg">
          <v-list color="transparent">
            <v-list-item
              v-for="n in 5"
              :key="n"
              link
              >
              <v-list-item-content>
                <v-list-item-title>
                  List Item {{ n }}
                </v-list-item-title>
              </v-list-item-content>
            </v-list-item>

            <v-divider class="my-2"></v-divider>

            <v-list-item
              link
              color="grey lighten-4"
              >
              <v-list-item-content>
                <v-list-item-title>
                  Refresh
                </v-list-item-title>
              </v-list-item-content>
            </v-list-item>
          </v-list>
        </v-sheet>
      </v-col>

    </v-row>
  </v-container>
</template>

<script>
export default {
  name: 'Company',
  data () {
    return {
      company: undefined
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
</style>
