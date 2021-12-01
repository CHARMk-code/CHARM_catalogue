<template>
  <v-card v-if="tags_data.length > 0">
    <v-card-title> {{ name }} </v-card-title>
    <v-card-text>
      <template v-for="tag in tags_data">
        <template v-if="tag.icon == ''">
          <v-chip small class="ma-1" :key="tag.id">
            {{ tag.name }}
          </v-chip>
        </template>
        <template v-if="tag.icon != ''">
          <v-avatar
            :key="tag.id"
            large
            :icon="tag.icon != ''"
            max-height="36px"
            max-width="36px"
          >
            <v-img
              large
              class="pa-0"
              max-height="36px"
              max-width="36px"
              :src="base_URL + tag.icon"
            />
          </v-avatar>
        </template>
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
