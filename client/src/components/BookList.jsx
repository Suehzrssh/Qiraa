import BookCard from "./BookCard";

export default function BookList({ genres, onSelectBook }) {
  return (
    <section className="md:col-span-1 space-y-6">
      {genres.map((genre) => (
        <div key={genre.id}>
          <h2 className="text-lg mb-2 text-neutral-300">
            {genre.title}
          </h2>

          <div className="space-y-2">
            {genre.books.map((book) => (
              <BookCard
                key={book.id}
                book={book}
                onSelect={onSelectBook}
              />
            ))}
          </div>
        </div>
      ))}
    </section>
  );
}
