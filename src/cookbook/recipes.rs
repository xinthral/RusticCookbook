use super::ingredients::{Ingredient, IngredientList};
use std::fmt;

/** RecipeList */
#[derive(Clone)]
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

/** RecipeType */
#[derive(Clone)]
pub enum RecipeType {
  Culinary,
  Protein,
  Produce,
  Grain,
  Craft,
}
impl fmt::Display for RecipeType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let category: String = match self {
      RecipeType::Culinary => "Culinary".to_string(),
      RecipeType::Protein => "Protein".to_string(),
      RecipeType::Produce => "Produce".to_string(),
      RecipeType::Grain => "Grain".to_string(),
      RecipeType::Craft => "Craft".to_string(),
    };
    write!(f, "{}", category)
  }
}

/** Recipe */
#[derive(Clone)]
pub struct Recipe {
  pub uuid: String,
  pub name: String,
  pub ingredients: IngredientList,
  pub instructions: String,
}
impl fmt::Display for Recipe {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "\nRecipe: {} [\n{}\n]\nInstructions: {}", self.name, self.ingredients, self.instructions)
  }
}
impl Recipe {
  pub fn new(uuid: &str, name: &str, instructions: &str) -> Self {
    Recipe {
      uuid: uuid.to_string(),
      name: name.to_string(),
      ingredients: IngredientList::new(),
      instructions: instructions.to_string(),
    }
  }
  pub fn add_ingredient(&mut self, ingredient: &Ingredient) {
    let cloned: Ingredient = ingredient.clone();
    self.ingredients.0.push(cloned);
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