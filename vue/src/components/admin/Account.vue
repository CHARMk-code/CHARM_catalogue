<template>
  <q-page padding>
    <q-card style="width: 500px" class="q-mt-lg q-mx-auto">
      <q-card-section class="text-center text-h5">
        Change password
      </q-card-section>
      <q-form @submit.prevent="update">
        <q-card-section>
          <q-input
            class="q-pa-md"
            filled
            v-model="password1"
            :type="show_pass1 ? 'text' : 'password'"
            label="Enter new password"
            :error="password1.length < 1"
            error-message="You must enter a password"
            required
          >
            <template #append>
              <q-checkbox
                v-model="show_pass1"
                checked-icon="mdi-eye"
                unchecked-icon="mdi-eye-off"
              >
              </q-checkbox>
            </template>
          </q-input>
          <q-input
            class="q-pa-md"
            filled
            v-model="password2"
            :error="password1 != password2"
            error-message="Passwords doesn't match"
            :type="show_pass2 ? 'text' : 'password'"
            @click:append="show_pass2 = !show_pass2"
            label="Enter new password again"
            required
          >
            <template #append>
              <q-checkbox
                v-model="show_pass2"
                checked-icon="mdi-eye"
                unchecked-icon="mdi-eye-off"
              >
              </q-checkbox>
            </template>
          </q-input>
        </q-card-section>
        <q-card-actions :align="'center'">
          <q-btn
            block
            large
            :disabled="!valid || password1 != password2"
            color="primary"
            class="mt-4"
            type="submit"
            :loading="btn_loader"
            required
            >Change Password</q-btn
          >
        </q-card-actions>
      </q-form>
    </q-card>
  </q-page>
</template>

<script lang="ts" setup>
import { useAuthStore } from "@/stores/modules/auth";
import { ref } from "vue";

let btn_loader = false;
let valid = ref(true);
let show_pass1 = ref(false);
let show_pass2 = ref(false);
let password1 = ref("");
let password2 = ref("");
let error = false;

const rules = {
  required: (value: any) => !!value || "Required",
};

function update() {
  if (password1.value == password2.value) {
    btn_loader = true;

    useAuthStore()
      .changePass(password1.value)
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
