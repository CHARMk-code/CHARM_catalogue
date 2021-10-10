<template>
  <v-card>
    <file-upload
      url="http://localhost:5008/api/manage/load"
      accept=".xlsx"
      :thumb-url="thumbUrl"
      :headers="headers"
      @change="onFileChange"
    ></file-upload>
  </v-card>
</template>

<script>
import Vue from "vue";
import FileUpload from "v-file-upload";
Vue.use(FileUpload);

export default {
  name: "Upload",
  data() {
    return {
      url: "localhost:5008/api/manage/load",
      headers: { "access-token": this.$store.auth.token },
      filesUploaded: [],
    };
  },
  methods: {
    thumbUrl(file) {
      return file.myThumbUrlProperty;
    },
    onFileChange(file) {
      // Handle files like:
      this.fileUploaded = file;
    },
  },
};
</script>
