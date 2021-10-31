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

          <template v-if="col.type == 'image'">
            <v-container :key="col.model">
              <v-row>
                <v-col>
                  <v-img
                    v-if="row[col.model] != undefined"
                    :src="tag_icon_base_url + row[col.model]"
                    max-height="100"
                    max-width="300"
                    contain
                  />
                </v-col>
                <v-col>
                  <template v-if="row[col.model] != ''"> Replace </template>
                  <template v-else> Add </template>
                  {{ col.label }}

                  <v-file-input
                    v-model="files[col.model]"
                    type="file"
                    clearable
                  />
                </v-col>
              </v-row>
            </v-container>
          </template>
        </template>
      </v-form>
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
  data() {
    return {
      tag_icon_base_url: "/api/manage/image/", //Might be a different URL later
      files: {},
      test: [],
    };
  },
  methods: {
    save() {
      console.log("this.row", this.row);
      console.log("logo", this.files);
      console.log("logo_info", this.test);
      this.uploadFiles(this.files).then(() => {
        this.$emit("save_row", this.row);
        this.close();
      });
    },
    uploadFiles(files) {
      const file_models = Object.keys(files);

      return Promise.all(
        Object.values(files).map((f, index) => {
          return new Promise((resolve, reject) => {
            console.log("f", f);
            console.log("name_model", index);
            const formData = new FormData();
            formData.append("file", f);
            this.$axios
              .post("/manage/upload", formData)
              .then((res) => {
                this.row[file_models[index]] = f.name;
                console.log(res.data);
                return resolve(res);
              })
              .catch((err) => {
                console.log(err);
                return reject(err);
              });
          });
        })
      );
    },
    close() {
      this.$emit("close_dialog");
    },
  },
};
</script>
