<template>
  <div>
    <div class="container">
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
      options: [
      ],
      parent_tag: null,
      tags: [],
    };
  },
  methods: {
    get_init() {
      axios.get('http://localhost:5000/api/tag/get')
        .then((res) => {
          this.tags = res.data;
        })
        .catch((error) => {
          // eslint-disable-next-line
          console.error(error);
        });
    },
        create(){
        const tag = "name";
        const parent_tag = "1";
        axios.get('http://localhost:5000/api/tag/create?tag=' + tag + "&parent_tag=" + parent_tag)
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
