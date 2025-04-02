use super::ingredients::{self, IngredientList};
use std::fmt;

pub struct RecipeList(pub Vec<Recipe>);
impl fmt::Display for RecipeList {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for recipe in &self.0 {
      write!(f, "- {}\n", recipe)?;
    }
    Ok(())
  }
}
impl RecipeList {
  pub fn new() -> Self {
    Self(
      Vec::new(),
    )
  }
  pub fn add(&mut self, recipe: Recipe) {
    self.0.push(recipe);
  }
  pub fn pop(&mut self) -> Option<Recipe> {
    self.0.pop()
  }
  pub fn clear(&mut self) {
    self.0.clear();
  }
}

pub struct Recipe {
  pub name: String,
  pub ingredients: ingredients::IngredientList,
  pub instructions: String,
}

impl fmt::Display for Recipe {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Recipe: {} (Ingredients: {})\nInstructions: {}", self.name, self.ingredients, self.instructions)
  }
}

impl Recipe {
  pub fn new(name: &str, instructions: &str) -> Self {
    Recipe {
      name: name.to_string(),
      ingredients: IngredientList::new(),
      instructions: instructions.to_string(),
    }
  }
  
  pub fn add_ingredient(&mut self, ingredient: ingredients::Ingredient) {
    self.ingredients.0.push(ingredient);
  }
  
  pub fn display(&self) -> String {
    format!("Recipe: {} (Ingredients: {})", self.name, self.ingredients)
  }
  
  pub fn display_ingredients(&self) {
    for ingredient in &self.ingredients.0 {
      println!("{}", ingredient);
    }
  }

  pub fn remove_ingredient(&mut self, ingredient: &str) {
    self.ingredients.0.retain(|i| i.name != ingredient);
  }
}