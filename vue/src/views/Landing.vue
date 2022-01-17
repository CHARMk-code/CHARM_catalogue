<template>
  <sideLayout>
    <v-container class="d-flex flex-wrap">
      <v-row>
        <v-col class="d-flex flex-wrap">
          <div class="mx-auto mb-4 text-center text-h3">
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
        <v-col>
          <div class="mx-auto mb-4 d-block text-center text-h3">Shortcuts</div>
          <v-container class="ma-0 pa-0 d-flex flex-wrap" style="gap: 16px">
            <shortcut
              v-for="shortcut in shortcuts"
              :key="shortcut.name"
              :icon="shortcut.icon"
              :name="shortcut.name"
              :desc="shortcut.desc"
              :link="shortcut.link"
            />
          </v-container>
        </v-col>
      </v-row>
    </v-container>
  </sideLayout>
</template>

<script>
import Vue from "vue";
import { mapGetters } from "vuex";
import sideLayout from "@/views/sideLayout";
import hoverCard from "@/components/landing/hoverCard";
import shortcut from "@/components/landing/shortcut";

export default {
  name: "Landing_view",
  components: {
    sideLayout,
    hoverCard,
    shortcut,
  },
  data() {
    return {
      hover: false,
      shortcuts: [
        {
          name: "CHARMtalks",
          icon: "mdi-domain",
          desc: "Companies part of and attending CHARMtalks",
          link: "/prepages/0",
        },
        {
          name: "A-companies",
          icon: "mdi-domain",
          desc: "Companies with an 'A' in the name",
          link: "/prepages/0",
        },
        {
          name: "Offering Summer jobs",
          icon: "mdi-domain",
          desc: "Checkout companies offering summer jobs",
          link: "/prepages/0",
        },
        {
          name: "Offering Master Thesis work",
          icon: "mdi-domain",
          desc: "Checkout companies offering Master thesis work",
          link: "/prepages/0",
        },
      ],
    };
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
