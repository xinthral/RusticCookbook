use std::fmt;
use std::vec::Vec;

/** IngredientList */
#[derive(Clone)]
pub struct IngredientList(pub Vec<Ingredient>);
impl fmt::Display for IngredientList {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for ingredient in &self.0 {
      write!(f, "- {}\n", ingredient)?;
    }
    Ok(())
  }
}
impl IngredientList {
  pub fn new() -> Self {
    Self(
      Vec::new(),
    )
  }
  pub fn add(&mut self, ingredient: Ingredient) {
    self.0.push(ingredient);
  }
  pub fn pop(&mut self) -> Option<Ingredient> {
    self.0.pop()
  }
  pub fn clear(&mut self) {
    self.0.clear();
  }
}

/** IngredientType */
#[derive(Clone)]
pub enum IngredientType {
  Culinary,
  Protein,
  Produce,
  Grain,
  Craft,
}
impl fmt::Display for IngredientType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let category: String = match self {
      IngredientType::Culinary => "Culinary".to_string(),
      IngredientType::Protein => "Protein".to_string(),
      IngredientType::Produce => "Produce".to_string(),
      IngredientType::Grain => "Grain".to_string(),
      IngredientType::Craft => "Craft".to_string(),
    };
    write!(f, "{}", category)
  }
}

/** Ingredient */
#[derive(Clone)]
pub struct Ingredient {
  pub uuid: String,
  pub name: String,
  pub category: IngredientType,
}
impl fmt::Display for Ingredient {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "\nIngredient: {} (Category: {})", self.name, self.category)
  }
}
impl Ingredient {
  pub fn new(uuid: &str, name: &str, category: IngredientType) -> Self {
    Self {
      uuid: uuid.to_string(),
      name: name.to_string(),
      category,
    }
  }
}