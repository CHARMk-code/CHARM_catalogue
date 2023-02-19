<template>
  <company_card_wrapper v-if="website != ''" name="website">
    <q-card-section>
      <div class="text-h6">Website</div>
      <div>More information can be found on their website</div>
    </q-card-section>
    <q-card-actions>
      <q-btn color="primary" target="_blank" :href="website">
        Visit their Website
      </q-btn>
    </q-card-actions>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

defineProps<{
  website: string;
}>();
</script>
