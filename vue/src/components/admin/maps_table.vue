<template>
  <v-container>
    <Table
      @save_edit="saveMap"
      @delete_row="deleteMap"
      name="Map"
      :headers="headers"
      :data="modified_maps"
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
        {{ item.ref == null || item.ref == "No Goto" ? "" : item.ref.name }}
      </template>
    </Table>
  </v-container>
</template>

<script>
import Table from "@/components/table";
import { mapGetters } from "vuex";

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
        { text: "Goto", value: "ref" },
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
      maps: "maps/get",
    }),
    row_meta() {
      return [
        { type: "image", model: "image", label: "Map image" },
        {
          type: "single_select",
          model: "ref",
          label: "Goto",
          items: this.maps.concat([{ name: "No Goto", id: null }]),
        },
        { type: "text", model: "name", label: "Name", displayname: true },
      ];
    },
    modified_maps() {
      return this.maps.map((m) => ({
        ...m,
        ref: m.ref == null ? "" : this.maps.filter((rm) => rm.id == m.ref)[0],
      }));
    },
  },
  methods: {
    saveMap(map) {
      this.$store.dispatch("maps/modifyMap", { ...map, ref: map.ref.id });
    },
    deleteMap(map) {
      this.$store.dispatch("maps/deleteMap", map);
    },
  },
};
</script>
