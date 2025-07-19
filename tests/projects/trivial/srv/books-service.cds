namespace trivial.books;

type Author {
  name: String;
  surname: String;
};

service BooksService {
  entity Books {
    id: UUID;
    author: Author;
  }
}
