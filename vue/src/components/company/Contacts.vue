<template>
  <v-card v-if="contacts != ''">
    <v-card-title> Contact information </v-card-title>
    <v-card-text>{{ contacts }} </v-card-text>
  </v-card>
</template>

<script>
export default {
  name: "Company_Contacts",
  props: ["contacts"],
};
</script>
