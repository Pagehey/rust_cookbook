use crate::app::recipe::Recipe;
use std::io;

pub fn display_recipes(recipes: &Vec<Recipe>) {
  recipes.iter().enumerate().for_each(|(index, recipe)| {
      println!("{}/ {} ({})", index + 1, recipe.name, recipe.description);
  })
}

pub fn ask_for(attr_name: &str) -> String {
  println!("What's the recipe's {}?", attr_name);

  let mut answer = String::new();
  io::stdin()
      .read_line(&mut answer)
      .expect("Failed to read line");

  answer.trim().into()
}
