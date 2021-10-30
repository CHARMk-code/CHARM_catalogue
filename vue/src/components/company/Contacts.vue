<template>
  <v-card> Contacts: {{ contacts }} </v-card>
</template>

<script>
export default {
  name: "Company_Contacts",
  props: ["contacts"],
};
</script>
