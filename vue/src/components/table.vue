<template>
  <v-data-table 
    :headers="headers"
    :items="data"
    class="elevation-1"
    :search="search"
    item-key="id"
    multi-sort
    >
    <template v-slot:top>
      <v-toolbar flat >
        <v-toolbar-title>{{ name }}</v-toolbar-title>
        <v-spacer/>
          <v-text-field
            v-model="search"
            label="search"
            class="mx-4"></v-text-field>
          <v-spacer/> 

            <v-dialog v-if="editable" persistent 
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
                  Create new  
                </v-btn>
              </template>

                <tableEditDialog 
                  @close_dialog="closeDialog()" 
                  @save_row="saveRow"
                  :name="name" 
                  :new="creating" 
                  :row="editedRow" 
                  :row_meta="row_meta" />
            </v-dialog>
      </v-toolbar>
    </template>

    <template v-if="editable" v-slot:item.actions="{ item }">
      <v-icon
        class="mr-2"
        @click="editRow(item)">
        mdi-pencil
      </v-icon>
    </template>

    <template v-slot:item.active="{ item }">
      <v-simple-checkbox disabled on-icon="mdi-eye" off-icon="mdi-eye-off" v-model="item.active"></v-simple-checkbox>
    </template>




  </v-data-table>
</template>


<script>
import tableEditDialog from "@/components/admin/table_edit_dialog"

export default {
  name: 'Table',
  components: {
    tableEditDialog 
  },
  props: ['editable', 'name', 'headers', 'data', 'row_meta'],
  data () {
    return {
      search: "",
      expanded: [],
      dialog: false,
      creating: true,
      editedRow: {},
    }
  },
  methods: {
    editRow(row) {
      console.log(row)
      this.editedRow = row;
      console.log(this.editedRow)
      this.creating = false;
      this.dialog = true;
    },
    closeDialog () {
      this.dialog = false
      this.creating = true
      this.$emit('row_changed', this.editedRow)
      this.editedRow= {}
    },
    saveRow(event, row) {
      this.$emit("save_edit", row)
    }
  }

}
</script>
