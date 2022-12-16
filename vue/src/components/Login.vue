<template>
  <v-card class="mx-auto" width="500">
    <v-card-title class="text-center">Login</v-card-title>

    <v-card-text>
      <v-card color="#ffe8e7" v-if="error" class="mb-6">
        <v-card-title>Invalid credentials</v-card-title>
        <v-card-subtitle>Your credentials are invalid.</v-card-subtitle>
      </v-card>

      <v-form
        @submit.prevent="validate"
        ref="form"
        v-model="valid"
        lazy-validation
      >
        <v-text-field
          v-model="password"
          :append-icon="show_pass ? 'mdi-eye' : 'mdi-eye-off'"
          prepend-inner-icon="mdi-lock"
          :type="show_pass ? 'text' : 'password'"
          @click:append="show_pass = !show_pass"
          label="Password"
          required
        ></v-text-field>

        <v-btn
          block
          large
          :disabled="!valid"
          color="primary"
          class="mt-4"
          type="submit"
          :loading="btn_loader"
          required
          >Sign in</v-btn
        >
      </v-form>
    </v-card-text>
  </v-card>
</template>

<script lang="ts" setup>
import { useAuthStore } from '@/stores/modules/auth';
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const authStore = useAuthStore();

const route = useRoute();
const router = useRouter();
const test = () => {
  show_pass.value = !show_pass.value
  console.log(show_pass)
}
let show_pass = ref(false);
let btn_loader = false;
let valid = ref(true);
let email = "";
let password = ref("");
let error = false;



function validate() {
  btn_loader = true;
  authStore.login({ password: password.value })
    .then(() => {
      btn_loader = false;

      if (route.params.nextUrl && !Array.isArray(route.params.nextUrl)) {
        router.push(route.params.nextUrl);
        return;
      }
      router.push("/admin")
    })
    .catch(() => {
      btn_loader = false;
      error = true; // "Invalid sign in credentials!";
    });
}
</script>
