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
          :append-icon="show_pass ? 'mdi-eye' : 'mdi-eye-off'"
          prepend-inner-icon="mdi-lock"
          :type="show_pass ? 'text' : 'password'"
          @click:append="show_pass = !show_pass"
          label="Enter Password"
          required
        ></v-text-field>
        <v-text-field
          v-model="password2"
          :rules="[rules.required]"
          :append-icon="show_pass ? 'mdi-eye' : 'mdi-eye-off'"
          prepend-inner-icon="mdi-lock"
          :type="show_pass ? 'text' : 'password'"
          @click:append="show_pass = !show_pass"
          label="Reenter password "
          required
        ></v-text-field>
        <template v-if="password1 != password2"> Passwords doesn't match </template>
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

<script>
// @ is an alias to /src
import Vue from "vue";
export default {
  name: "Account",
  components: {},
  data() {
    return {
      show_pass: false,
      btn_loader: false,
      valid: true,
      password1: "",
      password2: "",
      error: false,
      rules: {
        required: (value) => !!value || "Required",
      },
    };
  },
  methods: {
    update() {
      if (this.password1 == this.password2) {
        this.btn_loader = true;
        Vue.prototype.$axios
          .put("auth", { password: this.password1 })
          .then(() => {
            this.btn_loader = false;
          })
          .catch(() => {
            this.btn_loader = false;
            this.error = true; // "Invalid sign in credentials!";
          });
      }
    },
  },
};
</script>
