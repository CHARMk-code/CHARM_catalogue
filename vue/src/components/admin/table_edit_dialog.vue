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

          <template v-if="col.type == 'number'">
            <v-text-field
              :key="col.model"
              v-model.number="row[col.model]"
              :label="col.label"
              :rules="[
                (v) => {
                  Number.isInteger(v);
                },
              ]"
            />
          </template>

          <template v-if="col.type == 'textarea'">
            <v-textarea
              :key="col.model"
              v-model="row[col.model]"
              :label="col.label"
            />
          </template>

          <template v-if="col.type == 'map_select'">
            <v-select
              chips
              :key="col.model"
              v-model="row[col.model]"
              item-text="name"
              item-value="name"
              :items="col.items"
              :label="col.label"
              :hint="col.hint"
            >
            </v-select>
          </template>

          <template v-if="col.type == 'select'">
            <v-select
              multiple
              chips
              :key="col.model"
              v-model="row[col.model]"
              item-text="name"
              item-value="id"
              return-object
              :items="col.items"
              :label="col.label"
              :hint="col.hint"
            >
              <template v-slot:selection="{ item }">
                <template v-if="item.icon != ''">
                  <v-avatar>
                    <v-img
                      max-height="32px"
                      max-width="32px"
                      :src="base_URL + item.icon"
                    />
                  </v-avatar>
                </template>
                <template v-else>
                  <v-chip small>
                    {{ item.name }}
                  </v-chip>
                </template>
              </template>
            </v-select>
          </template>

          <template v-if="col.type == 'image'">
            <v-container :key="col.model">
              <v-row>
                <v-col>
                  <v-img
                    v-if="row[col.model] != undefined"
                    :src="base_URL + row[col.model]"
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
import Vue from "vue";
export default {
  name: "table_edit_dialog",
  props: ["name", "row", "row_meta", "new"],
  computed: {
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },
  },
  data() {
    return {
      files: {},
      test: [],
    };
  },
  methods: {
    save() {
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
