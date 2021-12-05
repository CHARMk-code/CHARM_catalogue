<template>
  <v-container>
    <Table
      @save_edit="saveMap"
      @delete_row="deleteMap"
      name="Map"
      :headers="headers"
      :data="maps"
      :row_meta="row_meta"
      :editable="true"
    >
      <template v-slot:item.name="{ item }">
        {{ item.name }}
      </template>
      <template v-slot:item.image="{ item }">
        {{ item.image }}
      </template>
      <template v-slot:item.ref="{ item }">
        {{ item.ref }}
      </template>
    </Table>
  </v-container>
</template>

<script>
import Table from "@/components/table";

export default {
  name: "maps_table",
  components: {
    Table,
  },
  data() {
    return {
      headers: [
        {
          text: "Name",
          value: "name",
        },
        {
          text: "Image",
          value: "image",
        },
        { text: "Refer to", value: "ref" },
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
      ],
    };
  },
  computed: {
    maps: function () {
      let temp_map = this.$store.getters["maps/get"];
      //console.log(temp_map);
      //temp_map.forEach((t) =>
      //  t.ref != null
      //    ? (t.ref = temp_map.filter((x) => x.id == t.ref)[0].name)
      //    : (t.ref = "None")
      //);
      //console.log(temp_map);
      return temp_map;
    },
  },
  methods: {
    saveMap(map) {
      this.$store.dispatch("maps/modifyMap", map);
    },
    deleteMap(map) {
      this.$store.dispatch("maps/deleteMap", map);
    },
  },
};
</script>
