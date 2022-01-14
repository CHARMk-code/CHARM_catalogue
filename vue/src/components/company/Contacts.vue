<template>
  <v-card v-if="contacts != ''">
    <v-card-title> Contact information </v-card-title>
    <v-card-text v-if="contacts.includes('https://')">
      <a :href="contacts"> {{ contacts }} </a>
    </v-card-text>
    <v-card-text v-else>{{ contacts }} ({{ contact_email }}) </v-card-text>
  </v-card>
</template>

<script>
export default {
  name: "Company_Contacts",
  props: ["contacts", "contact_email"],
};
</script>
