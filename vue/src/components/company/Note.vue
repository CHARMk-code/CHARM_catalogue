<template>
  <company_card_wrapper name="notes">
    <q-card-section>
      <div class="text-h6">Your Notes</div>
      <q-input v-model="note" filled autogrow />
    </q-card-section>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";
import { useNotesStore } from "@/stores/modules/notes";
import { computed } from "vue";

const props = defineProps<{
  id: number;
}>();

const notesStore = useNotesStore();
const note = computed({
  get() {
    return notesStore.notes[props.id];
  },
  set(v) {
    return notesStore.setNotes({ id: props.id, note: v });
  },
});
</script>
