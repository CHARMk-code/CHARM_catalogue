<template>
  <div>
    <div class="container">
      <div class = "row">
        <input type="text" v-model="tag">
      </div>
      <div class="row">
        <multiselect v-model="parent_tag" tag-placeholder="Add this as new tag"
        placeholder="Search Tag" label="name" track-by="id"
        :options="tags">
        </multiselect>
      </div>
      <div></div>
      <button type="button" v-on:click = "create()" class="btn btn-primary">Add</button>
    </div>
  </div>
</template>

<script>
// import Multiselect from 'vue-multiselect';
import axios from 'axios';

export default {
  name: 'Search',
  components: {
    // Multiselect,
  },
  data() {
    return {
      options: [],
      parent_tag: null,
      tags: [],
      tag: "",
    };
  },
  methods: {
    get_init() {
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
              this.tags = JSON.parse(localStorage.getItem("tag"));
          }
        })
    },
        create(){
        axios.get('http://localhost:5000/api/tag/create?tag=' + this.tag + "&parent_tag=" + this.parent_tag.id)
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
