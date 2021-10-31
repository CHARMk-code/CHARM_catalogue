<template>
  <v-card>
    <v-card-text>
      {{ desc }}
    </v-card-text>
  </v-card>
</template>

<script>
export default {
  name: "Company_description",
  props: ["desc"],
};
</script>
