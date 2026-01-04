import React, { useState } from "react";
import axios from "axios";

const API = import.meta.env.VITE_API;


export default function AddBook() {
  const [formData, setFormData] = useState({
    genre_id: "",
    title: "",
    author: "",
    info: "",
    description: "",
    historical_context: "",
    author_bio: "",
    image_url: "",
  });

  const handleChange = (e) => {
    setFormData(prev => ({
      ...prev,
      [e.target.name]: e.target.value,
    }));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      const response = await axios.post(`${API}/books/post`, formData);
      alert(`Book created with ID: ${response.data}`);
      setFormData({
        genre_id: "",
        title: "",
        author: "",
        info: "",
        description: "",
        historical_context: "",
        author_bio: "",
        image_url: "",
      });
    } catch (error) {
      console.error("Failed to add book:", error);
      alert("Failed to add book");
    }
  };

  return (
    <form onSubmit={handleSubmit} className="max-w-lg mx-auto p-6 bg-white rounded-md shadow-md">
      <h2 className="text-2xl font-semibold mb-6 text-gray-800">Add New Book</h2>
      
      <label className="block mb-2 font-medium text-gray-700">Genre ID</label>
      <input
        name="genre_id"
        value={formData.genre_id}
        onChange={handleChange}
        placeholder="Genre ID"
        required
        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 mb-4"
      />

      <label className="block mb-2 font-medium text-gray-700">Title</label>
      <input
        name="title"
        value={formData.title}
        onChange={handleChange}
        placeholder="Title"
        required
        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 mb-4"
      />

      <label className="block mb-2 font-medium text-gray-700">Author</label>
      <input
        name="author"
        value={formData.author}
        onChange={handleChange}
        placeholder="Author"
        required
        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 mb-4"
      />

      <label className="block mb-2 font-medium text-gray-700">Info</label>
      <input
        name="info"
        value={formData.info}
        onChange={handleChange}
        placeholder="Info"
        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 mb-4"
      />

      <label className="block mb-2 font-medium text-gray-700">Description</label>
      <input
        name="description"
        value={formData.description}
        onChange={handleChange}
        placeholder="Description"
        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 mb-4"
      />

      <label className="block mb-2 font-medium text-gray-700">Historical Context</label>
      <input
        name="historical_context"
        value={formData.historical_context}
        onChange={handleChange}
        placeholder="Historical Context"
        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 mb-4"
      />

      <label className="block mb-2 font-medium text-gray-700">Author Bio</label>
      <input
        name="author_bio"
        value={formData.author_bio}
        onChange={handleChange}
        placeholder="Author Bio"
        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 mb-4"
      />

      <label className="block mb-2 font-medium text-gray-700">Image URL</label>
      <input
        name="image_url"
        value={formData.image_url}
        onChange={handleChange}
        placeholder="Image URL"
        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 mb-6"
      />

      <button
        type="submit"
        className="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 rounded-md transition"
      >
        Add Book
      </button>
    </form>
  );
}
