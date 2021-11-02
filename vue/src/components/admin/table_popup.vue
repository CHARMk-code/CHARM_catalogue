<template>
  <v-card>
    <v-card-title> Delete {{ this.name.toLowerCase() }} </v-card-title>
    <v-card-text>
      Are you sure you want to delete {{ display_name }}
    </v-card-text>
    <v-card-actions>
      <v-btn color="primary" @click="delete_row()"> Delete </v-btn>

      <v-btn @click="close()"> Cancel </v-btn>
    </v-card-actions>
  </v-card>
</template>

<script>
export default {
  name: "table_popup",
  props: ["name", "row", "row_meta"],
  computed: {
    display_name() {
      return this.row[
        this.row_meta.filter((attr) => attr.displayname)[0].model
      ];
    },
  },
  methods: {
    delete_row() {
      this.$emit("delete_row", this.row);
      this.close();
    },
    close() {
      this.$emit("close_popup");
    },
  },
};
</script>
