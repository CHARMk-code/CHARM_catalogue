<template>
  <div>
    <MenuLeft :links="links" :routepath="routepath" />
    <router-view> </router-view>
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
        { name: "prepages", icon: "mdi-book-open-page-variant" },
        { name: "companies", icon: "mdi-account-group" },
        { name: "Tags", icon: "mdi-tag-heart-outline" },
        { name: "maps", icon: "mdi-map" },
        { name: "layout", icon: "mdi-human-male-board" },
        { name: "batch", icon: "mdi-tray-arrow-up" },
        { name: "account", icon: "mdi-account" },
      ],
      routepath: "/admin/",
    };
  },
};
</script>
