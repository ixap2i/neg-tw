pub mod datas;
use datas::drafts::Draft;
/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
 fn main() {
    let draft_obj = vec![
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
    draft_obj.iter().for_each(|draft: &Draft| println!("{:?}: {:?}", draft.id, draft.title));
}