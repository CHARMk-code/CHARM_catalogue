<template>
  <v-card>
    <v-card-title> Tags </v-card-title>
    <v-card-content>
      <template v-for="tag in tags_data">
        <v-btn :key="tag.id" disabled rounded class="ma-1">
          {{ tag.name }}
        </v-btn>
      </template>
    </v-card-content>
  </v-card>
</template>

<script>
export default {
  name: "Company_Tags",
  props: ["tags"],
  computed: {
    tags_data() {
      return this.tags.map((id) =>
        this.$store.getters["tags/getTagFromId"](id)
      );
    },
  },
};
</script>
