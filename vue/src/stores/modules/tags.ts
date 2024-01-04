import { defineStore } from "pinia";
import { useTagCategoriesStore } from "./tag_category";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Tag {
  id: number;
  name: string;
  icon: string;
  category: number;
}

// TODO: Replace this with a mapping from database
export type Category = "Division" | "Business Area" | "Looking For" | "Offering" | "Language" | "Fair Area"

interface State {
  tags: Map<number, Tag>;
  load_wait: number;
}
export const useTagsStore = defineStore("tags", {
  state: (): State => ({
    tags: new Map(),
    load_wait: 0,
  }),
  actions: {
    setAllTags(tags: Tag[]) {
      this.tags = new Map(tags.map((t) => [t.id, t]));
    },
    removeTagById(id: number) {
      this.tags.delete(id);
    },
    removeAllTags() {
      this.tags = new Map();
    },
    getTags() {
      return new Promise<void>((resolve, reject) => {
        if (this.load_wait < Date.now()) {
          this.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/v2/tag/")
            .then((resp: any) => {
              const tags = resp.data;
              if (tags.length > 0) this.setAllTags(tags);
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
    updateTag(tag: Tag) {
        console.log("save tag", tag);
      return new Promise((resolve, reject) => {
        this.axios
          .put("/v2/tag/", tag)
          .then((resp: any) => {
            this.tags.set(tag.id, tag);
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    removeTag(tag: Tag) {
      return new Promise((resolve, reject) => {
        this.axios
          .delete("/v2/tag/" + tag.id)
          .then((resp: any) => {
            this.removeTagById(tag.id);
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
  },
  getters: {
    getTagsFromIds: (state) => (ids: Iterable<number>) => {
      const result: Tag[] = [];
      for (const id of ids) {
        const tag = state.tags.get(id);
        if (tag) {
          result.push(tag);
        }
      }
      return result;
    },
    getTagsByCategoryFromIds: (state) => (category: Category, ids: Set<number>) => {
      const category_id = [...useTagCategoriesStore().tag_categories.values()].find(({name}) => name === category)?.id
      return [...state.tags.values()].filter((tag) => ids.has(tag.id) && category_id === tag.category) 
    },
    getTagsInCategory: (state) => (category: Category) => {
      const category_id = [...useTagCategoriesStore().tag_categories.values()].find(({name}) => name === category)?.id
      return Array.from(state.tags.values()).filter((t) => t.category === category_id);
    }
  },
});
