<template>
  <v-container>
    <Table
      @save_edit="saveLayout"
      @delete_row="deleteLayout"
      name="Layout"
      :headers="headers"
      :data="Array.from(this.layouts)"
      :row_meta="row_meta"
      :editable="true"
    >
      <template v-slot:item.placement="{ item }">
        <template v-if="item.placement == 0"> Company page</template>
        <template v-else-if="item.placement == 1"> Page Left </template>
        <template v-else-if="item.placement == 2"> Page Right </template>
      </template>
      <template v-slot:item.active="{ item }">
        <v-simple-checkbox
          disabled
          on-icon="mdi-eye"
          off-icon="mdi-eye-off"
          v-model="item.active"
        ></v-simple-checkbox>
      </template>
    </Table>
  </v-container>
</template>

<script>
import Table from "@/components/table.vue";
import { mapGetters } from "vuex";

export default {
  name: "layouts_table",
  components: {
    Table,
  },
  data() {
    return {
      headers: [
        {
          text: "Placement",
          value: "placement",
        },
        {
          text: "Image",
          value: "image",
        },
        { text: "Active", value: "active" },
        {
          text: "Actions",
          value: "actions",
          width: 130,
          align: "center",
          sortable: false,
        },
      ],
      row_meta: [
        { type: "image", model: "image", label: "image", displayname: true },
        { type: "checkbox", model: "active", label: "Active" },
        {
          type: "radio",
          model: "placement",
          label: "Placement",
          items: [
            { name: "Company page", value: 0 },
            { name: "Page Left", value: 1 },
            { name: "Page Right", value: 2 },
          ],
        },
      ],
    };
  },
  computed: {
    ...mapGetters({ layouts: "layouts/get" }),
  },
  methods: {
    saveLayout(layout) {
      this.$store.dispatch("layouts/modifyLayout", layout);
    },
    deleteLayout(layout) {
      this.$store.dispatch("layouts/deleteLayout", layout);
    },
  },
  beforeMount() {
    this.$store.dispatch("layouts/getLayouts");
  },
};
</script>
