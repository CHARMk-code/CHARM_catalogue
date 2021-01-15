<template>
  <v-dialog v-model="dialog" max-width="900">
    <template v-slot:activator="{on, attrs}">
      <v-list-item-content
        v-bind="attrs"
        v-on="on"
        >
        <v-list-item-title>
          Add tag
        </v-list-item-title>
      </v-list-item-content>
    </template>

    <v-card >
      <v-card-title class="headline grey lighten-2">
        Add tag
      </v-card-title>
      <v-card-text class="pa-6">
        <SearchBar goto="false" search_resource="tags"/>
        <v-list>

          <!-- Show the ability to create new tags if none is found -->
          <v-list-item
            v-if="matching_tags.length === 0"
            >
            <v-input placeholder="Create a new tag..."></v-input>
          </v-list-item>
          <v-list-item
            v-for="tag in matching_tags"
            :key="tag.id">
            <v-list-item-content>
              <v-list-item-title>
            {{ tag.name }}
              </v-list-item-title>
            </v-list-item-content>
          </v-list-item>

        </v-list>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn
          color="primary"
          text
          @click="dialog = false"
          > Add tag </v-btn>

        <v-btn
          color="primary"
          text
          @click="dialog = false"
          > Cancel </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script>
import SearchBar from '@/components/Search-bar'
import {mapGetters as mapSearchGetters, getterTypes} from 'vuex-search'

export default {
  name: 'AddTagDialog',
  components: {
    SearchBar
  },
  data () {
    return {
      dialog: false
    }
  },
  computed: {
    ...mapSearchGetters('tags', {
      search_result_ids: getterTypes.result,
      isLoading: getterTypes.isSearching
    }),
    matching_tags () {
      const actualResult = []

      for (var i = 0; i < this.search_result_ids.length; i++) {
        actualResult.push(this.$store.getters.getTagById(this.search_result_ids[i]))
      }
      return actualResult
    }
  },
  methods: {
  }
}
</script>
