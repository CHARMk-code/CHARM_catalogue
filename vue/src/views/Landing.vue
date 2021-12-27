<template>
  <sideLayout>
    <container clas="pa-md-4">
      <v-row>
        <v-col
          ><v-container fill-height>
            <v-card class="mx-auto" width="600" center to="/prepages/0">
              <!-- <v-card-title> Look through the catalogue </v-card-title> -->
              <img
                contain
                width="600"
                center
                :src="base_URL + prepages[0].image"
              />
            </v-card>
          </v-container>
        </v-col>
        <v-col>
          <v-container fill-height>
            <v-card
              justify="center"
              class="mx-auto"
              width="500px"
              height="150px"
              to="/search"
            >
              <p class="text-h2" center>Find the company for you!</p>
            </v-card>
          </v-container>
        </v-col>
      </v-row>
    </container>
  </sideLayout>
</template>

<script>
import Vue from "vue";
import { mapGetters } from "vuex";
import sideLayout from "@/views/sideLayout";
export default {
  name: "Landing_view",
  components: {
    sideLayout,
  },
  computed: {
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },
    ...mapGetters({
      prepages: "prepages/getActive",
    }),
  },
  beforeMount() {
    this.$store.dispatch("prepages/getPrepages");
  },
};
</script>
