using from '../db/schema';

service CatalogService {
  entity UserScopes {
    username : String;
    scope    : String;
  }
}