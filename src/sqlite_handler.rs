use rusqlite::{Connection, params};
use std::fs::File;
use std::io::{self, BufReader};
use csv::ReaderBuilder;

pub struct SQLiteHandler {
  pub conn: Connection,
}

impl SQLiteHandler {
  /// Initialize database and create tables
  pub fn new(db_path: &str) -> Self {
    let conn = Connection::open(db_path).expect("Failed to open database");

    conn.execute_batch(
      "CREATE TABLE IF NOT EXISTS ingredients (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        category TEXT NOT NULL
      );

      CREATE TABLE IF NOT EXISTS recipes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        instructions TEXT NOT NULL
      );

      CREATE TABLE IF NOT EXISTS recipe_ingredients (
        recipe_id INTEGER,
        ingredient_id INTEGER,
        quantity TEXT NOT NULL,
        PRIMARY KEY (recipe_id, ingredient_id),
        FOREIGN KEY (recipe_id) REFERENCES recipes(id) ON DELETE CASCADE,
        FOREIGN KEY (ingredient_id) REFERENCES ingredients(id) ON DELETE CASCADE
      );"
    ).expect("Failed to create tables");

    SQLiteHandler { conn }
  }

  /// Load ingredients from a CSV file into the database
  pub fn load_ingredients(&self, csv_path: &str) -> io::Result<()> {
    let file = File::open(csv_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    for result in csv_reader.records() {
      let record = result?;
      let name = &record[0];
      let category = &record[1];

      self.conn.execute(
        "INSERT OR IGNORE INTO ingredients (name, category) VALUES (?1, ?2)",
        params![name, category]
      ).expect("Failed to insert ingredient");
    }

    Ok(())
  }

  /// Load recipes from a CSV file into the database
  pub fn load_recipes(&self, csv_path: &str) -> io::Result<()> {
    let file = File::open(csv_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    for result in csv_reader.records() {
      let record = result?;
      let name = &record[0];
      let instructions = &record[1];

      self.conn.execute(
        "INSERT OR IGNORE INTO recipes (name, instructions) VALUES (?1, ?2)",
        params![name, instructions]
      ).expect("Failed to insert recipe");
    }

    Ok(())
  }

  /// Fetch all ingredients
  pub fn get_ingredients(&self) {
    let mut stmt = self.conn.prepare("SELECT id, name, category FROM ingredients").unwrap();
    let ingredient_iter = stmt.query_map([], |row| {
      Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
    }).unwrap();

    for ingredient in ingredient_iter {
      let (id, name, category) = ingredient.unwrap();
      println!("Ingredient {}: {} ({})", id, name, category);
    }
  }

  /// Fetch all recipes
  pub fn get_recipes(&self) {
    let mut stmt = self.conn.prepare("SELECT id, name, instructions FROM recipes").unwrap();
    let recipe_iter = stmt.query_map([], |row| {
      Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
    }).unwrap();

    for recipe in recipe_iter {
      let (id, name, instructions) = recipe.unwrap();
      println!("Recipe {}: {}\nInstructions: {}", id, name, instructions);
    }
  }
}
