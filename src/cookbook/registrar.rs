
use super::ingredients::{Ingredient, IngredientList};
use super::recipes::{Recipe, RecipeList};
use std::io::Result;

pub struct Registry {
  ingredients: IngredientList,
  recipes: RecipeList,
}

impl Registry {
  pub fn new() -> Self {
    Registry {
      ingredients: IngredientList::new(),
      recipes: RecipeList::new(),
    }
  }

  pub fn add_ingredient(&mut self, ingredient: Ingredient) {
    self.ingredients.add(ingredient);
  }

  pub fn add_recipe(&mut self, recipe: Recipe) {
    self.recipes.add(recipe);
  }

  pub fn display_ingredients(&self) {
    println!("{}", self.ingredients);
  }

  pub fn display_recipes(&self) {
    println!("{}", self.recipes);
  }
  
  pub fn save_to_file(&self) -> Result<()> {
    // let mut file_path: PathBuf = self.file_path.clone();
    // file_path.push("data");
    // file_path.push(&self.cookbook_name);

    // let content = self.recipes
    //   .iter()
    //   .map(|r| r.d0.play())
    //   .collect::<Vec<_>>()
    //   .join("\n\n");

    // fs::write(&file_path, content)?;
    // println!("Cookbook saved to {}", &file_path.display());
    Ok(())
  }

  pub fn load_from_file(&mut self) -> Result<()> {
    // let mut file_path: PathBuf = self.file_path.clone();
    // file_path.push("data");
    // file_path.push(&self.cookbook_name);
    // let contents: String = fs::read_to_string(&file_path)?;

    // println!("Loaded Cookbook Contents:\n{}", &contents);
    Ok(())
  }

  pub fn load_from_database(&self) -> Result<()> {
    // let mut ip: PathBuf = self.file_path.clone();
    // ip.push("data");
    // ip.push(&self.database_name);
    
    // println!("Loading ingredients...");
    // let db: SQLiteHandler = SQLiteHandler::new(&self.file_path);
    // db.load_ingredients(&ip.to_string_lossy()).expect("Failed to load ingredients");

    Ok(())
  }

  pub fn save_to_database(&self) -> Result<()> {
    // let db: SQLiteHandler = SQLiteHandler::new(&self.file_path);
    // let mut ip: PathBuf = self.file_path.clone();
    // ip.push("data");
    // ip.push(&self.database_name);
    // println!("Saving recipes to database...");
    // // db.flush_to_disk(&ip.to_string_lossy(), &self.recipes).expect("Failed to save recipes");
    Ok(())
  }
}