use super::ingredients::{Ingredient, IngredientList};
use std::fmt::{Debug, Display, Formatter, Result};
use strum_macros::EnumIter;

/** RecipeList */
#[derive(Clone)]
pub struct RecipeList(pub Vec<Recipe>);
impl Debug for RecipeList {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    for recipe in &self.0 {
      write!(f, "[{:width$}] {} ({}) :: {}\n", recipe.uuid, recipe.name, recipe.category, recipe.instructions, width=35)?;
    }
    Ok(())
  }
}
impl Display for RecipeList {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    for recipe in &self.0 {
      write!(f, "{}", recipe.name)?;
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
macro_rules! recipe_types {
  ($($variant:ident => $name:expr),*) => {
    #[derive(Clone, Debug, EnumIter, PartialEq, Eq)]
    pub enum RecipeType {
      $($variant),*
    }
    impl Display for RecipeType {
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.as_str())
      }
    }
    impl RecipeType {
      pub fn as_str(&self) -> &'static str {
        match self {
          $(RecipeType::$variant => $name),*
        }
      }
      pub fn from_str(name: &str) -> Option<Self> {
        match name {
          $(x if x.eq_ignore_ascii_case($name) => Some(RecipeType::$variant)),*,
          _ => None,
        }
      }
      pub fn all() -> &'static [RecipeType] {
        &[$(RecipeType::$variant),*]
      }
    }
  };
}
recipe_types! {
    Baked     => "Baked",
    Boiled    => "Boiled",
    Brewed    => "Brewed",
    Crafted   => "Crafted",
    Fried     => "Fried",
    Grilled   => "Grilled",
    Mixed     => "Mixed",
    Roasted   => "Roasted",
    Sautéed   => "Sautéed",
    Steamed   => "Steamed",
    Simmered  => "Simmered",
    Pending   => "Pending"
}
/** Recipe */
#[derive(Clone)]
pub struct Recipe {
  pub uuid:         String,
  pub category:     RecipeType,
  pub name:         String,
  pub ingredients:  IngredientList,
  pub instructions: String,
}
impl Debug for Recipe {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[{}] \n\tName: ({})\n\tIngredients: ({})\n\tInstructions: ({})", self.uuid, self.name, self.ingredients, self.instructions)
  }
}
impl Display for Recipe {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.name)
  }
}
impl Recipe {
  pub fn new(uuid: &str, category: RecipeType, name: &str, instructions: &str) -> Self {
    Recipe {
      uuid:         uuid.to_string(),
      category,
      name:         name.to_string(),
      ingredients:  IngredientList::new(),
      instructions: instructions.to_string(),
    }
  }
  pub fn add_ingredient(&mut self, ingredient: &Ingredient) {
    let cloned: Ingredient = ingredient.clone();
    self.ingredients.0.push(cloned);
  }
  pub fn display_ingredients(&self) {
    for ingredient in &self.ingredients.0 {
      println!("{}", ingredient);
    }
  }
  pub fn remove_ingredient(&mut self, ingredient: &str) {
    self.ingredients.0.retain(|i| i.name != ingredient);
  }
  pub fn set_type(&mut self, new_type: RecipeType) {
    self.category = new_type;
  }
}