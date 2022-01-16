<template>
<v-main>
  <v-card width="500px" align="center"> 
    <div center>
    <v-card v-if="this.feedback">{{ this.feedback }}</v-card>
    <input type="file" @change="onFileChange" />
    </div>
    <v-row>
      <v-col>
    <v-btn
      @click="onUploadFile"
      class="primary"
      :disabled="!this.selectedFile"
    >
    
      Upload file
    </v-btn>
    </v-col>
    <v-col>
    <v-btn @click="download" class="primary">
        download
    </v-btn>
    </v-col>
    </v-row>
  </v-card>
</v-main>
</template>

<script>
import Vue from "vue";
export default {
  name: "Upload",
  data() {
    return {
      selectedFile: "",
      feedback: "",
    };
  },
  methods: {
    onFileChange(e) {
      const selectedFile = e.target.files[0]; // accessing file
      this.selectedFile = selectedFile;
    },
    download(){
      this.$axios({
        url: Vue.prototype.$axios.defaults.baseURL+'/manage/download',
        method: 'GET',
        responseType: 'blob',
    }).then((response) => {
      var fileURL = window.URL.createObjectURL(new Blob([response.data]));
      var fURL = document.createElement('a');

      fURL.href = fileURL;
      fURL.setAttribute('download',"CHARM_catalogue_datadump.zip");
      document.body.appendChild(fURL);

      fURL.click();
      }).catch((err) => {
          console.log(err);
        });
    },
    onUploadFile() {
      const formData = new FormData();
      formData.append("file", this.selectedFile); // appending file

      // sending file to the backend
      this.$axios
        .post("/manage/upload", formData)
        .then((res) => {
          this.feedback = res.data;
        })
        .catch((err) => {
          console.log(err.response);
          this.feedback = err.response.data;
        });
    },
  },
};
</script>

<style scoped>
.file-upload {
  box-shadow: 2px 2px 9px 2px #ccc;
  border-radius: 1rem;
  padding: 2rem;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  font-size: 1rem;
  width: 60%;
  margin: 0 auto;
}

input {
  font-size: 0.9rem;
}

input,
v-card,
button {
  margin-top: 2rem;
}

.upload-button {
  width: 8rem;
  padding: 0.5rem;
  color: #fff;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
}

.upload-button:disabled {
  background-color: #b3bcc4;
  cursor: no-drop;
}
</style>
