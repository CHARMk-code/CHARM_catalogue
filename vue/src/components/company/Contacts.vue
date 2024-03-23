<template>
  <q-card>
    <q-card-section>
        <div class="text-h6">Contact</div>
          
        <div v-if="!company.contacts.includes('https://')">
            {{ company.contacts }} ({{ company.contact_email }})
        </div>
    </q-card-section>
    <q-card-actions>
      <q-btn color="primary" target="_blank" :href="company.website">
        Visit their Website
      </q-btn>
    </q-card-actions>
  </q-card>
</template>

<script lang="ts" setup>
import { Company } from "@/stores/modules/companies";

defineProps<{
  company?: Company
}>();
</script>
