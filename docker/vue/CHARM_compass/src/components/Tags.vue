<template>
  <v-container>
    <v-banner>
      Tags
    </v-banner>

    <v-list color="transparent">
      <v-list-item
        v-for="tag in companyTags"
        :key="tag.id"
        link
        >
        <v-list-item-content>
          <v-list-item-title>
            {{ tag.name }}
          </v-list-item-title>
        <v-list-item-subtitle>
          {{tag.approval * 100}}%
        </v-list-item-subtitle>
        </v-list-item-content>
      </v-list-item>

      <v-divider class="my-2"></v-divider>
      <v-list-item
        link
        color="grey lighten-4"
        >
        <AddTagDialog companyId="companyId"/>
      </v-list-item>

      <v-list-item
        link
        color="grey lighten-4"
        >

        <v-list-item-content>
          <v-list-item-title v-on:click="update_tags()">
            Refresh
          </v-list-item-title>
        </v-list-item-content>
      </v-list-item>
    </v-list>
  </v-container>
</template>

<script>
import AddTagDialog from '@/components/AddTagDialog'

export default {
  name: 'Tags',
  components: {
    AddTagDialog
  },
  props: {
    companyId: Number
  },
  data () {
    return {
      companyTags: []
    }
  },
  watch: {
    companyId (to, from) {
      const companyTags = this.$store.state.companyTags[this.companyId]

      if (typeof companyTags === 'undefined') {
        console.log('undefined companyTags')
        this.update_tags()
        return
      }

      this.companyTags = companyTags
    }
  },
  methods: {
    update_tags () {
      this.$store.dispatch('retrieveTagsForCompany', this.companyId)
    }
  },
  created () {
    this.unsubscribe = this.$store.subscribe((mutation, state) => {
      if (mutation.type === 'addTagstoCompany') {
        this.companyTags = mutation.payload.tags
        console.log(this.companyTags)
      }
      console.log(mutation.type)
      console.log(mutation.payload)
    })

    if (typeof companyTags === 'undefined') {
      console.log('undefined companyTags')
      this.update_tags()
    }
  },
  beforeDestroy () {
    this.unsubscribe()
  }
}
</script>
