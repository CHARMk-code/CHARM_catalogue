<template>
    <v-navigation-drawer app clipped permanent class="elevation-4">
      <v-list-item>
        <v-list-item-content>
          <v-list-item-title class="text-h5">
           Admin 
          </v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-divider></v-divider>

      <v-list
        dense
        nav
      >
        <v-list-item
          v-for="link in links"
          :key="link.name"
          :to="routepath + link.name"
        >
          <v-list-item-icon>
            <v-icon>{{ link.icon }}</v-icon>
          </v-list-item-icon>

          <v-list-item-content>
            <v-list-item-title>{{ link.name}}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>
</template>

<script>
export default {
  name: "MenuLeft",
  components: {},
  props: ["links", "routepath"],
};

</script>

<style scoped>

</style>
