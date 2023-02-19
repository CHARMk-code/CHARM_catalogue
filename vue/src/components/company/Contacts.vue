<template>
  <company_card_wrapper v-if="contacts != ''" name="contacts">
    <q-card-section>
      <div class="text-h6">Contact information</div>
      <div v-if="contacts.includes('https://')">
        <a :href="contacts"> {{ contacts }} </a>
      </div>
      <div v-else>{{ contacts }} ({{ contactEmail }})</div>
    </q-card-section>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

defineProps<{
  contacts: string;
  contactEmail: string;
}>();
</script>
