#[macro_use]
extern crate diesel;
extern crate neg_tw;

use yew::prelude::*;
use self::diesel::prelude::*;
use self::neg_tw::*;

pub mod datas;
use datas::drafts::Draft;
use datas::posts::Post;

enum Msg {
    HogeMethod,
    DisplayDraft
}

 struct Model {
    value: i64
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(context: &Context<Self>) -> Self {
        Self {
            value: 1
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HogeMethod => {
                self.value += 1;
            }
            Msg::DisplayDraft => {
                let _draft_obj = vec![
                    Draft {
                        id: 1i8,
                        title: "this is title".to_string(),
                        body: "this is body".to_string(),
                        link: "https:///".to_string(),
                        published: false
                    },
                    Draft {
                        id: 2i8,
                        title: "this is title".to_string(),
                        body: "this is body".to_string(),
                        link: "https:///".to_string(),
                        published: false
                    }
                ];
            }
        }
        true
    }

    fn view(&self, context: &Context<Self>) -> Html {
        html! {
            <div>
            <button onclick={context.link().callback(|_| Msg::HogeMethod)}>{ "+1" }</button>
            <p>{ self.value }</p>
            <div>
                {
                    draft_obj.iter().for_each(|draft: &Draft| println!("{:#?}: {:#?}", draft.id, draft.title))
                }
            </div>
            </div>
        }
    }
}
 fn main() {
    use neg_tw::schema::posts::dsl::*;
    let connection = establish_connection();
    let results = posts
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
    // yew::start_app::<Model>();
}
