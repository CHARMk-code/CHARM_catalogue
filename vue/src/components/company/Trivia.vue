<template>
  <company_card_wrapper
    name="didyouknow"
    v-if="!(talk_to_us_about == '' && sweden == -1 && world == -1)"
  >
    <q-card-section v-if="talk_to_us_about != ''">
      <div class="text-h6">Did you know...</div>
      <div class="q-pb-sm">
        <span class="text-bold">Talk to us about:</span>
        {{ talk_to_us_about }}
      </div>
      <div v-if="sweden != -1 || world != -1">
        <span class="text-bold"> Employees: </span>
        <span v-if="sweden != -1">{{ sweden }} in Sweden</span>
        <span v-if="sweden != -1 && world != -1"> and </span>
        <span v-if="world != -1">{{ world }} in the world</span>.
      </div>
    </q-card-section>
    <q-card-section v-if="world != -1"> </q-card-section>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

const props = defineProps<{
  talk_to_us_about: string;
  sweden: number;
  world: number;
}>();
</script>
