import { defineStore } from "pinia";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Tag_category {
  id: number;
  name: string;
}
interface State {
  tag_categories: Map<number, Tag_category>;
  load_wait: number;
}
export const useTagCategoriesStore = defineStore("tagCategories", {
  state: (): State => ({
    tag_categories: new Map(),
    load_wait: 0,
  }),
  actions: {
    setAllTagCategories(tag_categories: Tag_category[]) {
      this.tag_categories = new Map(tag_categories.map((t) => [t.id, t]));
    },
    removeTagCategoryById(id: number) {
      this.tag_categories.delete(id);
    },
    removeAllTagCategories() {
      this.tag_categories = new Map();
    },
    fetchTagCategories() {
      return new Promise<void>((resolve, reject) => {
        if (this.load_wait < Date.now()) {
          this.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/v2/tag_category/")
            .then((resp: any) => {
              const tag_categories = resp.data;
              if (tag_categories.length > 0) this.setAllTagCategories(tag_categories);
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
    updateTagCategory(tag_category: Tag_category) {
      return new Promise((resolve, reject) => {
        this.axios
          .put("/v2/tag_category/", tag_category)
          .then((resp: any) => {
            this.tag_categories.set(tag_category.id, tag_category);
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    removeTagCategory(tag_category: Tag_category) {
      return new Promise((resolve, reject) => {
        this.axios
          .delete("/v2/tag_category/" + tag_category.id)
          .then((resp: any) => {
            this.removeTagCategoryById(tag_category.id);
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
  },
});
