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
          :rules="[rules.required]"
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

<script>
// @ is an alias to /src

export default {
  name: "Login",
  components: {},
  data() {
    return {
      show_pass: false,
      btn_loader: false,
      valid: true,
      email: "",
      password: "",
      error: false,
      rules: {
        required: (value) => !!value || "Required",
      },
    };
  },
  methods: {
    validate() {
      console.log("Validating");
      this.btn_loader = true;
      this.$store
        .dispatch("auth/login", { password: this.password })
        .then(() => {
          this.btn_loader = false;

          if (this.$route.params.nextUrl != null) {
            this.$router.push(this.$route.params.nextUrl);
            return;
          }
          this.$router.push("/admin");
        })
        .catch(() => {
          this.btn_loader = false;
          this.error = true; // "Invalid sign in credentials!";
        });
    },
  },
};
</script>
