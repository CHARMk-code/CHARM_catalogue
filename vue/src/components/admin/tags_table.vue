<template>
  <v-container>
    <Table
      @save_edit="saveTag"
      name="Tags"
      :headers="headers"
      :data="Array.from(this.tags)"
      :row_meta="row_meta"
      :editable="true"
    />
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
      headers: [
        { text: "Name", value: "name" },
        //        {text: "Icon", value: "icon", sortable: false, base_url: "/manage/image/"},
        { text: "Actions", value: "actions", width: 100, sortable: false },
      ],
      row_meta: [
        { type: "text", model: "name", label: "Tag name" },
        {
          type: "file",
          model: "icon",
          label: "Tag Icon",
        },
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
