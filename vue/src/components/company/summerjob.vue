<template>
  <company_card_wrapper name="summerjob">
    <q-card-section>
      <div class="text-h6">Summer Jobs</div>

      <div>{{ desc }}</div>
    </q-card-section>

    <q-card-actions>
      <q-btn color="primary" target="_blank" :href="link">
        Apply here before: {{ deadline }}
      </q-btn>
    </q-card-actions>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

defineProps<{
  desc: string;
  link: string;
  deadline: string;
}>();
</script>
