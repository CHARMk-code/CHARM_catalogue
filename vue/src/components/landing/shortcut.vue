<template>
  <v-card :to="link" style="width: 100%">
    <div class="d-flex flex-no-wrap justify-space-between">
      <div>
        <v-card-title>
          <v-icon large left> {{ icon }} </v-icon>
          {{ name }}
        </v-card-title>
        <v-card-text> {{ desc }}</v-card-text>
      </div>
      <v-icon right x-large> mdi-chevron-right</v-icon>
    </div>
  </v-card>
</template>

<script>
export default {
  props: ["icon", "link", "name", "desc"],
};
</script>
