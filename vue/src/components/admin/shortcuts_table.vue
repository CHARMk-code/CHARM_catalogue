<template>
  <v-container>
    <Table
      @save_edit="saveShortcut"
      @delete_row="deleteShortcut"
      name="Shortcut"
      :headers="headers"
      :data="modified_shortcuts"
      :row_meta="row_meta"
      :editable="true"
    >
      <template v-slot:item.name="{ item }">
        {{ item.name }}
      </template>
      <template v-slot:item.desc="{ item }">
        {{ item.desc }}
      </template>
      <template v-slot:item.link="{ item }">
        {{ item.link }}
      </template>
      <template v-slot:item.icon="{ item }">
        <v-icon>{{ item.icon }}</v-icon>
      </template>
    </Table>
  </v-container>
</template>

<script>
import Table from "@/components/table";
import { mapGetters } from "vuex";

export default {
  name: "shortcuts_table",
  components: {
    Table,
  },
  data() {
    return {
      headers: [
        { text: "Icon", value: "icon" },
        {
          text: "Title",
          value: "name",
        },
        {
          text: "Description",
          value: "desc",
        },
        { text: "Link", value: "link" },
        {
          text: "Actions",
          value: "actions",
          width: 100,
          align: "center",
          sortable: false,
        },
      ],
    };
  },
  computed: {
    ...mapGetters({
      shortcuts: "shortcuts/get",
    }),
    row_meta() {
      return [
        { type: "text", model: "name", label: "Title" },
        { type: "text", model: "desc", label: "Description" },
        { type: "text", model: "icon", label: "Icon" },
        { type: "text", model: "link", label: "Link" },
      ];
    },
    modified_shortcuts() {
      return this.shortcuts.map((m) => ({
        ...m,
      }));
    },
  },
  methods: {
    saveShortcut(shortcut) {
      this.$store.dispatch("shortcuts/modifyShortcut", {
        ...shortcut,
      });
    },
    deleteShortcut(shortcut) {
      this.$store.dispatch("shortcuts/deleteShortcut", shortcut);
    },
  },
};
</script>
