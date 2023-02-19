<template>
  <q-card elevation="4" center :to="to"> </q-card>
</template>

<script lang="ts" setup>
defineProps<{
  to: string;
  backgroundImage: string;
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
