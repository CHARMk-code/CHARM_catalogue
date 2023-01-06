<template>
  <q-page padding class="column q-col-gutter-md">
    <div class="col-12-auto">
      <company_cards />
    </div>
    <div class="col-12-auto">
      <layouts_table />
    </div>
  </q-page>
</template>

<script lang="ts" setup>
import company_cards from "@/components/admin/company_cards.vue";
import layouts_table from "@/components/admin/layout_table.vue";
</script>
