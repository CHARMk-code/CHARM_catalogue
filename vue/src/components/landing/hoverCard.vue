<template>
    <v-card elevation="4" center :to="to">
      <v-img contain center :src="background_image"/>
    </v-card>
</template>

<script lang="ts" setup>
const props = defineProps<{
  to: string
  background_image: string
}>()
</script>

<style scoped>
#hovereffect {
  transition: opacity 0.4s ease-in-out;
}

#hovereffect:not(.on-hover) {
  opacity: 0;
}
</style>
