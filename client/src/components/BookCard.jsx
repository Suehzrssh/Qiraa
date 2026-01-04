export default function BookCard({ book, onSelect }) {
  return (
    <button
      onClick={() => onSelect(book)}
      className="w-full text-left p-3 rounded-xl bg-neutral-900 hover:bg-neutral-800 transition"
    >
      <p className="font-medium text-neutral-100">
        {book.title}
      </p>

      <p className="text-xs text-neutral-400">
        {book.author}
      </p>

      {/* Book info */}
      <p className="text-xs text-neutral-500 mt-1">
        {book.info}
      </p>
    </button>
  );
}
