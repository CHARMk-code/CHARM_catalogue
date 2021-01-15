<template>
  <v-container>
    <v-row>
      <v-col>
        <v-sheet
          min-height="70vh"
          rounded="lg"
          class="d-flex"
          style="position:relative;"
          >

          <router-link class='prev navigation' :to="'/company/' + (parseFloat(this.company.page) - 1)">
            <v-chip x-large>
              <v-icon x-large>mdi-arrow-left</v-icon>
            </v-chip>
          </router-link>

          <router-link class='next navigation' :to="'/company/' + (parseFloat(this.company.page) + 1)">
            <v-chip x-large>
              <v-icon x-large>mdi-arrow-right</v-icon>
            </v-chip>
          </router-link>

          <img center class='company-page' v-bind:src="require('@/assets/companies/' + this.company.page + '.png')"/>
        </v-sheet>
      </v-col>

      <!-- Used for tags later on -->
      <v-col>
        <v-sheet max-width="500" rounded="lg">
          <Tags :companyId="this.company.id"/>
        </v-sheet>
      </v-col>

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
    }
  },
  computed: {
    company () {
      return this.getCompany(this.$route.params.company)
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
