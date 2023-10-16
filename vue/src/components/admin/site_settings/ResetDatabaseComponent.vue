<template>
  <q-card class="q-mt-md">
    <q-card-section class="text-h5">Reset Database</q-card-section>
    <q-card-section >
      <q-btn color="primary" @click="resetDatabase">
         Remove EVERYTHING (more or less)
      </q-btn>
      {{ success }}
      {{ error }}

      </q-card-section>

  </q-card>

</template>

<script setup lang="ts">
import axios from "@/plugins/axios";
import { ref } from "vue";

const error = ref("")
const success = ref("")

function resetDatabase() {
  axios
    .delete("/v2/settings/reset")
    .then((res) => {
      if(res.status >= 400) {
        error.value = res.statusText
      } else {
        success.value = res.statusText
      }
    })
    .catch((err) => {
      error.value = err.response.statusText;
    });
}

</script>
