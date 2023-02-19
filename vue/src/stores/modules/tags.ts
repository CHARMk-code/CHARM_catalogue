import { defineStore } from "pinia";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Tag {
  id: number;
  name: string;
  parent_tag: number;
  up_votes: number;
  down_votes: number;
  crowd_sourced: boolean;
  icon: string;
  division: boolean;
  business_area: boolean;
  looking_for: boolean;
  offering: boolean;
  language: boolean;
  fair_area: boolean;
}
interface State {
  tags: Map<number, Tag>;
  load_wait: number;
}
/*
tagsStore.modifyTag({"name": "test", "desc": "testarmycket", "link": "https://coollink.se", "icon": "mdi-search"})
*/
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
            .get("/tag")
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
          .put("/tag", tag)
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
          .delete("/tag/" + tag.id)
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
        if (tag && tag.division) {
          result.push(tag);
        }
      }
      return result;
    },
    getBusinessAreasFromIds: (state) => (ids: Iterable<number>) => {
      const result: Tag[] = [];
      for (const id of ids) {
        const tag = state.tags.get(id);
        if (tag && tag.business_area) {
          result.push(tag);
        }
      }
      return result;
    },
    getLookingForFromIds: (state) => (ids: Iterable<number>) => {
      const result: Tag[] = [];
      for (const id of ids) {
        const tag = state.tags.get(id);
        if (tag && tag.looking_for) {
          result.push(tag);
        }
      }
      return result;
    },
    getOfferingsFromIds: (state) => (ids: Iterable<number>) => {
      const result: Tag[] = [];
      for (const id of ids) {
        const tag = state.tags.get(id);
        if (tag && tag.offering) {
          result.push(tag);
        }
      }
      return result;
    },
    getLanguagesFromIds: (state) => (ids: Iterable<number>) => {
      const result: Tag[] = [];
      for (const id of ids) {
        const tag = state.tags.get(id);
        if (tag && tag.language) {
          result.push(tag);
        }
      }
      return result;
    },
    getFairAreasFromIds: (state) => (ids: Iterable<number>) => {
      const result: Tag[] = [];
      for (const id of ids) {
        const tag = state.tags.get(id);
        if (tag && tag.fair_area) {
          result.push(tag);
        }
      }
      return result;
    },
    divisions: (state) => {
      return Array.from(state.tags.values()).filter((t) => t.division);
    },
    business_areas: (state) => {
      return Array.from(state.tags.values()).filter((t) => t.business_area);
    },
    looking_for: (state) => {
      return Array.from(state.tags.values()).filter((t) => t.looking_for);
    },
    offering: (state) => {
      return Array.from(state.tags.values()).filter((t) => t.offering);
    },
    languages: (state) => {
      return Array.from(state.tags.values()).filter((t) => t.language);
    },
    fair_areas: (state) => {
      return Array.from(state.tags.values()).filter((t) => t.fair_area);
    },
  },
});
