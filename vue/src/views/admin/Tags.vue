<template>
  <v-container>
    <tags_table />
  </v-container>
</template>

<script>
import tags_table from "@/components/admin/tags_table.vue";

export default {
  name: "Tags",
  components: {
    tags_table,
  },
};
</script>
