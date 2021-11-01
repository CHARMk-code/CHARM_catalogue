<template>
  <v-card>
    <v-card-title> Tags </v-card-title>
    <v-card-content>
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
            <v-img
              large
              class="pa-0"
              contain
              :src="'/api/manage/image/' + tag.icon"
            />
          </template>
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
