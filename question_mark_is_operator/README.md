# question_mark_is_operator

* **Question:** For the same `anyhow::Error` type, the `function_a()` method in `lib.rs` can directly return it using the `?` operator. However, in the `function_b()` and `function_c()` methods in `business.rs`, when using the `anyhow!()` macro or directly creating an `anyhow::Error` type, I need to use `into()` to return the error, which is very strange. If I use `into()` for conversion in `function_b` and `function_c`, I get the following error:

  ```bash
  mismatched types
  `anyhow::Error` and `Error` have similar names, but are actually distinct types
  ```

  

* **Scenario:** The `Result` definitions for all methods are as follows:

  ```rust
  pub type Result<T> = std::result::Result<T, Error>;
  ```

  ​	`Error` is an enum type that uses the `thiserror` library, which allows all `anyhow::Error` types to automatically implement `From`. This makes it convenient to directly use the `?` operator to convert them into the `Error::AllMyWrong` type.

* Explanation:

  This involves the characteristics of the `?` operator. In Rust, the `?` operator is a convenient mechanism for propagating errors; it not only propagates the error but also automatically performs type conversion. For `function_a()`, which has a return type of `anyhow::Result<String>`, the `?` operator will convert any encountered error to the return type of the calling function, specifically converting it to `Result<_, Error>`. This conversion is done via the `From` trait. In other words, using `?` effectively executes code like the following:

  ```rust
  crate::function_a()?;
  ```

  It’s equivalent to executing the following code:

  ```rust
  match crate::function_a() {
      Ok(value) => value,
      Err(err) => return Err(Error::from(err)),
  }
  ```

  When I create an `anyhow::Error` type and want to return this error, the compiler does not automatically perform the conversion as it does with the `?` operator. Therefore, I need to manually add `into()`. Admittedly, this is not very elegant.