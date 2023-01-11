<template>
  <q-dialog v-model="dialog">
    <q-card>
      <q-card-section>
        <div class="text-h5">Report a problem or give us your feedback</div>
        <span>
          We appreciate that you report problems and possible improvements of
          the catalogue so that we can improve it for next year.
        </span>
      </q-card-section>
      <q-form @submit="sendFeedback()">
        <q-card-section class="q-gutter-md">
          <q-input
            autofocus
            filled
            label="Title..."
            maxlength="50"
            counter
            v-model="feedback.title"
            required
          />
          <q-input
            autofocus
            filled
            label="Your feedback here..."
            type="textarea"
            maxlength="1000"
            v-model="feedback.text"
            counter
            required
          />
        </q-card-section>
        <q-card-actions>
          <q-btn type="submit" :loading="sendingFeedback" color="primary">
            Send feedback</q-btn
          >
          <q-btn v-close-popup> cancel</q-btn>
        </q-card-actions>
      </q-form>
    </q-card>
  </q-dialog>
</template>

<script lang="ts" setup>
import { useFeedbackStore, type Feedback } from "@/stores/modules/feedback";
import { reactive, ref } from "vue";

const dialog = ref(false);
const sendingFeedback = ref(false);
const feedback: Feedback = reactive({ title: "", text: "", meta: "" });

function sendFeedback() {
  sendingFeedback.value = true;
  useFeedbackStore().sendFeedback(feedback);

  // Code for sending the feedback to us
  dialog.value = false;
}
</script>
