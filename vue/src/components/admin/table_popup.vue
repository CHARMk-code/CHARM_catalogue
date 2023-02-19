<template>
  <v-card>
    <v-card-title> Delete {{ title }}</v-card-title>
    <v-card-text>
      Are you sure you want to delete <strong>{{ props.name }} </strong>?
    </v-card-text>
    <v-card-actions>
      <v-btn color="primary" @click="$emit('delete')"> Delete </v-btn>
      <v-btn @click="$emit('closePopup')"> Cancel </v-btn>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts" setup>
const props = defineProps<{
  name: string;
  title: string;
}>();

defineEmits<{
  (e: "delete"): void;
  (e: "close-popup"): void;
}>();
</script>
