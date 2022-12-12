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
  tags: Tag[],
  load_wait: number,
}

export const useTagsStore = defineStore('tags', {
  state: (): State => ({
    tags: [],
    load_wait: 0,
  }),
  actions: {
    getTags() {
      return new Promise<void>((resolve, reject) => {
        if (this.load_wait < Date.now()) {
          this.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/tag")
            .then((resp: any) => {
              this.tags = []
              const tags: Tag[] = resp.data;
              if (tags.length > 0) {
                this.tags = tags.filter(
                    (t) =>
                      !(
                        t.business_area ||
                        t.division ||
                        t.looking_for ||
                        t.offering ||
                        t.language
                      )
                );
              }
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
    modifyTag(tag: Tag) {
      return new Promise((resolve, reject) => {
        this.axios
          .put("/tag", tag)
          .then((resp: any) => {
            this.tags.filter((t) => t.id != tag.id)
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    deleteTag(tag: Tag) {
      return new Promise((resolve, reject) => {
        this.axios
          .delete("/tag/" + tag.id)
          .then((resp: any) => {
            this.tags = this.tags.filter((t) => t.id != tag.id);
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
  },
  getters: {
    getTagFromId: (state) => (id: number) => {
      if (state.tags.length > 0) {
        const result = state.tags.filter((t) => t.id == id);
        if (result.length > 0) {
          return result[0];
        }
      }
      return [];
    },
    getTagsFromIds: (state) => (ids: number[]) => {
      return state.tags
      .filter((t) => ids.indexOf(t.id) != -1);
    },
    getDivisionsFromIds: (state) => (ids: number[]) => {
      return state.tags
      .filter((t) => t.division)
      .filter((t) => ids.indexOf(t.id) != -1);
    },
    getBusinessAreasFromIds: (state) => (ids: number[]) => {
      return state.tags
      .filter((t) => t.business_area)
      .filter((t) => ids.indexOf(t.id) != -1);
    },
    getLookingForFromIds: (state) => (ids: number[]) => {
      return state.tags
      .filter((t) => t.looking_for)
      .filter((t) => ids.indexOf(t.id) != -1);
    },
    getOfferingsFromIds: (state) => (ids: number[]) => {
      return state.tags
      .filter((t) => t.offering)
      .filter((t) => ids.indexOf(t.id) != -1);
    },
    getLanguagesFromIds: (state) => (ids: number[]) => {
      return state.tags
      .filter((t) => t.language)
      .filter((t) => ids.indexOf(t.id) != -1);
    },
    tags: (state) => {
      return state.tags;
    },
    divisions: (state) => {
      return state.tags.filter((t) => t.division);
    },
    business_areas: (state) => {
      return state.tags.filter((t) => t.business_area);
    },
    looking_for: (state) => {
      return state.tags.filter((t) => t.looking_for);
    },
    offering: (state) => {
      return state.tags.filter((t) => t.offering);
    },
    languages: (state) => {
      return state.tags.filter((t) => t.language);
    },
  },
});
