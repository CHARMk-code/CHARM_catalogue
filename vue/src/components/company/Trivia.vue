<template>
  <company_card_wrapper
    v-if="!(talkToUsAbout == '' && sweden == -1 && world == -1)"
    name="didyouknow"
  >
    <q-card-section v-if="talkToUsAbout != ''">
      <div class="text-h6">Did you know...</div>
      <div class="q-pb-sm">
        {{ talkToUsAbout }}
      </div>
      <div v-if="location" class="q-pb-sm">
          <span class="text-bold"> We have</span> offices in {{ location }}
      </div>
      <div v-if="sweden != -1" class="q-pb-sm">
        <span class="text-bold">We have</span> {{ sweden }} employees in Sweden
      </div>
      <div v-if="world != -1" class="q-pb-sm">
        <span class="text-bold">We have</span> {{ world }} employees worldwide
      </div>
      <div v-if="founded" class="q-pb-sm">
        <span class="text-bold"> We were</span> founded in {{ founded }}
      </div>
    </q-card-section>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

defineProps<{
  talkToUsAbout: string;
  sweden: number;
  world: number;
  founded: number;
  location: string;
}>();
</script>
