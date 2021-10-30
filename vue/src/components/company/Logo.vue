<template>
  <v-img :src="src_base + src" max-height="150" max-width="500" />
</template>

<script>
export default {
  name: "Company_logo",
  data() {
    return {
      src_base: "/api/manage/image/",
    };
  },
  props: ["src"],
};
</script>
