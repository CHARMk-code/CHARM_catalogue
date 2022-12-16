<template>
  <v-container>
    <v-card>
      <v-card-title> Visible Company Cards</v-card-title>
      <v-card-text>
        <v-combobox
          v-model="selected"
          :filter="filter"
          :items="company_cards"
          item-title="text"
          item-value="id"
          label="Select the elements that should be visible for users"
          multiple
        >
          <!-- <template #selection="{ item }">
            <v-chip>{{ item.value.text }} </v-chip>
          </template>
          <template #item="{ props, item }">
            <v-list-item v-bind="props"
              ><v-chip
                >{{ props.value.active }}{{ item.value.text }}</v-chip
              ></v-list-item
            >
          </template> -->
          <template v-slot:no-data>
            <v-list-item>
              <span class="subheading">All cards are activated</span>
            </v-list-item>
          </template>
        </v-combobox>
      </v-card-text>

      <v-card-actions>
        <v-spacer />
        <v-btn large text @click="reset_popup = !reset_popup"> Reset </v-btn>
        <v-dialog persistent v-model="reset_popup" max-width="500px">
          <v-card absolute>
            <v-card-title> Reset to Default </v-card-title>
            <v-card-text>
              This will reset which company cards are shown to user to the
              default settings
              <v-card color="red lighten-4" v-if="error.length > 0">
                <v-card-title> Error: while resetting </v-card-title>
                <v-card-text> {{ error }} </v-card-text>
              </v-card>
            </v-card-text>
            <v-card-actions>
              <v-btn color="primary" :loading="resetting" @click="reset()">
                Reset
              </v-btn>

              <v-btn @click="reset_popup = false"> Cancel </v-btn>
            </v-card-actions>
          </v-card>
        </v-dialog>
        <v-btn class="mx-2" large color="primary" @click="save()"> Save </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script lang="ts" setup>
import {
  useSite_settingsStore,
  type Card,
} from "@/stores/modules/site_settings";
import { computed } from "vue";

let reset_popup = false;
let search = "";
let resetting = false;
let error = "";

let site_settingsStore = useSite_settingsStore();

const company_cards = site_settingsStore.settings.company_view.cards;

const selected = computed<Card[]>({
  get: () => company_cards.filter((c) => c.active),
  set: (new_cards) => site_settingsStore.setCompanyCards(new_cards),
});

const filter = (item, queryText, itemText) => {
  if (item.header) return false;

  const hasValue = (val: string) => (val != null ? val : "");

  const text = hasValue(itemText);
  const query = hasValue(queryText);

  return (
    text.toString().toLowerCase().indexOf(query.toString().toLowerCase()) > -1
  );
};

const save = () => {
  useSite_settingsStore().saveCompanyCards();
};
const reset = () => {
  resetting = true;
  useSite_settingsStore()
    .resetCompanyCards()
    .then(() => {
      resetting = false;
      reset_popup = false;
    })
    .catch((err) => {
      resetting = false;
      error = err;
    });
};
</script>
