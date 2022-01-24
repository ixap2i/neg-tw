use yew::prelude::*;
use std::fmt;
use chrono::{ Local, DateTime };

struct Draft {
  id: String,
  title: String,
  body: String,
  link: String,
  published: bool
}

impl Draft {
  fn new(&self) -> Draft {
    Draft {
      id: "id".to_string(),
      title: "title".to_string(),
      body: "body".to_string(),
      link: "link".to_string(),
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
      title: "■このアプリについて".to_string(),
      body: r#"・Rust x Wasmの組み合わせを学習するために作成しました。
      今後はWebスクレイピングの機能を入れます。(ほぼWebフレームワークとして使っているので、機能については構想中。)
      "#.to_string(),
      link: "".to_string(),
      published: false
    },
    Draft {
      id: "2".to_string(),
      title: "■環境".to_string(),
      body: r#"・Rust:1.24.0, Docker, trunk, Google Cloud Run / Container Registry
      "#.to_string(),
      link: "".to_string(),
      published: false
    },
    Draft {
      id: "3".to_string(),
      title: "■Rustのよいところ".to_string(),
      body: r#"・勉強させていただいた記事
      "#.to_string(),
      link: "https://python.ms/rust-or-go/#_1-%E6%A6%82%E8%A6%B3".to_string(),
      published: false
    }
  ];
  let drafts = dd.iter().map(|dobj| html! {
    <article class="article-box">
      <h2>{format!("{}", dobj.title)}</h2>
      <span>
        <p>{format!("{}", dobj.body)}</p>
        <a href={format!("{}", dobj.link)} class="sec" target="_blank">{format!("{}", dobj.link)}</a>
      </span>
    </article>
  }).collect::<Html>();


  html! {
    <div>
      <h1>{ "neg-tw" }</h1>
      <article>{ drafts }</article>
    </div>
  }
}

fn main() {
  yew::start_app::<App>();
}