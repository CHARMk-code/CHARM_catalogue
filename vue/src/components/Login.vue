<template>
  <q-card style="width: 400px" class="col-3">
    <q-form @submit="onSubmit">
      <q-card-section>
        <div class="text-center text-h5 q-mb-md">Login</div>
        <q-card color="error" v-if="error" class="mb-6">
          <q-card-section>
            <span class="text-bold">Invalid credentials </span>
            Your credentials are invalid
          </q-card-section>
        </q-card>
        <q-input
          filled
          v-model="password"
          :type="show_pass ? 'text' : 'password'"
          label="Password"
          required
        >
          <template #append>
            <q-checkbox
              v-model="show_pass"
              checked-icon="mdi-eye"
              unchecked-icon="mdi-eye-off"
            >
            </q-checkbox>
          </template>
        </q-input>
      </q-card-section>
      <q-card-actions :align="'center'">
        <q-btn color="primary" type="submit" :loading="btn_loader" required
          >Sign in</q-btn
        >
      </q-card-actions>
    </q-form>
  </q-card>
</template>

<script lang="ts" setup>
import { useAuthStore } from "@/stores/modules/auth";
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";

const authStore = useAuthStore();

const route = useRoute();
const router = useRouter();
const test = () => {
  show_pass.value = !show_pass.value;
  console.log(show_pass);
};
let show_pass = ref(false);
let btn_loader = ref(false);
let password = ref("");
let error = ref(false);

function onSubmit() {
  btn_loader.value = true;
  authStore
    .login({ password: password.value })
    .then(() => {
      btn_loader.value = false;

      if (route.params.nextUrl && !Array.isArray(route.params.nextUrl)) {
        router.push(route.params.nextUrl);
        return;
      }
      router.push("/admin");
    })
    .catch(() => {
      btn_loader.value = false;
      error.value = true; // "Invalid sign in credentials!";
    });
}
</script>
