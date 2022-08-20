## Lifetime

Variable has lifetime.

```rust
fn main() {
  let a;       // --------------+-- a start
  {            //               |
    let b = 5; // -+-- b start  |
  }            // -+-- b over   |
}              // --------------+-- a over
```

Reference(the borrower) can not live longer than variable(the lender).

Example 1:

```rust
fn main() {
  let a;                // -------------+-- a start
  {                     //              |
    let b = 5;          // -+-- b start |
    a = &b;             //  |           |
  }                     // -+-- b over  |
  println!("a: {}", a); //              |
}                       // -------------+-- a over

error[E0597]: `b` does not live long enough
 --> src/main.rs:5:13
  |
5 |         a = &b;
  |             ^^ borrowed value does not live long enough
6 |     };
  |     - `b` dropped here while still borrowed
7 |     println!("a:{}", a);
  |                      - borrow later used here
```

### Lifetime in function

Example 2:

If we give the return value of `max_num` to z, and z will be one of the reference of what x or y refer to(X, Y). 

```rust
fn max_num(x: &i32, y: &i32) -> &i32 {
  if x > y {
    &x
  } else {
    &y
  }
}

error[E0106]: missing lifetime specifier
 --> src/main.rs:1:33
  |
1 | fn max_num(x: &i32, y: &i32) -> &i32 {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
```

Does x will live shorter than X and Y whaterver the value of X and Y are? If rust compiler allow the code above compiled successfully, and let x = 8, y = 1, and then max will be the reference of x and will live no longer than x. But when x = 1, y = 8, max will be the reference of y, and as a dangling point of courese when y is dropped.

```rust
fn main() {
  let x = 1;                // -------------+-- x start
  let max;                  // -------------+-- max start
  {                         //              |
    let y = 8;              // -------------+-- y start
    max = max_num(&x, &y);  //              |
  }                         // -------------+-- y over
  println!("max: {}", max); //              |
}                           // -------------+-- max, x over
```

Memory leaks at runtime are what rust tries to avoid，so your code must make sure x live shorter than X and Y. 

```rust
fn max_num<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
  if x > y {
    &x
  } else {
    &y
  }
}
```

``a` is the overlap region of X and Y lifetime, so the example above still compile failed:

```rust
error[E0597]: `y` does not live long enough
  --> src/main.rs:13:27
   |
13 |         max = max_num(&x, &y);
   |                           ^^ borrowed value does not live long enough
14 |     }
   |     - `y` dropped here while still borrowed
15 |     println!("max: {}", max);
   |                         --- borrow later used here
```

### Lifetime in struct

Lifetime of struct can not be longer than it's element when element is a reference.

```rust
#[derive(Debug)]
struct Foo<'a> {
  v: &'a i32
}

fn main() {
  let foo;                    // -------------+-- foo start
  {                           //              |
    let v = 123;              // -------------+-- v start
    foo = Foo {               //              |
      v: &v                   //              |
    }                         //              |
  }                           // -------------+-- v over
  println!("foo: {:?}", foo); //              |
}                             // -------------+-- foo over
```

``static` means live forever. string literal and static variable has static lifetime.

```rust
let s: &str = "codercat is a static lifetime.";
static V: i32 = 123;
```

## T: `static

`T` contains variables, references, for example `i32` and `& i32` .

| `T`                                                      | `&T`                              | `&mut T`                                      |
| -------------------------------------------------------- | --------------------------------- | --------------------------------------------- |
| `i32`, `&i32`, `&mut i32`, `&&i32`, `&mut &mut i32`, ... | `&i32`, `&&i32`, `&&mut i32`, ... | `&mut i32`, `&mut &mut i32`, `&mut &i32`, ... |

`T` is the superset of `&T` nad `&mut T` and `&T` are disjoint sets.

```rust
trait Trait {}

impl<T> Trait for T {}

impl<T> Trait for &T {} // compile error

impl<T> Trait for &mut T {} // compile error


error[E0119]: conflicting implementations of trait `Trait` for type `&_`:
 --> src/lib.rs:5:1
  |
3 | impl<T> Trait for T {}
  | ------------------- first implementation here
4 |
5 | impl<T> Trait for &T {}
  | ^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&_`

error[E0119]: conflicting implementations of trait `Trait` for type `&mut _`:
 --> src/lib.rs:7:1
  |
3 | impl<T> Trait for T {}
  | ------------------- first implementation here
...
7 | impl<T> Trait for &mut T {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut _`
```

A `T` has a constraint means it assure the comiler that `T` will live until the end of the scope.

```rust
pub fn read_in_background<T: Read + Send>(mut f: T) {
    thread::spawn(move || {
        let mut buf = Vec::<u8>::new();
        if let Ok(count) = f.read_to_end(&mut buf) {
            println!("read {} bytes from file.", count);
        }
    });
}

error[E0310]: the parameter type `T` may not live long enough
 --> src/lib.rs:6:5
  |
5 | pub fn read_in_background<T: Read + Send>(mut f: T) {
  |                           -- help: consider adding an explicit lifetime bound...: `T: 'static +`
6 |     thread::spawn(move || {
  |     ^^^^^^^^^^^^^ ...so that the type `[closure@src/lib.rs:6:19: 11:6]` will meet its required lifetime bounds
```

Here `T` can be a file or a reference of file, and compiler won't know how long the thread spawned live. If `T` is a reference, it may be a dangling point at a future moment. We must explicitly tell compiler `T` will live as long as the closure.

* We pass a file to read_in_background? It's ok, beacuse we move the owner of T to the closure.
* We pass a referece to read_in_background? Not a good idea.

