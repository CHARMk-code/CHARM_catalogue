<template>
  <q-card v-if="company == undefined" flat class="q-px-md">
    No company information to display
  </q-card>
  <q-card v-else flat>
    <Name :id="company.id" :name="company.name" />
    <Logo :src="company.logo" />
    <Textblock title="Description" :body="company.description" />
    <q-btn class="q-mx-md" color="primary" :to="'/company/' + company.name"
      >Read More</q-btn
    >
  </q-card>
</template>

<script setup lang="ts">
import Name from "../company/Name.vue";
import { useCompaniesStore } from "@/stores/modules/companies";
import { computed } from "vue";
import Textblock from "../company/Textblock.vue";
import Logo from "../company/Logo.vue";

const props = defineProps<{
  companyId: number;
}>();

const company = computed(() =>
  useCompaniesStore().companies.get(props.companyId)
);
</script>
