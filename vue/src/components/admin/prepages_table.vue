<template>
  <v-container>
    <Table
      @save_edit="savePrepage"
      @delete_row="deletePrepage"
      name="Prepage"
      :headers="headers"
      :data="Array.from(this.prepages)"
      :row_meta="row_meta"
      :editable="true"
    >
      <template v-slot:item.name="{ item }">
        {{ item.name }}
      </template>
      <template v-slot:item.order="{ item }">
        {{ item.order }}
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
import Table from "@/components/table";
import { mapGetters } from "vuex";

export default {
  name: "prepages_table",
  components: {
    Table,
  },
  data() {
    return {
      icon_url: "/api/manage/image/",
      headers: [
        {
          text: "Name",
          value: "name",
        },
        {
          text: "Order",
          value: "order",
        },
        { text: "Active", value: "active" },
        {
          text: "Actions",
          value: "actions",
          width: 100,
          align: "center",
          sortable: false,
        },
      ],
      row_meta: [
        { type: "image", model: "icon", label: "page image" },
        { type: "checkbox", model: "active", label: "Active" },
        { type: "text", model: "order", label: "Order" },
        { type: "text", model: "name", label: "Name", displayname: true },

        //{ type: "file",model: "icon",label: "Tag Icon",},
      ],
    };
  },
  computed: {
    ...mapGetters({ prepages: "prepages/get" }),
  },
  methods: {
    savePrepage(prepage) {
      this.$store.dispatch("prepages/modifyPrepage", prepage);
    },
    deletePrepage(prepage) {
      this.$store.dispatch("prepages/deletePrepage", prepage);
    },
  },
  beforeMount() {
    this.$store.dispatch("prepages/getPrepages");
  },
};
</script>
