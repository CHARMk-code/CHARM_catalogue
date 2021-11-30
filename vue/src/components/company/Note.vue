<template>
  <v-card>
    <v-card-title> Your Notes </v-card-title>
    <v-card-text>
      <v-text-field @change="onChange" v-model="note" @ />
    </v-card-text>
  </v-card>
</template>

<script>
export default {
  name: "Company_note",
  data() {
    return { note: "" };
  },
  props: ["id"],
  methods: {
    onChange() {
      this.$store.commit("notes/setNotes", { id: this.id, note: this.note });
    },
  },
  watch: {
    id: function (id) {
      this.note = this.$store.getters["notes/notes"][id];
    },
  },
  beforeMount() {
    this.$store.commit("notes/loadForStorage");
    this.note = this.$store.getters["notes/notes"][this.id];
  },
};
</script>
