<template>
  <company_card_wrapper name="notes">
    <v-card-title> Your Notes </v-card-title>
    <v-card-text>
      <v-text-field @change="onChange" v-model="note" @ />
    </v-card-text>
  </company_card_wrapper>
</template>

<script>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

export default {
  name: "Company_note",
  data() {
    return { note: "" };
  },
  props: ["id"],
  components: {
    company_card_wrapper,
  },
  methods: {
    onChange() {
      this.$store.commit("notes/setNotes", { id: this.id, note: this.note });
    },
  },
  watch: {
    id: function (id) {
      try {
        this.note = this.$store.getters["notes/notes"][id];
      } catch (e) {
        this.note;
      }
    },
  },
  beforeMount() {
    this.$store.commit("notes/loadForStorage");
  },
};
</script>
