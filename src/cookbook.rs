pub mod ingredients;
use ingredients::Ingredient;
pub mod recipes;
use recipes::Recipe;
pub mod queries;
// use queries::{IngredientQuery, RecipeQuery};
pub mod registrar;

use std::path::PathBuf;

// use queries::SQLiteHandler;

pub struct Cookbook {
  pub data_path:        PathBuf,
  pub cookbook_name:    String,
  pub ingredient_book:  String,
  pub recipe_book:      String,
  pub database_name:    String,
  pub ingredients:      Vec<Ingredient>,
  pub recipes:          Vec<Recipe>,
  pub registrar:        registrar::Registry,
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
      registrar:        registrar::Registry::new(),
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

  pub fn list_recipes(&self) {
    if self.recipes.is_empty() {
      println!("No recipes found in the cookbook.");
    } else {
      for (i, recipe) in self.recipes.iter().enumerate() {
        println!("\n{}. {}", i + 1, recipe);
      }
    }
  }

  pub fn list_ingredients(&self) {
    if self.ingredients.is_empty() {
      println!("No ingredients found in the cookbook.");
    } else {
      for (i, ingredient) in self.ingredients.iter().enumerate() {
        println!("\n{}. {}", i + 1, ingredient);
      }
    }
  } 

}
