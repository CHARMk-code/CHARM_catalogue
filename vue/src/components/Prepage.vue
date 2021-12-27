<template>
  <!-- <v-btn -->
  <!--   class="prev navigation" -->
  <!--   v-on:click="prev()" -->
  <!--   v-if="page > 0" -->
  <!--   icon -->
  <!-- > -->
  <!--   <v-chip x-large> -->
  <!--     <v-icon x-large>mdi-arrow-left</v-icon> -->
  <!--   </v-chip> -->
  <!-- </v-btn> -->
  <!--  -->
  <!-- <v-btn class="next navigation" v-on:click="next()" icon> -->
  <!--   <v-chip x-large> -->
  <!--     <v-icon x-large>mdi-arrow-right</v-icon> -->
  <!--   </v-chip> -->
  <!-- </v-btn> -->
  <sideLayout>
    <v-container
      v-touch="{
        right: () => prev(),
        left: () => next(),
      }"
    >
      <img
        class="d-flex mx-auto"
        style="position: relative; height: 100vh"
        :src="base_URL + '/manage/image/' + prepages[page].image"
      />
    </v-container>
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
      console.log(next_index);
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

<style scoped>
.navigation {
  text-decoration: none;
  margin: 20px;
  position: absolute;
  top: 50%;
}
.navigation > * {
  top: -50%;
}
.next {
  right: 0%;
}
</style>
