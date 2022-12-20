<template>
  <div>
    <MenuLeft :links="links" baseRoute="/admin/" />
    <router-view> </router-view>
  </div>
</template>

<script lang="ts" setup>
import MenuLeft from "@/components/admin/MenuLeft.vue";

const links: { name: string; icon: string }[] = [
  { name: "prepages", icon: "mdi-book-open-page-variant" },
  { name: "companies", icon: "mdi-account-group" },
  { name: "Tags", icon: "mdi-tag-heart-outline" },
  { name: "maps", icon: "mdi-map" },
  { name: "layout", icon: "mdi-human-male-board" },
  { name: "shortcuts", icon: "mdi-star" },
  { name: "batch", icon: "mdi-tray-arrow-up" },
  { name: "account", icon: "mdi-account" },
];
</script>
