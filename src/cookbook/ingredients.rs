use std::fmt::{Debug, Display, Formatter, Result};
use std::vec::Vec;

/** IngredientList */
#[derive(Clone)]
pub struct IngredientList(pub Vec<Ingredient>);
impl Debug for IngredientList {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    for ingredient in &self.0 {
      write!(f, "{}", ingredient)?;
    }
    Ok(())
  }
}
impl Display for IngredientList {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    for ingredient in &self.0 {
      write!(f, "{}", ingredient)?;
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
macro_rules! ingredient_types {
  ($($variant:ident => $name:expr),*) => {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum IngredientType {
      $($variant),*
    }
    impl Display for IngredientType {
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.as_str())
      }
    }
    impl IngredientType {
      pub fn as_str(&self) -> &'static str {
        match self {
          $(IngredientType::$variant => $name),*
        }
      }
      pub fn from_str(name: &str) -> Option<Self> {
        match name {
          $(x if x.eq_ignore_ascii_case($name) => Some(IngredientType::$variant)),*,
          _ => None,
        }
      }
      pub fn all() -> &'static [IngredientType] {
        &[$(IngredientType::$variant),*]
      }
    }
  };
}
ingredient_types! {
  Condiment => "Condiment",
  Craft     => "Craft",
  Culinary  => "Culinary",
  Dairy     => "Dairy",
  Fat       => "Fat",
  Fish      => "Fish",
  Herb      => "Herb",
  Grain     => "Grain",
  Meat      => "Meat",
  Nut       => "Nut",
  Produce   => "Produce",
  Seasoning => "Seasoning",
  Sweetener => "Sweetener",
  Pending   => "Pending"
}

/** Ingredient */
#[derive(Clone)]
pub struct Ingredient {
  pub uuid:     String,
  pub category: IngredientType,
  pub name:     String,
}
impl Debug for Ingredient {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[{}] {} (Category: {})", self.uuid, self.name, self.category)
  }
}
impl Display for Ingredient {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.name)
  }
}
impl Ingredient {
  pub fn new(uuid: &str, category: IngredientType, name: &str) -> Self {
    Self {
      uuid:     uuid.to_string(),
      category,
      name:     name.to_string(),
    }
  }
  pub fn set_type(&mut self, category: IngredientType) {
    self.category = category;
  }
}