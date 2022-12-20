<template>
  <v-card>
    <v-card-title>{{
      (newRow ? "Create " : "Update ") + name.toLowerCase()
    }}</v-card-title>
    <v-card-text>
      <v-form>
        <template v-for="col in colMeta">
          <template v-if="col.type == 'checkbox'">
            <v-checkbox
              large
              v-model="row[col.model]"
              :on-icon="col.onIcon"
              :off-icon="col.offIcon"
              :label="col.label"
            />
          </template>

          <template v-if="col.type == 'text'">
            <v-text-field v-model="row[col.model]" :label="col.label" />
          </template>

          <template v-if="col.type == 'number'">
            <v-text-field v-model.number="row[col.model]" :label="col.label" />
          </template>

          <template v-if="col.type == 'textarea'">
            <v-textarea v-model="row[col.model]" :label="col.label" />
          </template>

          <template v-if="col.type == 'single-select'">
            <v-autocomplete
              v-model="row[col.model]"
              :items="col.items"
              :label="col.label"
              :hint="col.hint"
            >
            </v-autocomplete>
          </template>

          <template v-if="col.type == 'multiple-select'">
            <v-autocomplete
              multiple
              closable-chips
              chips
              :key="col.model"
              v-model="row[col.model]"
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
            </v-autocomplete>
          </template>

          <template v-if="col.type == 'radio'">
            <v-radio-group v-model="row[col.model]">
              <v-radio
                v-for="radios in col.items"
                :label="radios.title"
                :value="radios.value"
              />
            </v-radio-group>
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
      <v-btn variant="flat" color="primary" @click="save()"> Save </v-btn>

      <v-btn @click="close()"> Cancel </v-btn>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts" setup>
import type { TableRow } from "@/components/table.vue";
import axios from "@/plugins/axios";
import { getTransitionRawChildren, ref, type Ref } from "vue";

export interface TableColMeta {
  type:
    | "checkbox"
    | "text"
    | "number"
    | "textarea"
    | "single-select"
    | "multiple-select"
    | "radio"
    | "image";
  model: string;
  label: string;
  onIcon?: string;
  offIcon?: string;
  items?: { title: string; value: string | number }[];
  hint?: string;
}

const base_URL = axios.defaults.baseURL + "/manage/image/";

const props = defineProps<{
  name: string;
  row: TableRow;
  colMeta: TableColMeta[];
  newRow: boolean;
}>();

const emit = defineEmits<{
  (e: "saveRow", row: TableRow): void;
  (e: "closeDialog"): void;
}>();

const files: Ref<{ [key: string]: File[] }> = ref({});

async function save() {
  Promise.all(
    props.colMeta
      .filter((col) => col.type === "image")
      .map((col) => {
        console.log("col", col);
        return Promise.all(
          files.value[col.model].map((file: File) => {
            const formData = new FormData();
            formData.append("file", file);
            return axios
              .post("/manage/upload", formData)
              .then((res) => {
                console.log(props.row);
                props.row[col.model] = file.name;
              })
              .catch((err) => {});
          })
        );
      })
  ).then(() => {
    console.log("Done", props.row);
    emit("saveRow", props.row);
    close();
  });
}

function close() {
  emit("closeDialog");
}
</script>
