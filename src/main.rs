mod cookbook;
mod ingredients;
mod recipes;
mod sqlite_handler;

use cookbook::Cookbook;
use ingredients::Ingredient;
use recipes::Recipe;
use sqlite_handler::SQLiteHandler;
use std::fs;

fn main() {
  println!("\n##### Cookbook Begins! #####");

  let filepath = "/pond/Github/xTesting/rustic/cookbook/data".to_string();
  let filename = "testbook.ckb".to_string();
  let fullfile = format!("{}/{}", filepath, filename);
  println!("Checking File: {fullfile}");

  let _ = fs::read_to_string(&fullfile).unwrap_or_else(|_| {
    println!("File not found. Creating an empty cookbook...");
    "".to_string()
  });

  let chicken = Ingredient::new("Chicken", "Meat");
  let rice = Ingredient::new("Rice", "Grain");

  let chow_recipe = Recipe::new("Chicken Fried Rice", vec![chicken, rice], "Cook in a pan for 10 minutes.");
  println!("{}", chow_recipe.display());

  let mut ckb = Cookbook::new(&filepath, &filename);
  ckb.add_recipe(chow_recipe);
  ckb.list_recipes();
  ckb.save_to_file().expect("Failed to save the cookbook");

  let db = SQLiteHandler::new("cookbook.db");
  let ip = format!("{}/cookbook_i.ckb", filepath);

  println!("Loading ingredients...");
  db.load_ingredients(&ip).expect("Failed to load ingredients");

  let rp = format!("{}/cookbook_r.ckb", filepath);
  println!("Loading recipes...");
  db.load_recipes(&rp).expect("Failed to load recipes");

  println!("\n### Ingredients in Database ###");
  db.get_ingredients();

  println!("\n### Recipes in Database ###");
  db.get_recipes();


  println!("##### Cookbook Complete! #####\n");
}
