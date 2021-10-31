<template>
  <v-container>
    <tags_table />
  </v-container>
</template>

<script>
import tags_table from "@/components/admin/tags_table";

export default {
  name: "Tags",
  components: {
    tags_table,
  },
  mounted() {
    this.$store.dispatch("tags/getTags");
  },
};
</script>
