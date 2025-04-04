pub mod cookbook;
pub mod utilz;

use std::path::PathBuf;
use std::env;

use cookbook::Cookbook;
use crate::cookbook::ingredients;
use ingredients::{Ingredient, IngredientType};
use crate::cookbook::recipes;
use recipes::{Recipe, RecipeType};
use utilz::generate_uuid;

fn main() {
  println!("\n##### Cookbook Begins! #####");

  let mut data_path:        PathBuf = env::current_dir().expect("Failed to get current directory");
  data_path.push("data");

  println!("Data Path: {}", data_path.display());

  let cookbook_name:        String  = String::from("testbook.ckb");
  let cookbook_ingredients: String  = String::from("cookbook_i.ckb");
  let cookbook_recipes:     String  = String::from("cookbook_r.ckb");
  let database_name:        String  = String::from("testdb.db");

  let mut cookbook: Cookbook = Cookbook::new(&data_path, &cookbook_name, &cookbook_ingredients, &cookbook_recipes, &database_name);
  
  let butter: Ingredient = Ingredient::new(&generate_uuid(), "Butter", IngredientType::Culinary);
  let chicken: Ingredient = Ingredient::new(&generate_uuid(), "Chicken", IngredientType::Protein);
  println!("Generated UUID: {}", butter.uuid);

  cookbook.add_ingredient(&butter);
  cookbook.add_ingredient(&chicken);
  cookbook.list_ingredients();  // List ingredients in the cookbook


  let mut buttered_chicken: Recipe = Recipe::new(&generate_uuid(), "Buttered Chicken", "In a large skillet, sear it up.");
  buttered_chicken.add_ingredient(&butter);
  buttered_chicken.add_ingredient(&chicken);
  cookbook.add_recipe(&buttered_chicken);
  cookbook.list_recipes();  // List recipes in the cookbook
  println!("##### Cookbook Complete! #####\n");
}
