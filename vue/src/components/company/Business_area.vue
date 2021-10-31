<template>
  <v-card>
    <v-card-title> Business areas </v-card-title>
    <v-card-text>
      <v-list>
        <v-list-item v-for="area in areas.split(',')" :key="area">
          <v-list-item-title> {{ area }} </v-list-item-title>
        </v-list-item>
      </v-list>
    </v-card-text>
  </v-card>
</template>

<script>
export default {
  name: "Company_Business_area",
  props: ["areas"],
};
</script>
