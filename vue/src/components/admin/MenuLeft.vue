<template>
  <v-navigation-drawer app clipped permanent class="elevation-4">
    <v-list-item>
      <v-list-item-title class="text-h5"> Admin </v-list-item-title>
    </v-list-item>

    <v-divider></v-divider>

    <v-list dense nav>
      <v-list-item
        v-for="link in links"
        :key="link.name"
        :to="props.baseRoute + link.name"
        :prepend-icon="link.icon"
      >
        <v-list-item-title>{{ link.name }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script lang="ts" setup>
const props = defineProps<{
  links: {
    name: string;
    icon: string;
  }[];
  baseRoute: string;
}>();
</script>

<style scoped></style>
