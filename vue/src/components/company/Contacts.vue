<template>
  <company_card_wrapper name="contacts" v-if="contacts != ''">
    <q-card-section>
      <div class="text-h6">Contact information</div>
      <div v-if="contacts.includes('https://')">
        <a :href="contacts"> {{ contacts }} </a>
      </div>
      <div v-else>{{ contacts }} ({{ contact_email }})</div>
    </q-card-section>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

const props = defineProps<{
  contacts: string;
  contact_email: string;
}>();
</script>
