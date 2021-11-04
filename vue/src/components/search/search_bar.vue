<template>
  <v-text-field v-model="query" />
</template>

<script>
export default {
  name: "Search_bar",
  model: {
    prop: "query",
    event: "change",
  },
  props: {
    query: { type: String, default: "" },
  },
};
</script>
