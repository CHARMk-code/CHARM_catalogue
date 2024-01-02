import { defineStore } from "pinia";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Tag_category {
  id: number;
  name: string;
}
interface State {
  tags_categories: Map<number, Tag_category>;
  load_wait: number;
}
export const useTagCategoriesStore = defineStore("tagCategories", {
  state: (): State => ({
    tags_categories: new Map(),
    load_wait: 0,
  }),
  actions: {
    setAllTagCategories(tags_categories: Tag_category[]) {
      this.tags_categories = new Map(tags_categories.map((t) => [t.id, t]));
    },
    removeTagCategoryById(id: number) {
      this.tags_categories.delete(id);
    },
    removeAllTagCategories() {
      this.tags_categories = new Map();
    },
    fetchTagCategories() {
      return new Promise<void>((resolve, reject) => {
        if (this.load_wait < Date.now()) {
          this.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/v2/tag_category/")
            .then((resp: any) => {
              const tags_categories = resp.data;
              if (tags_categories.length > 0) this.setAllTagCategories(tags_categories);
              resolve(resp);
            })
            .catch((err: any) => {
              reject(err);
            });
        } else {
          resolve();
        }
      });
    },
    updateTag(tag_category: Tag_category) {
      return new Promise((resolve, reject) => {
        this.axios
          .put("/v2/tag_category/", tag_category)
          .then((resp: any) => {
            this.tags.set(tag_category.id, tag_category);
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    removeTag(tag_category: Tag_category) {
      return new Promise((resolve, reject) => {
        this.axios
          .delete("/v2/tag_category/" + tag_category.id)
          .then((resp: any) => {
            this.removeTagById(tag_category.id);
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
  },
});
