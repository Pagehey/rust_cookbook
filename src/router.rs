use crate::app::controller::Controller;
use std::{io, error::Error};

pub struct Router<'a> {
  controller: &'a mut Controller<'a>,
  running: bool
}

impl<'a> Router<'a> {
  pub fn new(controller: &'a mut Controller<'a>) -> Self {
      Router { controller, running: false }
  }

  pub fn run(&mut self) -> Result<(), csv::Error> {
      println!("Welcome to the Cookbook!");
      println!("------------------------");
      println!("What do you want to do?");

      self.running = true;

      while self.running {
          self.display_actions();
          let action = self.ask_for_action();
          match action {
              Ok(action) => self.route_action(action)?,
              Err(_) => {
                  println!("You must enter a valid number");
              }
          }
      }
      Ok(())
  }

  fn display_actions(&self) {
      println!("1 - List recipes");
      println!("2 - Add a recipe");
      println!("3 - Remove a recipe");
      println!("4 - Quit");
  }

  fn ask_for_action(&self) -> Result<u8, Box<dyn Error>> {
      let mut action = String::with_capacity(1);
      io::stdin()
          .read_line(&mut action)?;

      let action: u8 = action.trim().parse()?;

      Ok(action)
  }

  fn route_action(&mut self, action: u8) -> Result<(), csv::Error> {
      match action {
          1 => self.controller.index(),
          2 => self.controller.create()?,
          3 => self.controller.destroy()?,
          4 => {
              self.running = false;
              println!("Bye!");
          },
          _ => println!("You must enter a number between 1 and 4!")
      }
      Ok(())
  }
}
