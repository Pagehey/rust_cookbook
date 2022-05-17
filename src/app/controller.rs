use crate::app::recipe::Recipe;
use crate::app::cookbook::Cookbook;

use crate::app::view;

pub struct Controller<'a> {
    cookbook: &'a mut Cookbook,
}

impl<'a> Controller<'a> {
    pub fn new(cookbook: &'a mut Cookbook) -> Self {
        Controller { cookbook }
    }

    pub fn index(&self) {
        self.display_recipes();
    }

    pub fn create(&mut self) -> Result<(), csv::Error> {
        let name = view::ask_for("name");
        let description = view::ask_for("description");
        let recipe = Recipe::new(name, description);
        self.cookbook.add_recipe(recipe)?;
        Ok(())
    }

    pub fn destroy(&mut self) -> Result<(), csv::Error> {
        self.display_recipes();
        let index: Result<usize, _> = view::ask_for("index").parse();
        if let Ok(index) = index {
            self.cookbook.remove_recipe(index - 1)?;
        }
        Ok(())
    }

    fn display_recipes(&self) {
        let recipes = &self.cookbook.all();
        view::display_recipes(&recipes)
    }
}
