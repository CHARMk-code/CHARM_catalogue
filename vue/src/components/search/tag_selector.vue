<template>
  <v-autocomplete
    multiple
    chips
    return-object
    item-text="name"
    item-value="id"
    v-model="selected_tags"
    @change="onChange"
    :label="label"
    :items="tags"
  >
    <template v-slot:item="{ item /*, attrs */}">
      <!--<v-simple-checkbox :value="attrs.inputValue" />-->
      <template v-if="item.icon != ''">
        <v-img
          class="ml-2 mr-4"
          contain
          max-height="36px"
          max-width="36px"
          :src="base_URL + item.icon"
        />
      </template>
      {{ item.name }}
    </template>

    <template v-slot:selection="{ item }">
      <template v-if="item.icon != ''">
        <v-avatar>
          <v-img
            max-height="36px"
            max-width="36px"
            :src="base_URL + item.icon"
          />
        </v-avatar>
      </template>
      <template v-else>
        <v-chip small>
          {{ item.name }}
        </v-chip>
      </template>
    </template>
  </v-autocomplete>
</template>

<script lang="ts" setup>
import axios from '@/plugins/axios';
import { useTagsStore, type Tag } from '@/stores/modules/tags';
import { computed, type Ref } from 'vue';

const tagsStore = useTagsStore();

const base_URL = axios.defaults.baseURL + "/manage/image/";

const props = defineProps<{
  tags: number[]
  selected_tags: number[]
  label: string
}>();

const emit = defineEmits(['change'])

function onChange(v: number[]) {
  emit("change", v)
}

const selected_tags: Ref<Tag[]> = computed({
  get() {
    return tagsStore.getTagsFromIds(props.selected_tags);
  },
  set(newTags) {
    return newTags.map(tag => tag.id);
  }
});
</script>
