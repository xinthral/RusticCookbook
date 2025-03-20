use crate::recipes::Recipe;
use std::fs;
use std::io::{self, Write};

pub struct Cookbook {
  pub file_path: String,
  pub file_name: String,
  pub recipes: Vec<Recipe>,
}

impl Cookbook {
  pub fn new(file_path: &str, file_name: &str) -> Self {
    Self {
      file_path: file_path.to_string(),
      file_name: file_name.to_string(),
      recipes: Vec::new(),
    }
  }

  pub fn add_recipe(&mut self, recipe: Recipe) {
    self.recipes.push(recipe);
  }

  pub fn list_recipes(&self) {
    if self.recipes.is_empty() {
      println!("No recipes found in the cookbook.");
    } else {
      for (i, recipe) in self.recipes.iter().enumerate() {
        println!("{}. {}", i + 1, recipe.name);
      }
    }
  }

  pub fn save_to_file(&self) -> io::Result<()> {
    let file_path = format!("{}/{}", self.file_path, self.file_name);
    let content = self.recipes
      .iter()
      .map(|r| r.display())
      .collect::<Vec<_>>()
      .join("\n\n");

    fs::write(&file_path, content)?;
    println!("Cookbook saved to {}", file_path);
    Ok(())
  }

  pub fn load_from_file(&mut self) -> io::Result<()> {
    let file_path = format!("{}/{}", self.file_path, self.file_name);
    let contents = fs::read_to_string(&file_path)?;

    println!("Loaded Cookbook Contents:\n{}", contents);
    Ok(())
  }
}
