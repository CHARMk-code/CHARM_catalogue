<template>
  <v-main>
    <v-card class="mb-8" width="500px" align="center">
      <v-card-title> Upload data or resources </v-card-title>

      <v-card-subtitle v-if="feedback">{{ feedback }}</v-card-subtitle>

      <v-card-text>
        <v-file-input accept="image/*, .xlsx, .zip, .tar.gz"></v-file-input>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>

        <v-btn href="https://drive.google.com/drive/folders/1ARqpngACz8koJlrudFBCM7jHow94vemY" variant="flat">
          Example file
        </v-btn>
        <v-btn @click="onUploadFile" variant="flat" color="primary" :disabled="!selectedFile">
          Upload file
        </v-btn>
      </v-card-actions>
    </v-card>
    <v-card width="500px" align="center">
      <v-card-title> Download snapshot </v-card-title>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn @click="download" variant="flat" color="primary"> download </v-btn>
        <v-spacer></v-spacer>
      </v-card-actions>
    </v-card>
  </v-main>
</template>

<script lang="ts" setup>
import { ref, type Ref } from 'vue';
import axios from '@/plugins/axios'


let selectedFile: Ref<Blob|undefined> = ref()
let feedback = ref("")

function onFileChange(e: any) {
  console.log(e)
  if(e.target && e.target.files) {
    console.log("inside")
    const uploadedFile = e.target.files[0]; // accessing file
    selectedFile = uploadedFile;
  }
}

function onUploadFile() {
  // abort if no file is selected
  if(!selectedFile.value) return;

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
