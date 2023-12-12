import { defineStore } from "pinia";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Tag {
  id: number;
  name: string;
  icon: string;
  category: string;
}
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
    getDivisionsFromIds: (state) => (ids: Iterable<number>) => {
      const result: Tag[] = [];
      for (const id of ids) {
        const tag = state.tags.get(id);
        if (tag && tag.category == 0) {
          result.push(tag);
        }
      }
      return result;
    },
    getBusinessAreasFromIds: (state) => (ids: Iterable<number>) => {
      const result: Tag[] = [];
      for (const id of ids) {
        const tag = state.tags.get(id);
        if (tag && tag.category == 1) {
          result.push(tag);
        }
      }
      return result;
    },
    getLookingForFromIds: (state) => (ids: Iterable<number>) => {
      const result: Tag[] = [];
      for (const id of ids) {
        const tag = state.tags.get(id);
        if (tag && tag.category == 2) {
          result.push(tag);
        }
      }
      return result;
    },
    getOfferingsFromIds: (state) => (ids: Iterable<number>) => {
      const result: Tag[] = [];
      for (const id of ids) {
        const tag = state.tags.get(id);
        if (tag && tag.category == 3) {
          result.push(tag);
        }
      }
      return result;
    },
    getLanguagesFromIds: (state) => (ids: Iterable<number>) => {
      const result: Tag[] = [];
      for (const id of ids) {
        const tag = state.tags.get(id);
        if (tag && tag.category == 4) {
          result.push(tag);
        }
      }
      return result;
    },
    getFairAreasFromIds: (state) => (ids: Iterable<number>) => {
      const result: Tag[] = [];
      for (const id of ids) {
        const tag = state.tags.get(id);
        if (tag && tag.category == 5) {
          result.push(tag);
        }
      }
      return result;
    },
    divisions: (state) => {
      return Array.from(state.tags.values()).filter((t) => t.category == 0);
    },
    business_areas: (state) => {
      return Array.from(state.tags.values()).filter((t) => t.category == 1);
    },
    looking_for: (state) => {
      return Array.from(state.tags.values()).filter((t) => t.category == 2);
    },
    offering: (state) => {
      return Array.from(state.tags.values()).filter((t) => t.category == 3);
    },
    languages: (state) => {
      return Array.from(state.tags.values()).filter((t) => t.category == 4);
    },
    fair_areas: (state) => {
      return Array.from(state.tags.values()).filter((t) => t.category == 5);
    },
  },
});
