pub mod equals {
  pub fn call_nested_equals() {
		self::nested_equals::option();
  }
  // 2^8で構成された配列の中身が同じかを判定する処理
  pub fn equals_array(a: Vec<u8>, b: Vec<u8>) -> bool {
		if a.len() != b.len() {
				return false;
		}
		let mut iter_a = a.iter();
		let mut iter_b = a.iter();
		while iter_a.len() > 0 {
			if iter_a.next() != iter_b.next() {
				return false;
			}
		}
		return true;
  }
  pub mod nested_equals {
		pub fn equals_by_nested_equals() {
			print!("nothing");
		}
		pub(in super) fn option() {
			print!("only visible from equals");
		}
  }
}

#[cfg(test)]
pub mod test1 {
  #[test]
  pub fn check_equals_array() {
    let a = vec![1, 2, 3];
    let b = vec![1, 2, 3];
    assert_eq!(true, crate::equals::equals::equals_array(a, b));
  }
}

