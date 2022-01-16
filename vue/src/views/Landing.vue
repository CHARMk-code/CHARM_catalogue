<template>
  <sideLayout>
    <v-container class="d-flex flex-wrap">
      <v-row>
        <v-col class="d-flex flex-wrap" style="gap: 16px">
          <div class="mx-auto d-block text-center text-h3">
            Browse the catalogue
          </div>
          <hoverCard
            class="mx-auto"
            to="/prepages/0"
            :background_image="base_URL + prepages[0].image"
            max_width="400"
          >
            <div class="text-h3 text-center">Start Browsing</div>
          </hoverCard>
        </v-col>
        <v-col class="d-flex flex-wrap" style="gap: 16px">
          <div>
            <v-container class="ma-0 pa-0 d-flex flex-wrap" style="gap: 16px">
              <v-card max-width="200px">
                <v-card-title>
                  Check out companies attending CHARMtalks
                </v-card-title>
              </v-card>
              <v-card max-width="200px">
                <v-card-title>
                  Check out companies offering Summer Jobs
                </v-card-title>
              </v-card>
              <v-card max-width="200px">
                <v-card-title>
                  Check out companies starting with names containing cell
                </v-card-title>
              </v-card>
            </v-container>
          </div>
        </v-col>
      </v-row>
    </v-container>
  </sideLayout>
</template>

<script>
import Vue from "vue";
import { mapGetters } from "vuex";
import sideLayout from "@/views/sideLayout";
import hoverCard from "@/components/hoverCard";

export default {
  name: "Landing_view",
  components: {
    sideLayout,
    hoverCard,
  },
  data() {
    return { hover: false };
  },
  computed: {
    base_URL() {
      console.log(this.hover);
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

<style scoped>
/*.v-card {
  transition: opacity 0.4s ease-in-out;
}

.v-card:not(.on-hover) {
  opacity: 0.4;
}*/
#hovereffect {
  transition: opacity 0.4s ease-in-out;
  opacity: 0.75;
}

#hovereffect:not(.on-hover) {
  opacity: 0;
}
#hovereffect2 {
  transition: opacity 0.4s ease-in-out;
}

#hovereffect2:not(.on-hover) {
  opacity: 0;
}
</style>
