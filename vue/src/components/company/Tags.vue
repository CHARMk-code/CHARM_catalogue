<template>
  <v-card>
    <v-card-title> {{ name }} </v-card-title>
    <v-card-text>
      <div v-if="tags_data.length === 0">No {{ name }}</div>
      <template v-for="tag in tags_data">
        <template v-if="tag.icon == ''">
          <v-chip small class="ma-1" :key="tag.id" @click="gotoSearch(tag)">
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
              @click="gotoSearch(tag)"
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
  methods: {
    gotoSearch(tag) {
      const select_tags = {
        business_areas: [],
        offerings: [],
        looking_for: [],
        languages: [],
        divisions: [],
      };
      if (tag.business_area) {
        select_tags.business_areas.push(tag);
      } else if (tag.offering) {
        select_tags.offerings.push(tag);
      } else if (tag.looking_for) {
        select_tags.looking_for.push(tag);
      } else if (tag.division) {
        select_tags.divisions.push(tag);
      }

      this.$store
        .dispatch("filter/setFilters", {
          query: "",
          charmtalk: false,
          favorites: false,
          sweden: false,
          tags: select_tags,
        })
        .then(() => {
          this.$store.dispatch("filter/filterCompanies");
          this.$router.push("/search");
        });
    },
  },
};
</script>
