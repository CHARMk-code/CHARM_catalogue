<template>
  <sideLayout
    v-on:next="next"
    v-on:prev="prev"
    v-bind:button_left="page > 0"
    v-bind:button_right="true"
  >
    <v-sheet
      style="margin-top: -52px; max-height: 80%"
      v-touch="{ right: () => prev(), left: () => next() }"
    >
      <v-img
        class="prepage"
        contain
        max-width="100%"
        :src="base_URL + '/manage/image/' + prepages[page].image"
      />
    </v-sheet>
  </sideLayout>
</template>

<script>
import { mapGetters } from "vuex";
import Vue from "vue";
import sideLayout from "@/views/sideLayout";
export default {
  name: "Prepage",
  components: {
    sideLayout,
  },
  computed: {
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL;
    },
    page() {
      return this.$route.params.page;
    },
    ...mapGetters({
      prepages: "prepages/getActive",
      filteredCompanies: "filter/filteredCompanies",
    }),
  },
  methods: {
    next() {
      if (parseInt(this.page) + 1 >= this.prepages.length) {
        this.$store.dispatch("companies/getCompanies");
        this.$store.dispatch("filter/filterCompanies");
        this.$router.push("/company/" + this.filteredCompanies[0].name);
      } else {
        this.$router.push("/prepages/" + (parseInt(this.page) + 1));
      }
    },
    prev() {
      const next_index = parseInt(this.page) - 1;
      if (next_index >= 0) {
        this.$router.push("/prepages/" + next_index);
      }
    },
  },
  beforeMount() {
    this.$store.dispatch("prepages/getPrepages");
  },
};
</script>

<style lang="scss" scoped>
.prepage {
  max-height: calc(100vh - 64px);
}
</style>
