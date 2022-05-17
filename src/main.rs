use std::error::Error;

mod app;
mod router;

use router::Router;

use app::{cookbook::Cookbook, controller::Controller};

fn main() -> Result<(), Box<dyn Error>> {
    let mut cookbook = Cookbook::new("data/recipes.csv".to_string())?;
    let mut controller = Controller::new(&mut cookbook);

    let mut router = Router::new(&mut controller);

    router.run()?;

    Ok(())
}
