<template>
  <v-container>
    <v-sheet max-width="750px" elevation="1" class="px-12 py-6">
    <v-form>
      <v-checkbox v-model="company.active" label="Active (Required for company to be visible)"/>

      <v-text-field v-model="company.name" label="Company name"/>

      <v-textarea v-model="company.desc" label="Company description"/>

      <v-text-field v-model="company.founded" label="Founded"/>

      <v-text-field v-model="company.contact" label="contact"/>
      
      <v-text-field v-model="company.website" label="website"/>
    
      <v-text-field v-model="company.employees_sweden" label="employees sweden"/>

      <v-text-field v-model="company.employees_world" label="employees world"/>
        
      <v-text-field v-model="company.trivia" label="Trivia"/>
      
      <v-select v-model="company.tags" :items="all_tags" label="Tags" multiple chips hint="Active tags for company"/>

      <v-btn color="primary">
       Save 
      </v-btn>

    </v-form>
    </v-sheet>
  </v-container>

</template>

<script>
export default {
  props: ['name'],
  //get the actual company information from the store
  data() {
    return {
      company: {
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
      },
      all_tags: ['a', 'b', 'c', 'd', 'e', 'f', 'h', 'i','k','m'],
    }
  }
  
}
</script>
