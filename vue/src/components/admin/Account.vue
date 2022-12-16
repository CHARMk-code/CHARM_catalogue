<template>
  <v-card max-width="500px" class="mx-auto">
    <v-card-text>
      <v-form
        @submit.prevent="update"
        ref="form"
        v-model="valid"
        lazy-validation
      >
        <v-text-field
          v-model="password1"
          :rules="[rules.required]"
          :append-icon="show_pass1 ? 'mdi-eye' : 'mdi-eye-off'"
          prepend-inner-icon="mdi-lock"
          :type="show_pass1 ? 'text' : 'password'"
          @click:append="show_pass1 = !show_pass1"
          label="Enter Password"
          required
        ></v-text-field>
        <v-text-field
          v-model="password2"
          :rules="[rules.required]"
          :append-icon="show_pass2 ? 'mdi-eye' : 'mdi-eye-off'"
          prepend-inner-icon="mdi-lock"
          :type="show_pass2 ? 'text' : 'password'"
          @click:append="show_pass2 = !show_pass2"
          label="Enter password again"
          required
        ></v-text-field>
        <template v-if="password1 != password2">
          Passwords doesn't match
        </template>
        <v-btn
          block
          large
          :disabled="!valid || password1 != password2"
          color="primary"
          class="mt-4"
          type="submit"
          :loading="btn_loader"
          required
          >Update</v-btn
        >
      </v-form>
    </v-card-text>
  </v-card>
</template>

<script lang="ts" setup>
import { useAuthStore } from "@/stores/modules/auth";
import { ref } from "vue";

let btn_loader = false
let valid = ref(true)
let show_pass1 = ref(false)
let show_pass2 = ref(false)
let password1 = ref("")
let password2 = ref("")
let error = false

const rules = {
        required: (value: any) => !!value || "Required"
      }

function update() {
  if (password1.value == password2.value) {
    btn_loader = true;

    useAuthStore().changePass(password1.value)
      .then(() => {
        btn_loader = false;
      })
      .catch(() => {
        btn_loader = false;
        error = true; // "Invalid sign in credentials!";
      });
  }
}
</script>
