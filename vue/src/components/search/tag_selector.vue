<template>
  <q-select
    v-model="selected"
    class="q-ma-sm"
    filled
    multiple
    :options="tags"
    :label="label"
    clearable
  >
    <template #option="{ opt, itemProps }">
      <q-item v-bind="itemProps">
        <q-item-section v-if="opt.icon && opt.icon.length > 0" avatar>
          <Tag_group :tags="[opt]"></Tag_group>
        </q-item-section>
        <q-item-section> {{ opt.name }}</q-item-section>
      </q-item>
    </template>

    <template #selected-item="{ index, opt }">
      <Tag_group v-if="index < props.maxShown" :tags="[opt]"></Tag_group>
      <span
        v-if="index === props.maxShown"
        class="text-grey text-caption align-self-center"
        >(+ {{ selected.length - maxShown }} others)</span
      >
    </template>
  </q-select>
</template>

<script lang="ts" setup>
import type { Tag } from "@/stores/modules/tags";
import { ref, watch, type Ref } from "vue";
import Tag_group from "@/components/Tag_group.vue";

const props = defineProps<{
  tags: Tag[];
  selectedTags: Tag[];
  label: string;
  maxShown: number;
}>();

const emit = defineEmits<{
  (e: "update:selectedTags", v: Tag[]): void;
  (e: "change"): void;
}>();

const selected: Ref<Tag[]> = ref([]);

selected.value = props.selectedTags;

watch(selected, (tags: Tag[]) => {
  emit("update:selectedTags", tags);
});
</script>
