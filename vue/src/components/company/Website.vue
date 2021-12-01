<template>
  <v-card v-if="website != ''">
    <v-card-title> Website </v-card-title>
    <v-card-text>
      <a :href="website">
        {{ website }}
      </a>
    </v-card-text>
  </v-card>
</template>

<script>
export default {
  name: "Company_website",
  props: ["website"],
};
</script>
