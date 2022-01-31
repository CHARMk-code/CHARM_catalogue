<template>
  <v-main>
    <v-row>
      <v-col>
        <v-card width="500px" align="center">
          <v-card-title> Upload data or resources </v-card-title>

          <div>
            <v-card v-if="this.feedback">{{ this.feedback }}</v-card>
            <v-tooltip bottom max-width="20%">
              <template v-slot:activator="{ on }">
                <div v-on="on">
                  <input type="file" @change="onFileChange" />
                </div>
              </template>
              <span>
                Allows .jpg, .png, .svg, .xlsx, .zip, and .tar.gz. For more
                information please refer to the github.
              </span>
            </v-tooltip>
          </div>

          <v-row class="ma-auto">
            <v-col>
              <v-btn
                @click="onUploadFile"
                class="primary ma-auto"
                :disabled="!this.selectedFile"
              >
                Upload file
              </v-btn>
            </v-col>
            <v-col>
              <v-tooltip bottom max-width="20%">
                <template v-slot:activator="{ on }">
                  <div v-on="on">
                    <v-btn
                      href="https://drive.google.com/drive/folders/1ARqpngACz8koJlrudFBCM7jHow94vemY"
                      class="primary"
                    >
                      See example
                    </v-btn>
                  </div>
                </template>
                <span>
                  We have provide a example of the formating for a complete
                  upload. If you want more information please refer to the
                  github page.
                </span>
              </v-tooltip>
            </v-col>
          </v-row>
        </v-card>
      </v-col>
      <v-col>
        <v-card width="500px" align="center">
          <v-card-title> Download snapshot </v-card-title>
          <v-tooltip bottom max-width="20%">
            <template v-slot:activator="{ on }">
              <div v-on="on">
                <v-btn @click="download" class="primary mb-5"> download </v-btn>
              </div>
            </template>
            <span>
              Downloads all the data need to restore the current state of the
              catalogue, including images and a xlsx similarly to the example.
              Good for when you want a checkpoint before making changes or when
              you want to save the current catalogue for archiving.
            </span>
          </v-tooltip>
        </v-card>
      </v-col>
    </v-row>
  </v-main>
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
    download() {
      this.$axios({
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
    },
    onUploadFile() {
      const formData = new FormData();
      formData.append("file", this.selectedFile); // appending file

      // sending file to the backend
      this.$axios
        .post("/manage/upload", formData)
        .then((res) => {
          window.location.reload();
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
