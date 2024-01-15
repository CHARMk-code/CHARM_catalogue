<template>
  <q-card v-if="company">
    <q-card-section v-if="company.talk_to_us_about != ''">
      <div class="text-h6">Did you know...</div>
      <div class="q-pb-sm">
        <span class="text-bold">Talk to us about:</span>
        {{ company.talk_to_us_about }}
      </div>
      <div v-if="company.employees_sweden != -1 || company.employees_world != -1">
        <span class="text-bold"> Employees: </span>
        <span v-if="company.employees_sweden != -1">{{ company.employees_sweden }} in Sweden</span>
        <span v-if="company.employees_sweden != -1 && company.employees_world != -1"> and </span>
        <span v-if="company.employees_world != -1">{{ company.employees_world }} in the World</span>.
      </div>
      <div v-if="company.office_location">
          <span class="text-bold"> Office Location</span>
          <span> {{ company.office_location}}</span>
      </div>
      <div v-if="company.male_board_share && company.female_board_share && company.nonbinary_board_share">
        <span class="text-bold"> Gender balance on company board: </span> 
        <span> {{ company.male_board_share }} (Male)</span>
        <span> / </span>
        <span> {{ company.female_board_share }} (Female)</span>
        <span> / </span>
        <span> {{ company.nonbinary_board_share }} (Non-binary)</span>
      </div>
    </q-card-section>
  </q-card>
</template>

<script lang="ts" setup>
import { Company } from "@/stores/modules/companies";

defineProps<{
  company?: Company
}>();
</script>
