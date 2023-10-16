<template>
  <q-page padding>
    <theme_config />
    <ResetDatabaseComponent/>
  </q-page>
</template>

<script lang="ts" setup>
import theme_config from "@/components/admin/site_settings/theme_config.vue";
import ResetDatabaseComponent from "@/components/admin/site_settings/ResetDatabaseComponent.vue";
</script>
