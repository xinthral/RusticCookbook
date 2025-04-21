pub mod cookbook;

use std::path::PathBuf;
use std::env;

use cookbook::Cookbook;
use crate::cookbook::registrar;
use registrar::Registry;

fn init_cookbook(data_path: &PathBuf) -> Cookbook {
  Cookbook {
    data_path:        data_path.clone(),
    cookbook_name:    "testbook.ckb".to_string(),
    database_name:    "test.db".to_string(),
    ingredient_book:  "old_i.ckb".to_string(),
    recipe_book:      "old_r.ckb".to_string(),
    ingredients:      Vec::new(),
    recipes:          Vec::new(),
    registrar:        Registry::new(),
  }
}

fn main() {
  println!("\n##### Cookbook Begins! #####");

  let mut data_path: PathBuf = env::current_dir().expect("Failed to get current directory");
  data_path.push("data");

  println!("Data Path: {}", data_path.display());

  let mut cookbook: Cookbook = init_cookbook(&data_path);
  let _ = cookbook.load_from_file();
  // let _ = cookbook.load_from_database();
  // cookbook.list_recipes(true);  // List recipes in the cookbook
  // cookbook.list_ingredients(true);  // List ingredients in the cookbook
  // cookbook.registrar.display_ingredients();  // Display ingredients in the database
  // cookbook.registrar.display_recipes();  // Display ingredients in the database
  let _ = cookbook.save_to_database();  // Save cookbook to the database
  // let _ = cookbook.save_to_file();  // Save cookbook to the file

  println!("##### Cookbook Complete! #####\n");
}