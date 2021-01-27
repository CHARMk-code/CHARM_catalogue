<template>
  <v-container style="margin: auto;">
    <v-row>
      <v-col>
        <v-sheet
          style="margin: auto;"
          max-width="890"
          rounded="lg"
          >
          <v-img
            contain
            class='page'
            v-bind:src="require('@/assets/pages/' + this.page + '.png')"/>
        </v-sheet>
          </v-col>
    </v-row>
    <v-row>
      <v-container
          style="margin: auto;"
          max-width="890"
          rounded="lg"
        >
      <v-col>
        <v-responsive max-width="890" style="margin: auto;">
          <v-row>
            <v-col>
              <v-btn
                height="64"
                v-if="this.prev > 0">
                <router-link
                  style="text-decoration: none; color: inherit;"
                  :to="'/' + this.prev">
                  <v-icon x-large>mdi-arrow-left</v-icon>
                  Previous
                </router-link>
              </v-btn>
            </v-col>

            <v-spacer/>

              <v-col>
                <v-btn
                  height="64"
                  style="float: right;"
                  v-if="this.next <= this.max">
                  <router-link
                    style="text-decoration: none; color: inherit;"
                    :to="'/' + this.next">
                    Next
                    <v-icon x-large>mdi-arrow-right</v-icon>
                  </router-link>
                </v-btn>
              </v-col>

          </v-row>
        </v-responsive>
      </v-col>
      </v-container>
    </v-row>
  </v-container>
</template>

<script>
import Tags from '@/components/Tags'
export default {
  name: 'Company',
  components: {
    Tags
  },
  data () {
    return {
      max: 53
    }
  },
  computed: {
    page () {
      const param = this.$route.params

      if (typeof param === 'undefined') {
        return 1
      } else {
        return param.page
      }
    },
    next () {
      return parseFloat(this.page) + 1
    },
    prev () {
      return parseFloat(this.page) - 1
    }
  },
  methods: {
    getCompany (id) {
      let company = this.$store.getters.getCompanyById(id)

      if (typeof company === 'undefined') {
        company = this.$store.dispatch('retrieveCompanies')
          .then(() => {
            return this.$store.getters.getCompanyById(id)
          })
      }
      return company
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
