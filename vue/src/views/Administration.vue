<template>
  <div>
    <MenuLeft :links="links" :routepath="routepath"/>
    <v-main>
      <router-view> </router-view>
    </v-main>
  </div>
</template>

<script>
import MenuLeft from "@/components/admin/MenuLeft";
// @ is an alias to /src
export default {
  name: "Administration",
  components: { MenuLeft },
  data() {
    return {
      links: [
        { name: "prepages", icon: "mdi-calendar" },
        { name: "companies", icon: "mdi-account-group" },
        { name: "maps", icon: "mdi-account" },
        { name: "layout", icon: "mdi-ticket" },
        { name: "batch", icon: "mdi-ticket" },
        { name: "account", icon: "mdi-ticket" }
      ],
      routepath: "/admin/"
    };
  }
};
</script>
