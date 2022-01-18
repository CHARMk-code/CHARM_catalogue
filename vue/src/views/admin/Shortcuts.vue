<template>
  <v-container>
    <shortcuts_table />
  </v-container>
</template>

<script>
import shortcuts_table from "@/components/admin/shortcuts_table";

export default {
  name: "Shortcuts",
  components: {
    shortcuts_table,
  },
};
</script>
