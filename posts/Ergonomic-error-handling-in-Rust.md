There are many ways you can handle errors in Rust programs: panic at every
fallible situation, propagate foreign errors up the call stack, craft your
own error type.

I've recently fallen into a comfortable pattern when dealing with errors in Rust
and believe it serves great for small or large projects.

This post will walk through a few different ways to handle errors and then
ultimately introduce the [`snafu`](https://github.com/shepmaster/snafu){target="_blank"} library as a clean and ergonomic solution to the elusive task of dealing with erroneous behaviour.

### Contents

<!--ts-->
- [Panic! at the disco](#panic-at-the-disco)
- [Recoverable errors with Result&lt;T, E&gt;](#recoverable-errors-with-resultt-e)
- [Crafting custom error types](#crafting-custom-error-types)
- [Enter snafu](#enter-snafu)
<!--te-->

### Panic! at the disco

Let's say we have some arbitrary text file on disk that we want to read from,
and depending on the content of the file, we want to write back some bytes to
it. Simple enough?

```rust
use std::{
  fs::{self, OpenOptions},
  io::prelude::*
};

const PATH: &str = "file.txt";

fn run() {
  let content = fs::read_to_string(PATH).unwrap();

  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(PATH)
    .unwrap();

  if content.trim() == "hello" {
    file.write_all(b"world!").unwrap();
  }

  Ok(())
}

fn main() {
  run();
}
```

This program can fail in a few different ways:

- The file doesn't exist.
- The contents of the file aren't valid UTF-8.
- The call to [`write_all`](https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all){target="_blank"}
  gets interrupted.

We are handling each of these instances with our calls to [`unwrap`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap){target="_blank"} on the fallible function calls. However, instead of being able to recover from an error if it occurs, the current thread [`panics`](https://doc.rust-lang.org/std/macro.panic.html){target="_blank"}, which, if it is the main thread, terminates all threads and ends the program with code `101`.

### Recoverable errors with Result<T, E>

Instead of calls to `panic!` each time the program can fail, we should be
writing more robust code by giving our caller a chance to recover from this
behaviour.

```rust
use std::{
  fs::File,
  io::{self, prelude::*},
};

const PATH: &str = "file.txt";

fn run() -> Result<(), io::Error> {
  let content = fs::read_to_string(PATH)?;

  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(PATH)?;

  if content.trim() == "hello" {
    file.write_all(b"world!")?;
  }

  Ok(())
}

fn main() {
  match run() {
    Ok(()) => {},
    Err(e) => eprintln!("{}", e)
  }
}
```

The `Result` type is an `enum` which has an `Ok` and an `Err` variant. The `Ok`
variant holds a generic value `T` that gets returned in the case that no errors
have occurred. The `Err` variant holds a generic value `E`, usually an error
type, if an error has occurred.

The `?` operator only works when the function we're applying it to
returns a `Result` in which whose `Err` variant contains the error type we wish to
propagate. It is essentially a shorthand for the following code:

```rust
match result {
  Ok(v)  => v,
  Err(e) => Err(e)
}
```

This implementation allows for our caller to recover from any erroneous
behaviour. However, we can only return one type of error from our
function. This prompts anyone who wishes to possibly return more than one error
type to create their own type that contains multiple variants.

### Crafting custom error types

We can craft our own custom error type as a Rust [`enum`](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html){target="_blank"} and add multiple variants to it. We can then define a `Result` type which we can use throughout our program which contains our custom error type as the error to propagate.

In order to demonstrate the need for multiple error types, we'll extend the
functionality of this tiny application to include a call to some API endpoint
that is written in the text file, using the [`reqwest`](https://github.com/seanmonstar/reqwest){target="_blank"} library.

```rust
use std::{
  fmt::{self, Display},
  fs::{self, OpenOptions},
  io::{self, prelude::*},
};
use reqwest::blocking;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
enum Error {
  IoError(io::Error),
  ApiError(reqwest::Error)
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Error {
    Error::IoError(err)
  }
}

impl From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Error {
    Error::ApiError(err)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::IoError(e) => write!(f, "IO Error: {}", e),
      Error::ApiError(e) => write!(f, "Api Error: {}", e)
    }
  }
}

const PATH: &str = "file.txt";

fn run() -> Result<()> {
  let content = fs::read_to_string(PATH)?;

  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(PATH)?;

  let response = blocking::get(content.trim())?;

  file.write_all(
    response
      .text()?
      .as_bytes()
  )?;

  Ok(())
}

fn main() {
  match run() {
    Ok(()) => {}
    Err(e) => eprintln!("{}", e),
  }
}
```

In order to apply our nice shorthand `?` operator, we have the implement the [`From`](https://doc.rust-lang.org/std/convert/trait.From.html){target="_blank"} trait for each error type we wish to add to our custom `enum`. In addition, in order for `main` to be able to print out the error message, we must implement [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html){target="_blank"} for our custom error type, matching on each variant.

This implementation solves the problem of dealing with multiple error types in a
given function. Having our own custom error type also allows for other users of
our application to deal with our various custom error type variants using a
similar approach. However, as we shall see, we can avoid some of this tedium by
bringing in a third party.

### Enter snafu

The `snafu` library allows for the easy assignment of foreign errors into domain-specific errors while also adding contextual information. Whenever a foreign error type is encountered, we can easily pin it to our own custom typing by adding a `source` attribute.

For instance:

```rust
use std::{
  fs::{self, OpenOptions},
  io::{self, prelude::*},
};
use reqwest::{self, blocking};
use snafu::Snafu;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
enum Error {
  #[snafu(context(false), display("IO Error: {}", source))]
  Io { source: io::Error },

  #[snafu(context(false), display("Api Error: {}", source))]
  Api { source: reqwest::Error },
}

const PATH: &str = "file.txt";

fn run() -> Result<()> {
  let content = fs::read_to_string(PATH)?;

  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(PATH)?;

  let response = blocking::get(content.trim())?;

  file.write_all(
    response
      .text()?
      .as_bytes()
  )?;

  Ok(())
}

fn main() {
  match run() {
    Ok(()) => {}
    Err(e) => eprintln!("{}", e),
  }
}
```

The [`Snafu`](https://docs.rs/snafu/0.6.10/snafu/guide/the_macro/index.html){target="_blank"} macro implements all the necessary traits in order to begin using our custom error type in functions.

Crafting our type this way not only let's us avoid implementing traits ourselves
but also gives us the added benefit of easily adding in custom fields to our
error types as contextual information.

This tiny example doesn't show all of the functionality the `snafu` library has to offer, so I suggest you have a look at the official [documentation](https://docs.rs/snafu/0.6.10/snafu/index.html){target="_blank"}.

### Fin

Quality Rust code should never panic when it doesn't absolutely need to, and
knowing this you should make the best of the situation by crafting your own
error types that other people can use in their applications. You can either make
it tedious, by doing it by hand, or by using the nifty `snafu` library to do
the job.
