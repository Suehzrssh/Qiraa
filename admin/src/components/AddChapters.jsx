import { useState, useEffect } from 'react';
import axios from 'axios';

const API = import.meta.env.VITE_API;

const AddChapters = () => {
  const [books, setBooks] = useState([]);
  const [form, setForm] = useState({
    book_id: '',
    order_no: '',
    title: '',
    content: '',
  });
  const [message, setMessage] = useState('');

  useEffect(() => {
    axios.get(`${API}/books`)
      .then(response => setBooks(response.data))
      .catch(() => setBooks([]));
  }, []);

  const handleChange = (e) => {
    setForm(prev => ({
      ...prev,
      [e.target.name]: e.target.value,
    }));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setMessage('');

    try {
      const response = await axios.post(`${API}/chapters`, {
        ...form,
        order_no: Number(form.order_no),
      });

      setMessage(`Chapter created with ID: ${response.data}`);
      alert(`Chapter created with ID: ${response.data}`);
      setForm({ book_id: '', order_no: '', title: '', content: '' });
    } catch (error) {
      setMessage('Failed to create chapter: ' + (error.response?.data || error.message));
    }
  };

  return (
    <div className="max-w-xl mx-auto p-6 bg-white shadow-md rounded-md mt-8">
      <h2 className="text-2xl font-semibold mb-4">Add New Chapter</h2>

      <form onSubmit={handleSubmit} className="space-y-4">
        <select
          name="book_id"
          value={form.book_id}
          onChange={handleChange}
          required
          className="w-full border border-gray-300 rounded px-3 py-2"
        >
          <option value="" disabled>Select a Book</option>
          {books.map(book => (
            <option key={book.id} value={book.id}>{book.title}</option>
          ))}
        </select>

        <input
          type="number"
          name="order_no"
          placeholder="Order Number"
          value={form.order_no}
          onChange={handleChange}
          required
          min={1}
          className="w-full border border-gray-300 rounded px-3 py-2"
        />

        <input
          type="text"
          name="title"
          placeholder="Chapter Title"
          value={form.title}
          onChange={handleChange}
          required
          className="w-full border border-gray-300 rounded px-3 py-2"
        />

        <textarea
          name="content"
          value={form.content}
          onChange={handleChange}
          rows={20}
          className="w-full border border-gray-300 rounded px-3 py-2"
        />

        <button
          type="submit"
          className="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700"
          disabled={!form.book_id}
        >
          Add Chapter
        </button>
      </form>

      {message && <p className="mt-4 text-center text-sm text-gray-700">{message}</p>}
    </div>
  );
}

export default AddChapters
