<template>
  <v-card>
    <v-card-title> Website </v-card-title>
    <v-card-text>
      {{ website }}
    </v-card-text>
  </v-card>
</template>

<script>
export default {
  name: "Company_website",
  props: ["website"],
};
</script>
