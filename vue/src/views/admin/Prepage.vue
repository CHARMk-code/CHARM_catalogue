<template>
  <v-container>
    <prepages_table />
  </v-container>
</template>

<script>
import prepages_table from "@/components/admin/prepages_table";

export default {
  name: "Prepages",
  components: {
    prepages_table,
  },
};
</script>
