use rusqlite;// use rusqlite::ToSql;
use super::recipes::RecipeList;
use super::ingredients::IngredientList;

// pub struct RecipeQuery {}

// pub struct IngredientQuery {}

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
        category TEXT NOT NULL,
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
    Ok( Self{connection} )
  }
  pub fn store_ingredients(&mut self, ingredients: &IngredientList) -> Result<(), rusqlite::Error> {
    let tx: rusqlite::Transaction<'_> = self.connection.transaction()?;
    for ingredient in &ingredients.0 {
      tx.execute(
        "INSERT INTO ingredients (uuid, name, type) VALUES (?,?,?)",
        &[&ingredient.uuid, &ingredient.name, &ingredient.category.to_string()],
      )?;
    }
    tx.commit()?;
    Ok(())
  }
  pub fn store_recipes(&mut self, recipes: &RecipeList) -> Result<(), rusqlite::Error> {
    let tx: rusqlite::Transaction<'_> = self.connection.transaction()?;
    for recipe in &recipes.0 {
      tx.execute(
        "INSERT INTO recipes (uuid, name, category, instructions) VALUES (?,?,?,?)",
        &[&recipe.uuid, &recipe.name, &recipe.category.to_string(), &recipe.instructions],
      )?;
      for ingredient in &recipe.ingredients.0 {
        tx.execute(
          "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, quantity, unit) VALUES (?,?,?,?)",
          &[&recipe.uuid, &ingredient.uuid, "1", "ton"],
        )?;
      }
    }
    tx.commit()?;
    Ok(())
  }
  // pub fn load_recipes(&mut self, dbname: &str, recipelist: &mut RecipeList) -> Result<(), rusqlite::Error> {
  //   Ok(())
  // }
  // pub fn load_ingredients(&mut self, dbname: &str, ingredientlist: &mut IngredientList) -> Result<(), rusqlite::Error> {
  //   Ok(())
  // }
}

