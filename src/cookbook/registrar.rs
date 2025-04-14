use std::fs;
use std::io::Result;
use std::path::PathBuf;

use crate::cookbook::utilz::generate_uuid;

use super::ingredients::{Ingredient, IngredientList, IngredientType};
use super::recipes::{Recipe, RecipeList, RecipeType};
use super::queries::SQLiteConnection;
// use super::utilz::generate_uuid;

pub struct Registry {
  ingredients: IngredientList,
  recipes: RecipeList,
}

impl Registry {
  pub fn new() -> Self {
    Self {
      ingredients: IngredientList::new(),
      recipes: RecipeList::new(),
    }
  }
  pub fn add_ingredient(&mut self, ingredient: Ingredient) {
    self.ingredients.add(ingredient);
  }
  pub fn add_recipe(&mut self, recipe: Recipe) {
    self.recipes.add(recipe);
  }
  pub fn display_ingredients(&self) {
    println!("{:?}", self.ingredients);
  }
  pub fn display_recipes(&self) {
    println!("{:?}", self.recipes);
  }
  pub fn save_to_file(&self) -> Result<()> {
    // let mut file_path: PathBuf = self.file_path.clone();
    // file_path.push("data");
    // file_path.push(&self.cookbook_name);

    // let content = self.recipes
    //   .iter()
    //   .map(|r| r.d0.play())
    //   .collect::<Vec<_>>()
    //   .join("\n\n");

    // fs::write(&file_path, content)?;
    // println!("Cookbook saved to {}", &file_path.display());
    Ok(())
  }
  pub fn load_ingredients_from_file(&mut self, file_path: &PathBuf, ingredient_book_name: &str, ingredientlist: &mut Vec<Ingredient>) -> Result<()> {
    let mut file_path: PathBuf = file_path.clone();
    file_path.push(ingredient_book_name);
    println!("Loading Ingredients from {}", &file_path.display());

    let contents: String = fs::read_to_string(&file_path)?;
    for line in contents.lines().filter(|line| !line.starts_with("name")) {
      if let Some((name, _category)) = line.split_once(",") {
        let itype: IngredientType = 
        self.add_ingredient(Ingredient::new(&generate_uuid(), IngredientType::Pending, &name));
      }
    }
    ingredientlist.extend(self.ingredients.0.clone());
    println!("Loaded Ingredients Contents: {}", self.ingredients.0.len());
    Ok(())
  }
  pub fn load_recipes_from_file(&mut self, file_path: &PathBuf, recipe_book_name: &str, recipelist: &mut Vec<Recipe>) -> Result<()> {
    let mut file_path: PathBuf = file_path.clone();
    file_path.push(recipe_book_name);
    println!("Loading Recipes from {}", &file_path.display());

    let contents: String = fs::read_to_string(&file_path)?;

    for line in contents.lines().filter(|line| !line.starts_with("name")) {
      if let Some((name, ingredients)) = line.split_once(",") {
        self.add_recipe(Recipe::new(&generate_uuid(), RecipeType::Pending, &name, &ingredients.replace(";", " ")));
      }
    }
    recipelist.extend(self.recipes.0.clone());
    println!("Loaded Recipes Contents: {}", self.recipes.0.len());
    Ok(())
  }
  pub fn load_from_database(&mut self, file_path: &PathBuf, database_name: &str) -> Result<()> {
    let mut ip: PathBuf = file_path.clone();
    ip.push(database_name);
    let dbname = ip.to_string_lossy();
    
    println!("Loading ingredients from [{}]", dbname);
    // let mut db: SQLiteConnection = SQLiteConnection::new(&dbname).unwrap();
    let _db: SQLiteConnection = SQLiteConnection::new(&dbname).expect("Failed to connect to database");
    // db.load_ingredients(&dbname, &mut self.ingredients).expect("Failed to load ingredients");
    Ok(())
  }
  pub fn save_to_database(&self) -> Result<()> {
    // let db: SQLiteHandler = SQLiteHandler::new(&self.file_path);
    // let mut ip: PathBuf = self.file_path.clone();
    // ip.push("data");
    // ip.push(&self.database_name);
    // println!("Saving recipes to database...");
    // db.flush_to_disk(&ip.to_string_lossy(), &self.recipes).expect("Failed to save recipes");
    Ok(())
  }
}