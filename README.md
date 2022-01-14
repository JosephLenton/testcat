# TestCat

Helpful macros for writing test cases. Based on the JavaScript testing library Jest.

TestCat allows you to write your test cases out at the top of a file.
This is to improve readability. It makes it easier to see what test
cases exist within a file, which is especially useful on PR reviews.

Macros include ...

 * `it` and `test`
 * `describe`

## `it` and `test`

`it` and `test` are identical macros allow you to list test cases out together at the top.
These transform into a wrapper function, that calls your test.

For example ...

```
#[cfg(test)]
mod testing {
  use ::testcat::*;

  it!("should allow the user to do x", test_user_does_x);
  it!("should not allow the user to do y", test_y_disallowed);
  it!("should do foo before bar", test_foo_over_bar);

  fn test_user_does_x() {
    // code omitted
  }

  fn test_y_disallowed() {
    // code omitted
  }

  fn test_foo_over_bar() {
    // code omitted
  }
}
```

## `describe`

`describe` blocks allow you to wrap tests together.
Allowing you to gather behaviour into one block of tests.

These transform into a child module, where the tests are listed.

For example ...

```
#[cfg(test)]
mod testing {
  use ::testcat::*;

  describe("user interaction", {
    it!("should allow the user to do x", test_user_does_x);
    it!("should not allow the user to do y", test_y_disallowed);
  })

  describe("timing", {
    it!("should do foo before bar", test_foo_over_bar);
  })

  fn test_user_does_x() {
    // code omitted
  }

  fn test_y_disallowed() {
    // code omitted
  }

  fn test_foo_over_bar() {
    // code omitted
  }
}
```
