@@lesson m10-advanced-traits Advanced Traits: Operators, Conversions and Associated Types
@@text
In Module 7 you wrote traits and used the common derived ones. The standard library is built on a few dozen more traits, and implementing them makes your own types feel built in: `a + b` works on your vectors, `?` converts your errors automatically, `money.into()` just works, and `for x in my_collection` loops. This lesson covers the traits that power Rust's syntax.

## Operator overloading with `std::ops`

Every operator is a trait. `a + b` is really `Add::add(a, b)`:

| Operator | Trait | Method |
|---|---|---|
| `a + b`, `a - b`, `a * b`, `a / b`, `a % b` | `Add`, `Sub`, `Mul`, `Div`, `Rem` | `add(self, rhs)`... |
| `-a`, `!a` | `Neg`, `Not` | `neg(self)`, `not(self)` |
| `a += b` and friends | `AddAssign`, `SubAssign`... | `add_assign(&mut self, rhs)` |
| `a[i]`, `a[i] = x` | `Index`, `IndexMut` | `index(&self, i)` |
| `a == b`, `a < b` | `PartialEq`, `PartialOrd` | usually derived |
| `*a` | `Deref`, `DerefMut` | `deref(&self)` |

```rust
use std::ops::{Add, Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

// Vec2 * f64: the right-hand side can be a different type
impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, k: f64) -> Vec2 {
        Vec2 { x: self.x * k, y: self.y * k }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 { x: -self.x, y: -self.y }
    }
}

fn main() {
    let pos = Vec2 { x: 1.0, y: 2.0 };
    let vel = Vec2 { x: 0.5, y: -1.0 };
    println!("{:?}", pos + vel * 2.0);
    println!("{:?}", -pos);
}
```

Overload operators only when the meaning is obvious (maths types, money, units). `+` on two `User`s would be a puzzle for readers.

## Associated types

`type Output = Vec2;` above is an **associated type**: a type that the implementation chooses, named inside the trait. You've used one before: `Iterator` has `type Item`. The difference from a generic parameter:

- **Generic parameter** (`Mul<f64>`): a type can implement the trait *many times*, once per parameter (`Vec2 * f64`, `Vec2 * Vec2`).
- **Associated type** (`Output`, `Item`): *one* choice per implementation. A `Vec<u8>` iterator has exactly one `Item` type, so you write `I: Iterator<Item = u8>` instead of juggling extra parameters.

Your own traits can have them too:

```rust,norun
trait Storage {
    type Key;
    type Error;
    fn get(&self, key: &Self::Key) -> Result<String, Self::Error>;
}
```

**Default generic parameters** let a trait mean "the same type" unless you say otherwise: `trait Add<Rhs = Self>`. That's why `impl Add for Vec2` needs no `<Vec2>`.

## Conversions: `From`, `Into`, `TryFrom`

| Trait | Meaning | Fails? |
|---|---|---|
| `From<T> for U` | build a `U` from a `T` | never |
| `Into<U> for T` | the same, from the other side: **implemented for you** when you implement `From` | never |
| `TryFrom<T> for U` | build a `U` from a `T`, maybe failing | returns `Result<U, Self::Error>` |
| `TryInto<U>` | the other side of `TryFrom`, also automatic | |

```rust
use std::convert::TryFrom;

#[derive(Debug)]
struct Percent(u8);

impl TryFrom<i32> for Percent {
    type Error = String;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        if (0..=100).contains(&v) {
            Ok(Percent(v as u8))
        } else {
            Err(format!("{v} is not a percentage"))
        }
    }
}

fn main() {
    println!("{:?}", Percent::try_from(42));
    let p: Result<Percent, _> = 150.try_into();
    println!("{:?}", p);
}
```

`From` is also what makes `?` convert errors: if you `impl From<std::io::Error> for AppError`, then `?` on an I/O result inside a function returning `Result<_, AppError>` converts the error automatically. That's how the custom error types from Module 5 become painless.

Accept `impl Into<String>` in constructors so callers can pass `&str` or `String`:

```rust,norun
fn new(name: impl Into<String>) -> Self { Self { name: name.into() } }
```

## `AsRef` and `Borrow`: accepting many forms cheaply

`AsRef<T>` means "can be viewed as a `&T` cheaply". The standard library's `File::open` takes `impl AsRef<Path>`, which is why it accepts `&str`, `String`, `&Path` and `PathBuf`. Use it for read-only parameters:

```rust,norun
fn word_count(text: impl AsRef<str>) -> usize {
    text.as_ref().split_whitespace().count()
}
```

`Borrow<T>` is similar but promises that hashing and comparison behave identically, which is why `HashMap<String, V>::get` accepts a `&str`.

## `Deref`: smart pointer behaviour

Implementing `Deref` lets `*value` and method calls reach the inner value. `Box`, `Rc`, `String` (to `str`) and `Vec` (to slices) all implement it, and **deref coercion** turns `&String` into `&str` and `&Vec<T>` into `&[T]` automatically when a function wants them.

Use `Deref` for your own smart pointers and **newtype** wrappers that should behave like their content (e.g. `struct Email(String)` exposing `&str` methods). Don't use it to fake inheritance between unrelated structs: Rust prefers composition and traits.

## `Display`, `Default`, `FromStr`, `IntoIterator`

- `Display` (`impl fmt::Display`): how `{}` prints your type; also gives you `.to_string()` for free.
- `Default`: a sensible empty value; `#[derive(Default)]` or implement by hand; `..Default::default()` fills the rest of a struct.
- `FromStr`: makes `"3,4".parse::<Point>()` work.
- `IntoIterator`: makes `for x in my_collection` work. Implement it for `&MyCollection` to loop by reference.

## The newtype pattern and the orphan rule

You can implement a trait for a type only if **the trait or the type is defined in your crate** (the **orphan rule**). It keeps two crates from implementing the same trait for the same type in conflicting ways. To implement `Display` for `Vec<String>`, wrap it: `struct Lines(Vec<String>);` and implement it for `Lines`. A newtype also adds type safety for free: `struct Meters(f64)` and `struct Feet(f64)` can't be mixed up.

## Supertraits and blanket implementations

- **Supertrait**: `trait Shape: Display { ... }` means anything that is a `Shape` must also be `Display`, so `Shape` methods can print `self`.
- **Blanket implementation**: `impl<T: Display> Describe for T { ... }` implements your trait for *every* type that meets the bound. The standard library does this: `impl<T: Display> ToString for T`.

## Fully qualified syntax

When two traits give a type methods with the same name, say which one: `<Dog as Animal>::name()` or `Pilot::fly(&person)`. You'll rarely need it, but compiler errors will suggest it when you do.

@@quiz
What does `type Output = Vec2;` in `impl Add for Vec2` declare?
* An associated type: the type the `+` operation produces for this implementation
- A type alias for the whole module
- A generic parameter that callers choose
? Associated types are chosen once per implementation.

@@quiz
You implemented `From<io::Error> for AppError`. What else do you get?
- Nothing else
* `Into<AppError>` for `io::Error`, and `?` converting I/O errors into `AppError` automatically
- `Display` for `AppError`
- A `TryFrom` implementation
? `Into` is implemented automatically from `From`.

@@quiz
Why does `File::open` accept `&str`, `String` and `PathBuf`?
- It's overloaded three times
* It takes `impl AsRef<Path>`, and all of these can be viewed as a `&Path`
- Rust converts strings to paths automatically everywhere
- It uses `Deref`
? `AsRef` is the "cheap view" conversion.

@@quiz
Why can't you `impl Display for Vec<String>` in your crate?
- `Vec` can't be printed
* The orphan rule: neither `Display` nor `Vec` is defined in your crate. Wrap it in a newtype
- `Display` is sealed
- You need `unsafe`
? `struct Lines(Vec<String>)` is yours, so you can implement anything for it.

@@exercise Money with operators
Implement a `Money` type that stores **cents** as an `i64`:

- `Add` and `Sub` for `Money + Money` and `Money - Money`
- `Mul<i64>` for `Money * quantity`
- `Neg` (a refund)
- `Display` printing like `$12.05` or `-$3.50`
- `From<i64>` for a whole number of dollars, e.g. `Money::from(3)` is 300 cents
- `Sum` so that an iterator of `Money` can be `.sum()`ed (implement `std::iter::Sum` with `fold`).
@@starter
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Money {
    cents: i64,
}

// your implementations here

fn main() {
    let coffee = Money { cents: 350 };
    let cake = Money::from(4);
    let order = coffee * 2 + cake;
    println!("{}", order);
    println!("{}", -coffee);
    let total: Money = vec![coffee, cake, Money { cents: 5 }].into_iter().sum();
    println!("{}", total - Money::from(10));
}
@@expected
$11.00
-$3.50
-$2.45
@@tests
#[test]
fn checks() {
    assert_eq!(Money { cents: 100 } + Money { cents: 5 }, Money { cents: 105 });
    assert_eq!(Money { cents: 100 } - Money { cents: 250 }, Money { cents: -150 });
    assert_eq!(Money { cents: 125 } * 3, Money { cents: 375 });
    assert_eq!(Money::from(2), Money { cents: 200 });
    assert_eq!(format!("{}", Money { cents: 1205 }), "$12.05");
    assert_eq!(format!("{}", Money { cents: -7 }), "-$0.07");
    assert_eq!(format!("{}", Money { cents: 0 }), "$0.00");
    let empty: Money = Vec::<Money>::new().into_iter().sum();
    assert_eq!(empty, Money { cents: 0 });
}
@@requires
impl Add for Money
impl Mul<i64> for Money
impl Sum for Money
@@hint
Each operator: `impl Add for Money { type Output = Money; fn add(self, rhs: Money) -> Money { Money { cents: self.cents + rhs.cents } } }`. For `Display`, print a `-` when negative, then `self.cents.abs() / 100` and `self.cents.abs() % 100` with `{:02}`. `Sum`: `fn sum<I: Iterator<Item = Money>>(iter: I) -> Money { iter.fold(Money { cents: 0 }, |a, b| a + b) }`.
@@solution
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Money {
    cents: i64,
}

impl Add for Money {
    type Output = Money;
    fn add(self, rhs: Money) -> Money {
        Money { cents: self.cents + rhs.cents }
    }
}

impl Sub for Money {
    type Output = Money;
    fn sub(self, rhs: Money) -> Money {
        Money { cents: self.cents - rhs.cents }
    }
}

impl Mul<i64> for Money {
    type Output = Money;
    fn mul(self, qty: i64) -> Money {
        Money { cents: self.cents * qty }
    }
}

impl Neg for Money {
    type Output = Money;
    fn neg(self) -> Money {
        Money { cents: -self.cents }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.cents < 0 { "-" } else { "" };
        let c = self.cents.abs();
        write!(f, "{sign}${}.{:02}", c / 100, c % 100)
    }
}

impl From<i64> for Money {
    fn from(dollars: i64) -> Money {
        Money { cents: dollars * 100 }
    }
}

impl Sum for Money {
    fn sum<I: Iterator<Item = Money>>(iter: I) -> Money {
        iter.fold(Money { cents: 0 }, |a, b| a + b)
    }
}

fn main() {
    let coffee = Money { cents: 350 };
    let cake = Money::from(4);
    let order = coffee * 2 + cake;
    println!("{}", order);
    println!("{}", -coffee);
    let total: Money = vec![coffee, cake, Money { cents: 5 }].into_iter().sum();
    println!("{}", total - Money::from(10));
}

@@exercise Parse and convert
A `Rgb` colour should be creatable in several ways:

- `FromStr` for hex strings like `"#ff8800"` (exactly `#` plus 6 hex digits; anything else is an `Err(String)` with any message). Use `u8::from_str_radix(&s[1..3], 16)`.
- `From<(u8, u8, u8)>` from a tuple.
- `TryFrom<&[i32]>` from a slice of exactly three numbers, each 0–255; otherwise `Err(String)`.
- `Display` printing `#rrggbb` in lowercase.
@@starter
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb(u8, u8, u8);

// your implementations here

fn main() {
    let a: Rgb = "#FF8800".parse().unwrap();
    let b = Rgb::from((18, 52, 86));
    let c = Rgb::try_from(&[0, 128, 255][..]).unwrap();
    println!("{a} {b} {c}");
    println!("{}", Rgb::try_from(&[1, 2][..]).is_err());
}
@@expected
#ff8800 #123456 #0080ff
true
@@tests
#[test]
fn checks() {
    assert_eq!("#000000".parse::<Rgb>(), Ok(Rgb(0, 0, 0)));
    assert!("ff8800".parse::<Rgb>().is_err());
    assert!("#ff88".parse::<Rgb>().is_err());
    assert!("#gg0000".parse::<Rgb>().is_err());
    assert_eq!(Rgb::from((1, 2, 3)), Rgb(1, 2, 3));
    assert!(Rgb::try_from(&[0, 0, 256][..]).is_err());
    assert!(Rgb::try_from(&[-1, 0, 0][..]).is_err());
    assert_eq!(Rgb::try_from(&[255, 255, 255][..]), Ok(Rgb(255, 255, 255)));
    assert_eq!(Rgb(171, 205, 239).to_string(), "#abcdef");
}
@@requires
impl FromStr for Rgb
impl TryFrom<&[i32]> for Rgb
@@hint
`FromStr` needs `type Err = String;`. Check `s.len() == 7 && s.starts_with('#')`, then parse three pairs and map their errors with `.map_err(|e| e.to_string())?`. For `TryFrom<&[i32]>`, match `[r, g, b]` on the slice and use `u8::try_from(*r)`.
@@solution
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb(u8, u8, u8);

impl FromStr for Rgb {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 || !s.starts_with('#') {
            return Err(format!("not a #rrggbb colour: {s}"));
        }
        let part = |i: usize| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string());
        Ok(Rgb(part(1)?, part(3)?, part(5)?))
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Rgb(r, g, b)
    }
}

impl TryFrom<&[i32]> for Rgb {
    type Error = String;
    fn try_from(v: &[i32]) -> Result<Self, Self::Error> {
        match v {
            [r, g, b] => {
                let c = |x: i32| u8::try_from(x).map_err(|_| format!("{x} is out of range"));
                Ok(Rgb(c(*r)?, c(*g)?, c(*b)?))
            }
            _ => Err(format!("expected 3 values, got {}", v.len())),
        }
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
}

fn main() {
    let a: Rgb = "#FF8800".parse().unwrap();
    let b = Rgb::from((18, 52, 86));
    let c = Rgb::try_from(&[0, 128, 255][..]).unwrap();
    println!("{a} {b} {c}");
    println!("{}", Rgb::try_from(&[1, 2][..]).is_err());
}
