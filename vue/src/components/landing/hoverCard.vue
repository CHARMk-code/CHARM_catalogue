<template>
  <q-card elevation="4" center :to="to"> </q-card>
</template>

<script lang="ts" setup>
const props = defineProps<{
  to: string;
  background_image: string;
}>();
</script>

<style scoped>
#hovereffect {
  transition: opacity 0.4s ease-in-out;
}

#hovereffect:not(.on-hover) {
  opacity: 0;
}
</style>
