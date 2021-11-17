#[macro_export]
macro_rules! array {
  [] => ($crate::value::JsonValue::Array(Vec::new()));

  [ $($item:expr),*]  => ({
    let mut arr = Vec::new();

    $(
      arr.push($item.into());
    )*

    $crate::value::JsonValue::Array(arr)
  });
}

#[macro_export]
macro_rules! object {
    {} => ($crate::value::JsonValue::Object(std::collections::BTreeMap::new()));

    { $($key:expr => $val:expr), * } => ({
      use std::collections::BTreeMap;

      let mut obj = BTreeMap::new();

      $(
        obj.insert($key.into(), $val.into());
      )*

      $crate::value::JsonValue::Object(obj)

    });
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod tests {
        #[test]
        fn macro_test() {
            let arr = array![1, 2, "3"];
            let obj = object! {"name"=>"abc", "age" => 123};

            println!("arr {:?}", arr);
            println!("obj {:?}", obj);
        }
    }
}
