<template>
  <v-card>
    <v-card-title>{{
      (this.new ? "Create " : "Edit ") + this.name.toLowerCase()
    }}</v-card-title>
    <v-card-text>
      <v-form>
        <template v-for="col in row_meta">
          <template v-if="col.type == 'checkbox'">
            <v-checkbox
              large
              :key="col.model"
              v-model="row[col.model]"
              :on-icon="col.on_icon"
              :off-icon="col.off_icon"
              :label="col.label"
            />
          </template>

          <template v-if="col.type == 'text'">
            <v-text-field
              :key="col.model"
              v-model="row[col.model]"
              :label="col.label"
            />
          </template>

          <template v-if="col.type == 'textarea'">
            <v-textarea
              :key="col.model"
              v-model="row[col.model]"
              :label="col.label"
            />
          </template>

          <template v-if="col.type == 'select'">
            <v-select
              multiple
              chips
              :key="col.model"
              v-model="row[col.model]"
              :items="col.items"
              :label="col.label"
              :hint="col.hint"
            />
          </template>
        </template>
      </v-form>
      {{ row }}
    </v-card-text>
    <v-card-actions>
      <v-btn color="primary" @click="save()"> Save </v-btn>

      <v-btn @click="close()"> Cancel </v-btn>
    </v-card-actions>
  </v-card>
</template>

<script>
export default {
  name: "table_edit_dialog",
  props: ["name", "row", "row_meta", "new"],
  methods: {
    save() {
      console.log("this.row", this.row);
      this.$emit("save_row", this.row);
      this.close();
    },
    close() {
      this.$emit("close_dialog");
    },
  },
  data() {
    return {};
  },
};
</script>
