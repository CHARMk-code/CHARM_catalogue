<template>
  <div>
    <div class="container">
      <div class="row">
        <multiselect v-model="company" tag-placeholder="Add this as new tag"
        placeholder="Search company" label="name" track-by="id"
        :options="options_comp">
        </multiselect>
      </div>
      <div class="row">
        <multiselect v-model="tag" tag-placeholder="Add this as new tag"
        placeholder="Search tag" label="name" track-by="id"
        :options="options_tag">
        </multiselect>
      </div>
      <button type="button" v-on:click = "add()" class="btn btn-primary">Add</button>
    </div>
  </div>
</template>

<script>
import Multiselect from 'vue-multiselect';
import axios from 'axios';

export default {
  name: 'Add_tag',
  omponents: {
    Multiselect,  
  },
  data() {
    return {
      options_tag: [],
      options_comp: [],
      tag: null,
      company: null,
    };
  },
  methods: {
    get_init() {
      axios.get('http://localhost:5000/api/tag/get')
        .then((res) => {
          res.data.forEach((tag)=>
          {
            this.options_tag.push({name: tag.name, id: tag.id});
          })        })
        .catch((error) => {
          // eslint-disable-next-line
          console.error(error);
        });
      axios.get('http://localhost:5000/api/company/get')
        .then((res) => {
          res.data.forEach((company)=>
          {
            this.options_comp.push({name: company.name, id: company.id});
          })        
        })
        .catch((error) => {
          // eslint-disable-next-line
          console.error(error);
        });
    },
    add(){
        axios.get('http://localhost:5000/api/tag/add?company=' + this.company.id + "&tag=" + this.tag.id)
        .catch((error) => {
          // eslint-disable-next-line
          console.error(error);
        });
    },
  },
  created() {
    this.get_init();
  },
};
</script>
