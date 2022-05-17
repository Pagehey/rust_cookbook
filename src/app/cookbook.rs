extern crate csv;

use crate::app::recipe::Recipe;

pub struct Cookbook {
  recipes: Vec<Recipe>,
  csv_file_path: String
}

impl Cookbook {
  pub fn new(csv_file_path: String) -> Result<Self, csv::Error> {
      let mut cookbook = Cookbook { recipes: Vec::<Recipe>::new(), csv_file_path };
      cookbook.load_csv()?;
      Ok(cookbook)
  }

  pub fn all(&self) -> &Vec<Recipe> {
      &self.recipes
  }

  pub fn add_recipe(&mut self, recipe: Recipe) -> Result<(), csv::Error> {
      self.recipes.push(recipe);
      self.save_csv()?;
      Ok(())
  }

  pub fn remove_recipe(&mut self, index: usize) -> Result<(), csv::Error> {
      let exists = self.recipes.get(index);

      if let Some(_) = exists {
          self.recipes.remove(index);
          self.save_csv()?;
      }
      Ok(())
  }

  fn load_csv(&mut self) -> Result<(), csv::Error> {
      let mut reader = csv::ReaderBuilder::new()
          .has_headers(false)
          .from_path(&self.csv_file_path)?;

      for result in reader.records() {
          let record = result?;
          let recipe = Recipe::new(record[0].into(), record[1].into());
          self.add_recipe(recipe)?;
      }

      Ok(())
  }

  fn save_csv(&self) -> Result<(), csv::Error> {
      let mut writer = csv::WriterBuilder::new()
          .has_headers(false)
          .from_path(&self.csv_file_path)?;

      for recipe in self.recipes.iter() {
          writer.write_record([&recipe.name, &recipe.description])?;
      }

      writer.flush()?;

      Ok(())
  }
}
