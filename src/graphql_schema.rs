use juniper::{EmptyMutation, RootNode};

struct User {
    id: i32,
    username: String,
    email: String
}

#[juniper::object(description = "Registered user")]
impl User {
    pub fn id(&self) -> i32 {
      self.id  
    }
  
    pub fn username(&self) -> &str {
      self.username.as_str()
    }

    pub fn email(&self) -> &str {
        self.email.as_str()
    }
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
  fn users() -> Vec<User> {
    vec![
      User {
        id: 1,
        username: "Ryoji Kaji".to_owned(),
        email:"kaji@nerv.com".to_owned()
      },
      User {
        id: 2,
        username: "Shinji Ikari".to_owned(),
        email:"ikari@nerv.com".to_owned()
      },
      User {
        id: 3,
        username: "Misato Katsuragi".to_owned(),
        email:"katsuragi@nerv.com".to_owned()
      }
    ]
  }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;
    
pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}
