<template>
  <q-page padding>
    <tags_table class="q-mb-md"/>
    <tag_categories_table />
  </q-page>
</template>

<script lang="ts" setup>
import tags_table from "@/components/admin/tags_table.vue";
import tag_categories_table from "@/components/admin/tag_categories_table.vue";
</script>
