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
      // Check timestamp and update if newer
      axios.get('http://localhost:5000/api/company/get?timestamp=True')
        .then((timestamp) => {
          if ((timestamp.data > localStorage.last_update_company) || (localStorage.getItem("last_update_company") == null))
          {
            axios.get('http://localhost:5000/api/company/get')
            .then((res) => {
              this.company = res.data;
              console.log(res.data);
              localStorage.setItem("company",JSON.stringify(this.company));
              localStorage.last_update_company = timestamp.data;
            });
          }else{
            this.options_comp = JSON.parse(localStorage.getItem("company"));
          }
        })
        .catch((error) => {
          // eslint-disable-next-line
          console.error(error);
        });


      // Check timestamp and update if newer
      axios.get('http://localhost:5000/api/tag/get?timestamp=True')
        .then((timestamp) => {
          if ((timestamp.data > localStorage.last_update_tag) || (localStorage.getItem("last_update_tag") == null))
          {
            axios.get('http://localhost:5000/api/tag/get')
            .then((res) => {
              res.data.forEach((tag)=>
              {
                this.options.push({name: tag.name, id: tag.id});
              })
              this.tag = res.data;
              localStorage.setItem("tag",JSON.stringify(res.data));
              localStorage.setItem("tag_options", JSON.stringify(this.options));
              localStorage.last_update_tag = timestamp.data;
            })       
          }else{
              // this.tag = localStorage.getItem("tag");
              this.options_tag = JSON.parse(localStorage.getItem("tag_options"));
          }
        })
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
