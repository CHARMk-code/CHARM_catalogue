<template>
  <q-card
    v-ripple
    class="bg-primary text-white cursor-pointer q-hoverable q-pa-xs"
    @click="externalLink ? gotoExternal(link) : router.push(link)"
  >
    <span class="q-focus-helper"></span>
    <q-item>
      <q-item-section avatar>
        <q-icon left size="md" color="white" :name="icon"></q-icon>
      </q-item-section>
      <q-item-section>
        <q-item-label class="text-h6">{{ name }}</q-item-label>
        <q-item-label class="text-subtitle"> {{ desc }}</q-item-label>
      </q-item-section>
      <q-item-section avatar>
        <q-icon right size="md" color="white" name="mdi-chevron-right"></q-icon>
      </q-item-section>
    </q-item>
  </q-card>
</template>

<script lang="ts" setup>
import { useRouter } from "vue-router";

const props = defineProps<{
  icon: string | undefined;
  link: string;
  name: string;
  desc: string | undefined;
}>();

function gotoExternal(link) {
  window.location.href = link;
}

const router = useRouter();

const externalLink = props.link.startsWith("http");
</script>
