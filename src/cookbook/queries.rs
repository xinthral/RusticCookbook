use rusqlite;// use rusqlite::ToSql;

pub struct RecipeQuery {}

pub struct IngredientQuery {}

pub struct SQLiteConnection {
  connection: rusqlite::Connection,
}
impl SQLiteConnection {
  pub fn new(path: &str) -> Result<Self, rusqlite::Error> {
    let connection = rusqlite::Connection::open(path)?;
    connection.execute(
      "CREATE TABLE IF NOT EXISTS ingredients (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        uuid TEXT NOT NULL,
        name TEXT NOT NULL,
        type TEXT NOT NULL
      )",
      [],
    )?;
    connection.execute(
      "CREATE TABLE IF NOT EXISTS recipes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        uuid TEXT NOT NULL,
        name TEXT NOT NULL,
        description TEXT NOT NULL,
        instructions TEXT NOT NULL
      )",
      [],
    )?;
    connection.execute(
      "CREATE TABLE IF NOT EXISTS recipe_ingredients (
        recipe_id INTEGER NOT NULL,
        ingredient_id INTEGER NOT NULL,
        quantity REAL NOT NULL,
        unit TEXT NOT NULL,
        FOREIGN KEY(recipe_id) REFERENCES recipes(id),
        FOREIGN KEY(ingredient_id) REFERENCES ingredients(id)
      )",
      [],
    )?;
    Ok(Self {connection})
  }
}