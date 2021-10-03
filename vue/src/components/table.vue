<template>
  <v-data-table 
    :headers="headers"
    :items="items"
    class="elevation-1"
    :search="search"
    item-key="id"
    multi-sort
    >
    <template v-slot:top>
      <v-toolbar flat >
        <v-toolbar-title>Companies</v-toolbar-title>
        <v-spacer/>
          <v-text-field
            v-model="search"
            label="search"
            class="mx-4"></v-text-field>
          <v-spacer/> 

            <v-dialog persistent 
            v-model="dialog"
            max-width="900px">
              <template v-slot:activator="{ on, attrs }">
                <v-btn
                  color="primary"
                  dark
                  class="mb-2"
                  v-bind="attrs"
                  v-on="on"
                  >
                  Add company 
                </v-btn>
              </template>
              <Company_edit_dialog @close_dialog="close_dialog()" :creating="creating" :company="editedCompany"/>
            </v-dialog>
      </v-toolbar>
    </template>

    <template v-slot:item.actions="{ item }">
      <v-icon
        class="mr-2"
        @click="editCompany(item)">
        mdi-pencil
      </v-icon>
    </template>

    <template v-slot:item.active="{ item }">
      <v-simple-checkbox disabled on-icon="mdi-eye" off-icon="mdi-eye-off" v-model="item.active"></v-simple-checkbox>
    </template>




  </v-data-table>
</template>


<script>
import Company_edit_dialog from "@/components/admin/Company_edit_dialog"

export default {
  name: 'Table',
  components: {
    Company_edit_dialog
  },
  data () {
    return {
      search: "",
      expanded: [],
      dialog: false,
      creating: true,
      editedCompany: {},
      headers: [
        {text: 'Name', value: 'name'},
        {text: 'Website', value: 'website'},
        {text: 'Tags', value: 'tags'},
        {text: 'Active', value: 'active'},
        {text: 'Actions', value: 'actions', sortable: false },
        
      ],
      items: [
        {
          id: 0,
          active: 'true',
          name: 'Volvo',
          desc: 'this is quite a long sentence since it is meant to be the whole description for the company. However it is not very descriptive, mostly from a lack of information about the actual company.',
          founded: '1903',
          contact: 'person@volvogroup.com',
          website: 'https://volvogroup.com',
          employees_sweden: '22450',
          employees_world: '45000',
          trivia: 'This is not a true fun fact/trivia',
          tags: ['a', 'd', 'e', 'f', 'h', 'i','k','m'],
        }
      ]
    }
  },
  methods: {
    editCompany (comp) {
      this.editedCompany = comp 
      this.dialog = true
      this.creating = false
    },
    close_dialog () {
      this.dialog = false
      this.creating = true
      this.editedCompany = {}
    }
  }

}
</script>
