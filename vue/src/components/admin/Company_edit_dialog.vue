<template>
  <v-card>
    <v-form>
    <v-card-title v-if="!creating">Edit company</v-card-title>
    <v-card-title v-if="creating">Add new company</v-card-title>
    <v-card-text>
      <v-checkbox v-model="company.active" large on-icon="mdi-eye" off-icon="mdi-eye-off" label="Active (Required for company to be visible)"/>

      <v-text-field v-model="company.name" label="Company name"/>

      <v-textarea v-model="company.desc" label="Company description"/>

      <v-text-field v-model="company.founded" label="Founded"/>

      <v-text-field v-model="company.contact" label="contact"/>
      
      <v-text-field v-model="company.website" label="website"/>
    
      <v-text-field v-model="company.employees_sweden" label="employees sweden"/>

      <v-text-field v-model="company.employees_world" label="employees world"/>
        
      <v-text-field v-model="company.trivia" label="Trivia"/>
      
      <v-select v-model="company.tags" :items="all_tags" label="Tags" multiple chips hint="Active tags for company"/>
    </v-card-text>
    <v-card-actions>
      <v-btn color="primary" @click="save()">
       Save 
      </v-btn>

      <v-btn @click="close()">
       Cancel 
      </v-btn>
    </v-card-actions>
    </v-form>
  </v-card>

</template>

<script>
export default {
  name: "Company_edit_dialog",
  props: ['company', 'creating'],
  methods: {
    save () {
      console.log(this.company)
      //update the store

      //send update data to database
      this.close()
    },
    close () {
      this.$emit("close_dialog")
    }
  },
  data() {
    return {
      //make sure to get from store instead
      all_tags: ['a', 'b', 'c', 'd', 'e', 'f', 'h', 'i','k','m'], 
    }
  }
}
</script>
