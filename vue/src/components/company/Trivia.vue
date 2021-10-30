<template>
  <v-card>
    {{ trivia }}
  </v-card>
</template>

<script>
export default {
  name: "Company_Trivia",
  props: ["trivia"],
};
</script>
