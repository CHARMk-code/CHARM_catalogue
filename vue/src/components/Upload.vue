<template>
  <div class="file-upload">
    <v-card v-if="this.feedback">{{ this.feedback }}</v-card>
    <input type="file" @change="onFileChange" />
    <button
      @click="onUploadFile"
      class="upload-button"
      :disabled="!this.selectedFile"
    >
      Upload file
    </button>
  </div>
</template>

<script>
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
div,
button {
  margin-top: 2rem;
}

.upload-button {
  width: 7rem;
  padding: 0.5rem;
  background-color: #278be9;
  color: #fff;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  border-radius: 1rem;
}

.upload-button:disabled {
  background-color: #b3bcc4;
  cursor: no-drop;
}
</style>
