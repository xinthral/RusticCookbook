use crate::ingredients::Ingredient;

#[derive(Debug, Clone)]
pub struct Recipe {
  pub name: String,
  pub ingredients: Vec<Ingredient>,
  pub instructions: String,
}

impl Recipe {
  pub fn new(name: &str, ingredients: Vec<Ingredient>, instructions: &str) -> Self {
    Self {
      name: name.to_string(),
      ingredients,
      instructions: instructions.to_string(),
    }
  }

  pub fn display(&self) -> String {
    let ing_list = self.ingredients
      .iter()
      .map(|ing| format!("- {}", ing.display()))
      .collect::<Vec<_>>()
      .join("\n");

    format!(
      "Recipe: {}\nIngredients:\n{}\nInstructions:\n{}",
      self.name, ing_list, self.instructions
    )
  }
}
