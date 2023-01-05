<template>
  <company_card_wrapper name="website" v-if="website != ''">
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

const props = defineProps<{
  website: string;
}>();
</script>
