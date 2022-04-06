<template>
  <v-data-table
    :headers="headers"
    :items="data"
    class="elevation-1"
    :search="search"
    @click:row="onRowClick"
    v-on:pagination="pageChange"
    fluid
    item-key="id"
    multi-sort
    fixed-header
    :page="page"
  >
    <template v-slot:top>
      <v-toolbar flat>
        <v-toolbar-title>{{ name }}</v-toolbar-title>
        <v-spacer />
        <v-text-field
          v-if="!noSearch"
          v-model="search"
          label="search"
          class="mx-4"
        ></v-text-field>
        <v-spacer />

        <v-dialog v-if="editable" persistent v-model="dialog" max-width="900px">
          <template v-slot:activator="{ on, attrs }">
            <v-btn color="primary" dark class="mb-2" v-bind="attrs" v-on="on">
              Create new
            </v-btn>
          </template>

          <tableEditDialog
            @close_dialog="closeDialog()"
            @save_row="saveRow"
            :name="name"
            :new="creating"
            :row="editedRow"
            :row_meta="row_meta"
          />
        </v-dialog>
        <v-dialog persistent v-model="show_popup" max-width="500px">
          <tablePop
            @close_popup="closePop()"
            @delete_row="deleteRow"
            :row="editedRow"
            :row_meta="row_meta"
            :name="name"
          />
        </v-dialog>
      </v-toolbar>
    </template>

    <template v-if="editable" v-slot:item.actions="{ item }">
      <v-icon class="mr-2" @click="editRow(item)"> mdi-pencil </v-icon>
      <v-icon class="mr-2" @click="openPop(item)">mdi-delete</v-icon>
      <slot name="extra_actions" :item="item"></slot>
    </template>

    <template
      v-for="field in Object.keys($scopedSlots)"
      v-slot:[field]="{ item }"
    >
      <slot :name="field" :item="item" />
    </template>
  </v-data-table>
</template>

<script>
import tableEditDialog from "@/components/admin/table_edit_dialog";
import tablePop from "@/components/admin/table_popup";

import { mapGetters } from "vuex";
export default {
  name: "Table",
  components: {
    tableEditDialog,
    tablePop,
  },
  props: ["editable", "name", "headers", "data", "row_meta", "noSearch"],
  data() {
    return {
      search: "",
      expanded: [],
      dialog: false,
      creating: true,
      editedRow: {},
      show_popup: false,
      page:1
    };
  },
  computed: {
    ...mapGetters({
      getUrlQuery: "filter/getUrlQuery"
    })
  },
  methods: {
    editRow(row) {
      this.editedRow = row;
      this.creating = false;
      this.dialog = true;
    },
    closeDialog() {
      this.dialog = false;
      this.creating = true;
      this.editedRow = {};
    },
    saveRow(row) {
      this.$emit("save_edit", row);
    },
    openPop(row) {
      this.show_popup = true;
      this.editedRow = row;
    },
    closePop() {
      this.show_popup = false;
    },
    deleteRow(row) {
      this.$emit("delete_row", row);
    },
    onRowClick(row) {
      this.$emit("click_row", row);
    },
    pageChange(page_data){ 
      let query = {};
      if (this.$route.name == "Search") {
        query = this.getUrlQuery;
      }
      query.page = page_data.page
      this.$router.replace({
        path: "/search",
        query,
      });
    }
  },
  created(){
      const urlQuery = this.$route.query;
      if (typeof urlQuery.page != "undefined") {
        this.page= Number(urlQuery.page);
      }
  }
};
</script>
