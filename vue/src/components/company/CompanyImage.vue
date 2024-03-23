<template>
  <Image :image-name="image" fit="contain" />
</template>

<script setup lang="ts">
import Image from "../utils/Image.vue";

defineProps<{
  image: string
}>()
</script>
