<template>
  <q-page
    padding
    :class="{
      row: $q.screen.gt.sm,
      column: $q.screen.lt.md,
      'q-col-gutter-md': true,
    }"
  >
    <div class="col-6">
      <q-card>
        <q-card-section class="text-h5">
          Upload data or resources
        </q-card-section>
        <q-card-section>
          Upload a .zip containing a full configuration. Look at the example
          file for instructions on how it should look
        </q-card-section>
        <q-card-section v-if="feedback">
          <span class="text-bold">File Upload result:</span> {{ feedback }}
        </q-card-section>
        <q-form>
          <q-card-section>
            <q-file
              filled
              accept="image/*, .xlsx, .zip, .tar.gz"
              v-model="selectedFile"
            >
              <template v-slot:prepend>
                <q-icon name="attach_file" />
              </template>
            </q-file>
          </q-card-section>
          <q-card-actions :align="'right'">
            <q-btn
              href="https://drive.google.com/drive/folders/1ARqpngACz8koJlrudFBCM7jHow94vemY"
              variant="flat"
            >
              Example file
            </q-btn>
            <q-btn
              @click="onUploadFile"
              variant="flat"
              color="primary"
              :disabled="!selectedFile"
            >
              Upload file
            </q-btn>
          </q-card-actions>
        </q-form>
      </q-card>
    </div>
    <div class="col-6">
      <q-card>
        <q-card-section class="text-h5"> Download snapshot </q-card-section>
        <q-card-section>
          Download a snapshot of the current configuration. The configuration is
          put into a .zip file containing .xlsx with the configuration and all
          uploaded images used. if the .zip file is uploaded the system will be
          restored to the state from when it was downloaded. same state
        </q-card-section>
        <q-card-actions :align="'right'">
          <q-btn @click="download" variant="flat" color="primary">
            Download snapshot
          </q-btn>
        </q-card-actions>
      </q-card>
    </div>
  </q-page>
</template>

<script lang="ts" setup>
import { ref, type Ref } from "vue";
import axios from "@/plugins/axios";

let selectedFile: Ref<File | undefined> = ref();
let feedback = ref("");

function onUploadFile() {
  // abort if no file is selected
  if (!selectedFile.value) return;

  const formData = new FormData();

  formData.append("file", selectedFile.value); // appending file

  // sending file to the backend
  axios
    .post("/manage/upload", formData)
    .then((res) => {
      feedback.value = res.data;
    })
    .catch((err) => {
      console.log(err.response);
      feedback.value = err.response.data;
    });
}

function download() {
  axios({
    url: "/manage/download",
    method: "GET",
    responseType: "blob",
  })
    .then((response) => {
      var fileURL = window.URL.createObjectURL(new Blob([response.data]));
      var fURL = document.createElement("a");

      fURL.href = fileURL;
      fURL.setAttribute("download", "CHARM_catalogue_datadump.zip");
      document.body.appendChild(fURL);

      fURL.click();
    })
    .catch((err) => {
      console.log(err);
    });
}
</script>
