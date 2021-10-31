<template>
  <v-card>
    <v-card-title> Did you know...</v-card-title>
    <v-card-text>
      <v-card flat>
        <v-card-title class="text-subtitle-1 ma-0 pa-0"> Trivia: </v-card-title>
        <v-card-text>
          {{ trivia }}
        </v-card-text>
      </v-card>

      <v-card flat>
        <v-card-title class="text-subtitle-1 ma-0 pa-0">
          Employees:
        </v-card-title>
        <v-card-text>
          {{ sweden }} in Sweden <br />
          {{ world }} in the world
        </v-card-text>
      </v-card>

      <v-card flat>
        <v-card-title class="text-subtitle-1 ma-0 pa-0">
          Founded in:
        </v-card-title>
        <v-card-text>
          {{ year }}
        </v-card-text>
      </v-card>
    </v-card-text>
  </v-card>
</template>

<script>
export default {
  name: "Company_Trivia",
  props: ["trivia", "sweden", "world", "year"],
};
</script>
