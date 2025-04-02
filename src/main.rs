pub mod cookbook;
pub mod utilz;
use cookbook::Cookbook;
use std::path::PathBuf;
use std::env;

fn main() {
  println!("\n##### Cookbook Begins! #####");

  let mut data_path:        PathBuf = env::current_dir().expect("Failed to get current directory");
  data_path.push("data");

  println!("Data Path: {}", data_path.display());

  let cookbook_name:        String  = String::from("testbook.ckb");
  let cookbook_ingredients: String  = String::from("cookbook_i.ckb");
  let cookbook_recipes:     String  = String::from("cookbook_r.ckb");
  let database_name:        String  = String::from(":memory:");

  let cookbook: Cookbook = Cookbook::new(&data_path, &cookbook_name, &cookbook_ingredients, &cookbook_recipes, &database_name);
  cookbook.list_recipes();  // List recipes in the cookbook

  let uuid: String = utilz::generate_uuid();
  println!("Generated UUID: {}", &uuid);

  println!("##### Cookbook Complete! #####\n");
}
