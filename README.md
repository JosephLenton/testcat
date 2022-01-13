# TestCat

Helpful macros for writing test cases.

## Examples

TestCat allows you to write your test cases out at the top of a file.
This is to improve readability. It makes it easier to see what test
cases exist within a file, which is especially useful on PR reviews.

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
