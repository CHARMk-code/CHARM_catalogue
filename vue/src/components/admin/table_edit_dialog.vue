<template>
  <q-card>
    <q-card-section class="text-h5 text-left">{{
      (newRow ? "Create " : "Update ") + name.toLowerCase()
    }}</q-card-section>
    <q-card-section>
      <q-form class="q-gutter-md">
        <template v-for="col in colMeta">
          <template v-if="col.type == 'checkbox'">
            <q-checkbox
              :key="col.model"
              v-model="rawRow[col.model]"
              filled
              large
              :checked-icon="col.onIcon"
              :unchecked-icon="col.offIcon"
              :label="col.label"
            />
          </template>

          <template v-if="col.type == 'text'">
            <q-input
              :key="col.model"
              v-model="rawRow[col.model]"
              filled
              :label="col.label"
            />
          </template>

          <template v-if="col.type == 'number'">
            <q-input
              :key="col.model"
              v-model="rawRow[col.model]"
              filled
              :label="col.label"
            />
          </template>

          <template v-if="col.type == 'icon'">
            <q-input
              :key="col.model"
              v-model="rawRow[col.model]"
              filled
              :label="col.label"
            >
              <template #before>
                <q-icon size="lg" :name="rawRow[col.model]" />
              </template>
            </q-input>
          </template>

          <template v-if="col.type == 'textarea'">
            <q-input
              :key="col.model"
              v-model="rawRow[col.model]"
              filled
              type="textarea"
              :label="col.label"
            />
          </template>

          <template v-if="col.type == 'single-select'">
            <q-select
              :key="col.model"
              v-model="rawRow[col.model]"
              filled
              :options="col.items"
              :label="col.label"
              :hint="col.hint"
            >
              <template #option="{ opt, itemProps }">
                <q-item v-bind="itemProps">
                  <q-item-section
                    v-if="opt.label.icon && opt.label.icon.length > 0"
                    avatar
                  >
                    <Tag_group :tags="[opt.label]"></Tag_group>
                  </q-item-section>
                  <q-item-section> {{ opt.label.name }}</q-item-section>
                </q-item>
              </template>

              <template #selected-item="{ opt }">
                <Tag_group :tags="[opt.label]"></Tag_group>
              </template>
            </q-select>
          </template>

          <template v-if="col.type == 'multiple-select'">
            <q-select
              :key="col.model"
              v-model="rawRow[col.model]"
              filled
              multiple
              :options="col.items"
              :label="col.label"
              :hint="col.hint"
            >
              <template #option="{ opt, itemProps }">
                <q-item v-bind="itemProps">
                  <q-item-section
                    v-if="opt.label.icon && opt.label.icon.length > 0"
                    avatar
                  >
                    <Tag_group :tags="[opt.label]"></Tag_group>
                  </q-item-section>
                  <q-item-section> {{ opt.label.name }}</q-item-section>
                </q-item>
              </template>

              <template #selected-item="{ opt }">
                <Tag_group :tags="[opt.label]"></Tag_group>
              </template>
            </q-select>
          </template>

          <template v-if="col.type == 'radio'">
            <q-option-group
              :key="col.model"
              v-model="rawRow[col.model]"
              filled
              type="radio"
              :options="col.items"
            ></q-option-group>
          </template>

          <template v-if="col.type == 'image'">
            <div :key="col.model" class="row q-col-gutter-md">
              <div class="col-6">
                <Image
                  v-if="rawRow[col.model] != undefined"
                  class="q-mx-md"
                  :image-name="rawRow[col.model]"
                  height="300px"
                  fit="contain"
                >
                </Image>
              </div>
              <div class="col-6">
                <div class="q-ml-md q-mt-md">
                  <template
                    v-if="
                      typeof rawRow[col.model] === 'string' &&
                      rawRow[col.model].length > 0
                    "
                  >
                    Replace
                  </template>
                  <template v-else> Add </template>
                  {{ col.label }}

                  <q-file
                    v-model="files[col.model]"
                    filled
                    accept="image/*"
                    clearable
                    :label="col.label"
                  >
                    <template #prepend>
                      <q-icon name="attach_file" />
                    </template>
                  </q-file>
                </div>
              </div>
            </div>
          </template>
        </template>
      </q-form>
    </q-card-section>
    <q-card-actions>
      <q-btn color="primary" @click="save()"> Save </q-btn>

      <q-btn v-close-popup> Cancel </q-btn>
    </q-card-actions>
  </q-card>
</template>

<script lang="ts" setup>
import type { TableRow } from "@/components/table.vue";
import Tag_group from "@/components/Tag_group.vue";
import axios from "@/plugins/axios";
import { reactive, ref, type Ref } from "vue";
import { deepUnref } from "vue-deepunref";
import Image from "../utils/Image.vue";

export interface TableColMeta {
  type:
    | "checkbox"
    | "text"
    | "number"
    | "icon"
    | "textarea"
    | "single-select"
    | "multiple-select"
    | "radio"
    | "image";
  model: string;
  label: string;
  onIcon?: string;
  offIcon?: string;
  items?: any[];
  hint?: string;
  meta?: boolean;
}

const props = defineProps<{
  name: string;
  row: TableRow;
  metaRow?: any;
  colMeta: TableColMeta[];
  newRow: boolean;
  metaModelCallback?: (meta: any, row: TableRow) => void;
}>();

const rawRow = reactive({ ...deepUnref(props.row), ...props.metaRow });

const emit = defineEmits<{
  (e: "save-row", rawRow: any): void;
}>();

const files: Ref<{ [key: string]: File }> = ref({});

async function save() {
  Promise.all(
    props.colMeta
      .filter((col) => col.type === "image")
      .map((col) => {
        if (!files.value[col.model]) return true;

        const file = files.value[col.model];
        const formData = new FormData();
        formData.append("file", file);
        return axios
          .post("/manage/upload", formData)
          .then(() => {
            rawRow[col.model] = file.name;
          })
          .catch(() => {});
      })
  ).then(() => {
    for (const col of props.colMeta) {
      if (!col.meta && col.model in rawRow) {
        props.row[col.model] = rawRow[col.model];
      }
    }
    if (props.metaModelCallback) props.metaModelCallback(rawRow, props.row);

    emit("save-row", rawRow);
  });
}
</script>
