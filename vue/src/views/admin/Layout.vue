<template>
  <v-container>
    <layouts_table />
  </v-container>
</template>

<script>
import layouts_table from "@/components/admin/layout_table";

export default {
  name: "Layout",
  components: {
    layouts_table,
  },
};
</script>
