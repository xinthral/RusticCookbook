use std::borrow::Cow;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

use crate::cookbook::utilz::generate_uuid;
use super::ingredients::*;
use super::recipes::*;
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
      if let Some((name, category)) = line.split_once(",") {
        let itype: IngredientType = self.translate_ingredient_category(category);
        self.add_ingredient(Ingredient::new(&generate_uuid(), itype, &name));
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
      if let Some((name, category)) = line.split_once(",") {
        if let Some((category, ingredients)) = category.split_once(",") {
          let rtype: RecipeType = self.translate_recipe_category(category);
          self.add_recipe(Recipe::new(&generate_uuid(), rtype, &name, &ingredients.replace(";", " ")));
        }
      }
    }
    recipelist.extend(self.recipes.0.clone());
    println!("Loaded Recipes Contents: {}", self.recipes.0.len());
    Ok(())
  }
  pub fn load_from_database(&mut self, file_path: &PathBuf, database_name: &str) -> Result<()> {
    let mut ip: PathBuf = file_path.clone();
    ip.push(database_name);
    let dbname: Cow<'_, str> = ip.to_string_lossy();
    println!("Loading ingredients from [{}]", dbname);
    let _db: SQLiteConnection = SQLiteConnection::new(&dbname).expect("Failed to connect to database");
    Ok(())
  }
  pub fn save_to_database(&mut self, file_path: &PathBuf, database_name: &str) -> Result<()> {
    let mut ip: PathBuf = file_path.clone();
    ip.push(database_name);
    let dbname: Cow<'_, str> = ip.to_string_lossy();
    println!("Saving ingredients to [{}]", dbname);
    let mut db: SQLiteConnection = SQLiteConnection::new(&dbname).expect("Failed to connect to database");
    let _ = db.store_ingredients(&self.ingredients);
    let _ = db.store_recipes(&self.recipes);
    Ok(())
  }
  fn translate_ingredient_category(&self, category: &str) -> IngredientType {
    IngredientType::from_str(category).unwrap_or(IngredientType::Pending)
  }
  fn translate_recipe_category(&self, category: &str) -> RecipeType {
    RecipeType::from_str(category).unwrap_or(RecipeType::Pending)
  }
}