import axios from "axios";

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL ? import.meta.env.VITE_API_URL + '/api' : "http://localhost:3000/api",
});

console.log("API base URL:", import.meta.env.VITE_API_URL);
// genres
export const fetchGenres = async () => {
  const res = await api.get("/genres");
  return res.data;
};

// books by genre (if backend supports it)
export const fetchBooks = async () => {
  const res = await api.get("/books");
  return res.data;
};

// one book
export const fetchBook = async (bookId) => {
  const res = await api.get(`/books/${bookId}`);
  return res.data;
};

// chapters of a book
export const fetchChapters = async (bookId) => {
  const res = await api.get(`/books/${bookId}/chapters`);
  return res.data;
};

// one chapter
export const fetchChapter = async (bookId, chapterId) => {
  const res = await api.get(
    `/books/${bookId}/chapters/${chapterId}`
  );
  return res.data;
};
