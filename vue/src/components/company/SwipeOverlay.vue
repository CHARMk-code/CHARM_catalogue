<template>
  <q-dialog v-model="dialog" persistent maximized class="text-white">
    <q-btn v-morph:swipeleft.tween="morphGroup" flat class="transparent">
      <div>
        <q-icon left name="swipe_left" color="white" size="xl" />
        <div class="text-h5">Swipe left</div>
        <div class="text-h5">for next company/page</div>
      </div>
    </q-btn>

    <q-btn v-morph:swiperight.tween="morphGroup" flat class="transparent">
      <div>
        <q-icon left name="swipe_right" color="white" size="xl" />
        <div class="text-h5">Swipe right</div>
        <div class="text-h5">for previous company/page</div>
      </div>
    </q-btn>
  </q-dialog>
</template>

<script setup lang="ts">
import { ref } from "vue";

const dialog = ref(false);
const morphGroup = ref("swipeleft");

localStorage.setItem("swipeInstructions", "true");

console.log("yes");

function next() {
  console.log(morphGroup.value);
  if (morphGroup.value === "swipeleft") {
    console.log("morph");
    morphGroup.value = "swiperight";
  } else {
    morphGroup.value = "none";
    dialog.value = false;
  }
}

window.addEventListener("click", next);
</script>
