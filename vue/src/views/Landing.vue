<template>
  <sideLayout>
    <v-container>
      <v-row style="justify-content: center">
        <v-col full-width xs="12" md="6">
          <div class="mx-auto mb-4 text-center text-h5 text-md-h3">
            Browse the catalogue
          </div>
          <hoverCard
            class="mx-auto"
            to="/prepages/0"
            :background_image="
              base_URL + (prepages.length === 0 ? '' : prepages[0].image)
            "
          />
        </v-col>
        <v-col xs="12" md="6">
          <div class="mx-auto mb-4 d-block text-center text-h5 text-md-h3">
            Shortcuts
          </div>
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
import sideLayout from "@/views/sideLayout.vue";
import hoverCard from "@/components/landing/hoverCard.vue";
import shortcut from "@/components/landing/shortcut.vue";

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
    };
  },
  computed: {
    base_URL() {
      console.log(this.hover);
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },
    ...mapGetters({
      prepages: "prepages/getActive",
      shortcuts: "shortcuts/get",
    }),
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
