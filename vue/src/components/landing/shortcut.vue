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

<script lang="ts" setup>
const props = defineProps<{
  icon: string
  link: string
  name: string
  desc: string
}>()

</script>
