<template>
  <v-container>
    <Table
      @save_edit="saveTag"
      name="Tags"
      :headers="headers"
      :data="Array.from(this.tags)"
      :row_meta="row_meta"
      :editable="true"
    >
      <template v-slot:item.business_area="{ item }">
        <v-simple-checkbox disabled v-model="item.business_area" />
      </template>
      <template v-slot:item.division="{ item }">
        <v-simple-checkbox disabled v-model="item.division" />
      </template>
      <template v-slot:item.looking_for="{ item }">
        <v-simple-checkbox disabled v-model="item.looking_for" />
      </template>

      <template v-slot:item.icon="{ item }">
        <v-img
          :src="headers.filter((x) => x.value == 'icon')[0].base_url + item"
        />
      </template>

      <template v-slot:item.actions="{ item }">
        <v-icon class="mr-2" @click="editRow(item)"> mdi-pencil </v-icon>
      </template>
    </Table>
  </v-container>
</template>

<script>
import Table from "@/components/table";
import { mapGetters } from "vuex";

export default {
  name: "tags_table",
  components: {
    Table,
  },
  data() {
    return {
      icon_url: "/manage/image",
      headers: [
        { text: "Name", value: "name" },
        {
          text: "Business Area",
          value: "business_area",
          align: "center",
          width: 100,
        },
        { text: "Division", value: "division", align: "center", width: 120 },
        {
          text: "Looking for",
          value: "looking_for",
          align: "center",
          width: 100,
        },
        { text: "Offering", value: "offering", align: "center", width: 120 },
        //        {text: "Icon", value: "icon", sortable: false},
        {
          text: "Actions",
          value: "actions",
          width: 80,
          align: "center",
          sortable: false,
        },
      ],
      row_meta: [
        { type: "text", model: "name", label: "Tag name" },
        { type: "checkbox", model: "business_area", label: "Business area" },
        { type: "checkbox", model: "division", label: "division" },
        { type: "checkbox", model: "looking_for", label: "Looking for" },
        //{ type: "file",model: "icon",label: "Tag Icon",},
      ],
    };
  },
  computed: {
    ...mapGetters({ tags: "tags/tags" }),
  },
  methods: {
    saveTag(tag) {
      this.$store.dispatch("tags/modifyTag", tag);
    },
  },
};
</script>
