<template>
  <company_card_wrapper
    v-if="!(talkToUsAbout == '' && sweden == -1 && world == -1)"
    name="didyouknow"
  >
    <q-card-section v-if="talkToUsAbout != ''">
      <div class="text-h6">Company</div>
      <div class="q-pb-sm">
        {{ talkToUsAbout }}
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
