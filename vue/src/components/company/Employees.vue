<template>
  <v-card>
    Employees in Sweden: {{ sweden }} Employees in world: {{ world }}
  </v-card>
</template>

<script>
export default {
  name: "Company_Trivia",
  props: ["sweden", "world"],
};
</script>
