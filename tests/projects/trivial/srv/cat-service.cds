service CatalogService {
  entity UserScopes {
    username : String;
    scope    : String;
  }

  function getUserScopesCount() returns Integer;
}
