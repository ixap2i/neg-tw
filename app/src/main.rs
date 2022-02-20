pub mod datas;
use datas::drafts::Draft;
use yew::prelude::*;
/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/

 enum Msg {
    HogeMethod,
    DisplayDraft
}

 struct Model {
    value: i64,
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
            // <div>
            //     {
            //         draft_obj.iter().for_each(|draft: &Draft| println!("{:#?}: {:#?}", draft.id, draft.title))
            //     }
            // </div>
            </div>
        }
    }
}
 fn main() {
    yew::start_app::<Model>();
}

