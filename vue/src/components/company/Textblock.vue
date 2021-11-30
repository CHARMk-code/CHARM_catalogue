<template>
  <v-card>
    <v-card-title>
      {{ title }}
    </v-card-title>
    <v-card-text>
      {{ body }}
    </v-card-text>
  </v-card>
</template>

<script>
export default {
  name: "Company_text",
  props: ["body", "title"],
};
</script>
