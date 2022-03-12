#[test]
fn borrow_variable() {
    let test = "test";

    // testを共有参照している(参照の代入)
    let t = &test;
    // testを可変参照している
    // let t = &mut test;
    // 8行目がコメントアウトされている場合、可変オブジェクトとして借用しているため、print内で再度借用することは出来ない
    print!("{}", test);
    print!("{}", t);
}

fn local_scope() {
    {
        let out_of_scope;
        {
            let inner_scope = 1;
            // 借用された値は十分に生存できないとエラーが出る
            // ブロックを出るとinner_scopeはしぬ
            out_of_scope = &inner_scope;
            // 2. inner_scopeと同じ生存期間で使用されているのでOK
            assert_eq!(out_of_scope, &1);
        }
        // 1. だめ
        // assert_eq!(out_of_scope, 1);
    }
}

fn life_time() {
    fn print_v<'a>(target_v: &'a i32) {  print!("{}", target_v); }
    let vv = 10;
    print_v(&vv);
}