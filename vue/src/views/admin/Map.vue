<template>
  <v-container>
    <maps_table />
  </v-container>
</template>

<script>
import maps_table from "@/components/admin/maps_table.vue";

export default {
  name: "Maps",
  components: {
    maps_table,
  },
};
</script>
