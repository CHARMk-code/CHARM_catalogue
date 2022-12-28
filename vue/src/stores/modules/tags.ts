import { defineStore } from "pinia";
const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Tag {
  id: number,
  name: string,
  parent_tag: number,
  up_votes: number,
  down_votes: number,
  crowd_sourced: boolean
  icon: string,
  division: boolean,
  business_area: boolean,
  looking_for: boolean,
  offering: boolean,
  language: boolean,
}
interface State {
  tags: Map<number, Tag>,
  load_wait: number,
  getter_uses: number
}
/*
tagsStore.modifyTag({"name": "test", "desc": "testarmycket", "link": "https://coollink.se", "icon": "mdi-search"})
*/
export const useTagsStore = defineStore('tags', {
  state: (): State => ({
    tags: new Map(),
    load_wait: 0,
    getter_uses: 0,
  }),
  actions: {
    setAllTags(tags: Tag[]) {
      this.tags = new Map(tags.map(t => [t.id, t]))
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
              if (tags.length > 0)
                this.setAllTags(tags)
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
            this.tags.set(tag.id, tag)
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
            this.removeTagById(tag.id)
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
  },
  getters: {
    getTagsFromIds: (state) => (ids: number[]) => {
      return ids
        .map((id) => state.tags.get(id))
    },
    getDivisionsFromIds: (state) => (ids: number[]) => {
      const a = ids.map((id) => state.tags.get(id))
        .filter((t) => t?.division)
      return a
    },
    getBusinessAreasFromIds: (state) => (ids: number[]) => {
      const a = ids.map((id) => state.tags.get(id))
        .filter((t) => t?.business_area)
      return a
    },
    getLookingForFromIds: (state) => (ids: number[]) => {
      const a = ids
        .map((id) => state.tags.get(id))
        .filter((t) => t?.looking_for)
      return a
    },
    getOfferingsFromIds: (state) => (ids: number[]) => {
      return ids
        .map((id) => state.tags.get(id))
        .filter((t) => t?.offering)
    },
    getLanguagesFromIds: (state) => (ids: number[]) => {
      return ids
        .map((id) => state.tags.get(id))
        .filter((t) => t?.language)
    },
    divisions: (state) => {
      return Array.from(state.tags.values())
        .filter((t) => t.division);
    },
    business_areas: (state) => {
      return Array.from(state.tags.values())
        .filter((t) => t.business_area);
    },
    looking_for: (state) => {
      return Array.from(state.tags.values())
        .filter((t) => t.looking_for);
    },
    offering: (state) => {
      return Array.from(state.tags.values())
        .filter((t) => t.offering);
    },
    languages: (state) => {
      return Array.from(state.tags.values())
        .filter((t) => t.language);
    },
  },
});
