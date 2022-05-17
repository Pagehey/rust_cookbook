pub struct Recipe {
  pub name: String,
  pub description: String
}

impl Recipe {
  pub fn new(name: String, description: String) -> Self {
      Recipe { name, description }
  }
}
