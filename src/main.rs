use yew::prelude::*;
use std::fmt;

struct Draft {
  id: String,
  title: String,
  body: String,
  published: bool
}

impl Draft {
  fn new(&self) -> Draft {
    Draft {
      id: "id".to_string(),
      title: "title".to_string(),
      body: "body".to_string(),
      published: false
    }
  }
  pub fn add_text(&mut self, text: &str) {
    self.title.push_str(text);
  }
}

#[function_component(App)]

fn app() -> Html {

  let dd = vec![
    Draft {
      id: "1".to_string(),
      title: "title!".to_string(),
      body: "body!".to_string(),
      published: false
    }
  ];
  let drafts = dd.iter().map(|dobj| html! {
    <p>{format!("{}: {}", dobj.title, dobj.body)}</p>
  }).collect::<Html>();
  html! {
    <div>
      <h1>{ "Hello World" }</h1>
      <div>{ drafts }</div>
    </div>
  }
}

fn main() {
  yew::start_app::<App>();
}