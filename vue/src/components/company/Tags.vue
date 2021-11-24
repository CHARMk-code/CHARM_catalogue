<template>
  <v-card>
    <v-card-title> {{ name }} </v-card-title>
    <v-card-text>
      <template v-for="tag in tags_data">
        <v-btn
          :icon="tag.icon != ''"
          :key="tag.id"
          disabled
          small
          rounded
          class="ma-1"
        >
          <template v-if="tag.icon == ''">
            {{ tag.name }}
          </template>
          <template>
            <v-img large class="pa-0" contain :src="base_URL + tag.icon" />
          </template>
        </v-btn>
      </template>
    </v-card-text>
  </v-card>
</template>

<script>
import Vue from "vue";
export default {
  name: "Company_Tags",
  props: ["tags", "name", "getter_target"],
  computed: {
    tags_data() {
      return this.$store.getters[this.getter_target](this.tags);
    },
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },
  },
};
</script>
