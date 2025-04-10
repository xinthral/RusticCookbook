use std::error::Error;
use std::path::PathBuf;

pub mod ingredients;
use ingredients::Ingredient;
pub mod recipes;
use recipes::Recipe;
pub mod queries;
// use queries::{IngredientQuery, RecipeQuery};
pub mod registrar;
use registrar::Registry;
pub mod utilz;
// use utilz::generate_uuid;

pub struct Cookbook {
  pub data_path:        PathBuf,
  pub cookbook_name:    String,
  pub ingredient_book:  String,
  pub recipe_book:      String,
  pub database_name:    String,
  pub ingredients:      Vec<Ingredient>,
  pub recipes:          Vec<Recipe>,
  pub registrar:        Registry,
}
impl Cookbook {
  pub fn new(data_path: &PathBuf, cookbook_name: &str, ingredient_book: &str, recipe_book: &str, database_name: &str) -> Self {
    Self {
      data_path:        data_path.clone(),
      cookbook_name:    cookbook_name.to_string(),
      database_name:    database_name.to_string(),
      ingredient_book:  ingredient_book.to_string(),
      recipe_book:      recipe_book.to_string(),
      ingredients:      Vec::new(),
      recipes:          Vec::new(),
      registrar:        Registry::new(),
    }
  }
  pub fn add_recipe(&mut self, recipe: &Recipe) {
    let cloned: Recipe = recipe.clone();
    self.recipes.push(cloned);
  }
  pub fn add_ingredient(&mut self, ingredient: &Ingredient) {
    let cloned: Ingredient = ingredient.clone();
    self.ingredients.push(cloned);
  }
  pub fn list_recipes(&self, debugger: bool) {
    if self.recipes.is_empty() {
      println!("No recipes found in the cookbook.");
    } else {
      println!("Listing Recipes: ");
      for (i, recipe) in self.recipes
        .iter()
        .enumerate() {
          if debugger {
            println!("{:width$}. {:?}", i + 1, recipe, width=3);
          } else {
            println!("{:width$}. {}", i + 1, recipe, width=3);
          }
      }
    }
  }
  pub fn list_ingredients(&self, debugger: bool) {
    if self.ingredients.is_empty() {
      println!("No ingredients found in the cookbook.");
    } else {
      for (i, ingredient) in self.ingredients
        .iter()
        .enumerate() {
          if debugger {
            println!("{:width$}. {:?}", i + 1, ingredient, width=3);
          } else {
            println!("{:width$}. {}", i + 1, ingredient, width=3);
          }
      }
    }
  }
  pub fn load_from_file(&mut self) -> Result<(), Box<dyn Error>> {
    self.registrar.load_ingredients_from_file(&self.data_path, &self.ingredient_book, &mut self.ingredients)?;
    self.registrar.load_recipes_from_file(&self.data_path, &self.recipe_book, &mut self.recipes)?;
    Ok(())
  }
  pub fn load_from_database(&mut self) -> Result<(), Box<dyn Error>> {
    self.registrar.load_from_database(&self.data_path, &self.database_name)?;
    Ok(())
  }
}
