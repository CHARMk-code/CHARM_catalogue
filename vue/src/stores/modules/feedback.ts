import { defineStore } from "pinia";

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Feedback {
  id?: number;
  title: string;
  text: string;
  meta: string;
  received: Date;
  important: boolean;
  archived: boolean;
}

interface State {
  feedback: Feedback[];
  load_wait: number;
}

export const useFeedbackStore = defineStore("feedback", {
  state: (): State => ({
    feedback: [],
    load_wait: 0,
  }),
  actions: {
    removeFeedbackById(id: number) {
      this.feedback.filter((f) => f.id !== id);
    },
    fetchFeedback() {
      return new Promise<void>((resolve, reject) => {
        if (this.load_wait < Date.now()) {
          this.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/v2/feedback/")
            .then((resp: any) => {
              this.feedback = resp.data;
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
    deleteFeedback(feedback: Feedback) {
      return new Promise((resolve, reject) => {
        if (feedback.id == undefined) {
          reject("No id");
        } else {
          this.axios
            .delete("/v2/feedback/" + feedback.id)
            .then((resp: any) => {
              this.removeFeedbackById(feedback.id);
              resolve(resp);
            })
            .catch((err: any) => {
              reject(err);
            });
        }
      });
    },
    sendUserFeedback(feedback: Feedback) {
      return new Promise((resolve, reject) => {
        this.axios
          .put("/v2/feedback", feedback)
          .then((resp: any) => {
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    sendAdminFeedback(feedback) {
      return new Promise((resolve, reject) => {
        this.axios
          .post("/v2/feedback", feedback)
          .then((resp: any) => {
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    sendAllFeedback() {
      const promises = this.feedback.map((f) => this.sendAdminFeedback(f));
      return Promise.all(promises);
    },
  },
});
