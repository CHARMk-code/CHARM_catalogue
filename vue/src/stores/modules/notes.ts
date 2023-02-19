import type { Note } from "esbuild";
import { defineStore } from "pinia";

interface Notes {
  [propName: number]: string;
}
interface State {
  notes: Notes;
}

export const useNotesStore = defineStore("notes", {
  state: (): State => ({
    notes: {},
  }),
  actions: {
    setNotes(data: { id: number; note: string }) {
      this.notes[data.id] = data.note;
      localStorage.setItem("notes", JSON.stringify(this.notes));
    },
    loadFromStorage() {
      const stored_notes = localStorage.getItem("notes") || "[]";
      if (stored_notes == "{}") {
        this.notes = {};
      } else {
        this.notes = JSON.parse(stored_notes);
      }
    },
  },
  getters: {
    getById: (state) => (id: number) => {
      if (id in state.notes) {
        return state.notes[id];
      } else {
        return "";
      }
    },
  },
});
