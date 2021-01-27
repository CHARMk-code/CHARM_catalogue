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
        <v-data-table
          :headers="tag_headers"
          :items="matching_tags"
          :items-per-page="10"
          >
          <template v-slot:item.actions="{ item }">
            <v-btn
              v-bind:disabled="hasVoted[item.id] === 1"
              color="green"
              @click="upvote(item)"
              >
              <v-icon>mdi-thumb-up-outline</v-icon>
            </v-btn>

            <v-btn
              v-bind:disabled="hasVoted[item.id] === -1"
              color="red"
              @click="downvote(item)"
              >
              <v-icon>mdi-thumb-down-outline</v-icon>
            </v-btn>
          </template>
        </v-data-table>

      </v-card-text>
      <v-spacer></v-spacer>
      <v-card-actions>
        <v-btn
          color="primary"
          text
          @click="dialog = false"
          > Close </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script>
import SearchBar from '@/components/Search-bar'
import {mapGetters as mapSearchGetters, getterTypes} from 'vuex-search'
import {HTTP} from '@/plugins/http-common.js'

export default {
  name: 'AddTagDialog',
  components: {
    SearchBar
  },
  props: [
    'companyId'
  ],
  data () {
    return {
      dialog: false,
      hasVoted: {},
      tag_headers: [
        {
          text: 'Tag',
          align: 'start',
          sortable: false,
          value: 'id'
        },
        {
          text: 'Upvotes',
          value: 'upvotes'
        },
        {
          text: 'Downvotes',
          value: 'downvotes'
        },
        {
          text: 'Rating',
          value: 'rating'
        },
        {
          text: 'Voting',
          value: 'actions'
        }
      ]
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
    upvote (tag) {
      HTTP.post('/company/vote', { company: this.company, tag: tag.id, vote: 1 })
        .then(() => {
          this.hasVoted[tag.id] = 1
        })
      this.hasVoted[tag.id] = 1
    },
    downvote (tag) {
      HTTP.post('/company/vote', { company: this.company, tag: tag.id, vote: -1 })
        .then(() => {
          this.hasVoted[tag.id] = -1
        })
      this.hasVoted[tag.id] = -1
    }
  }
}
</script>
