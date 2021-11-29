export default {
  namespaced: true,
  state: () => ({
    notes: {},
  }),
  mutations: {
    setNotes(state, data) {
      console.log(data);
      state.notes[data.id] = data.note;
      localStorage.setItem("notes", JSON.stringify(state.notes));
    },
    loadForStorage(state) {
      if (
        localStorage.getItem("notes") == "{}" ||
        localStorage.getItem("notes") == null
      ) {
        state.notes = {};
      } else {
        state.notes = JSON.parse(localStorage.getItem("notes"));
      }
    },
  },
  getters: {
    notes: (state) => {
      return state.notes;
    },
    getById: (state, id) => {
      if (id in state.notes) {
        return state.notes[id];
      } else {
        return "";
      }
    },
  },
};
