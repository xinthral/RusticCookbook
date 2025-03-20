#[derive(Debug, Clone)]
pub struct Ingredient {
  pub name: String,
  pub category: String, // Renamed `_type` to `category` (Rust convention)
}

impl Ingredient {
  pub fn new(name: &str, category: &str) -> Self {
    Self {
      name: name.to_string(),
      category: category.to_string(),
    }
  }

  pub fn display(&self) -> String {
    format!("Ingredient: {} (Category: {})", self.name, self.category)
  }
}
