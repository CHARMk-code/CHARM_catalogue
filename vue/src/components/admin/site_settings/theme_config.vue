<template>
  <q-card>
    <q-card-section class="text-h5">Website Themeing</q-card-section>
    <q-card-section class="row">
      <Image
        class="col-6"
        :image-name="site_settingsStore.settings.theme.logo"
        height="150px"
        fit="contain"
      >
      </Image>
      <q-file
        v-model="uploadedLogo"
        class="col-6 self-center"
        label="Upload new logo"
        filled
        accept="image/*"
      >
        <template #prepend>
          <q-icon name="attach_file" />
        </template>
        <template #after>
          <q-btn color="primary" @click="onSubmit"> Change Logo </q-btn>
        </template>
      </q-file>
    </q-card-section>
    <q-card-section>
      <q-input
        v-model="site_settingsStore.settings.theme.primary"
        label="Primary theme colour"
      >
        <template #after
          ><q-btn color="primary" @click="saveColors"
            >Apply colour</q-btn
          ></template
        >
      </q-input>
    </q-card-section>
  </q-card>
</template>

<script lang="ts" setup>
import { type Ref, ref } from "vue";
import { useSite_settingsStore } from "@/stores/modules/site_settings";
import Image from "@/components/utils/Image.vue";
import axios from "@/plugins/axios";
import { setCssVar } from "quasar";

const site_settingsStore = useSite_settingsStore();

const uploadedLogo: Ref<File | undefined> = ref();
const feedback = ref("");

function onSubmit() {
  console.log("test");
  // abort if no file is selected
  if (!uploadedLogo.value) return;

  const fileName = uploadedLogo.value.name;

  const formData = new FormData();

  formData.append("file", uploadedLogo.value); // appending file

  // sending file to the backend
  axios
    .post("/manage/upload", formData)
    .then((res) => {
      feedback.value = res.data;
      site_settingsStore.settings.theme.logo = fileName;
      site_settingsStore.saveSettings();
    })
    .catch((err) => {
      feedback.value = err.response.data;
    });
}

function saveColors() {
  site_settingsStore.saveSettings();
  setCssVar("primary", site_settingsStore.settings.theme.primary);
}
</script>
