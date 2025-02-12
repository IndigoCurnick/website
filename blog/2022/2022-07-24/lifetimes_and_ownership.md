Lifetimes and ownership in Rust are some of the aspects of the language users coming from other languages struggle to
understand. Few languages have concepts similar to these, so they can be unfamiliar. However, they are by far the
most powerful aspects of the language that give Rust its appeal. Through lifetimes and ownership, we can write code
that we know will be memory safe, without the use of a garbage collector. Read on to find out how that is possible.

Consider the following piece of code

```
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("The value of r is {}", r);
}
```

This code can not compile. We get the very helpful error message

```
error[E0597]: `x` does not live long enough
         --> src/main.rs:6:13
          |
        6 |         r = &x;
          |             ^^ borrowed value does not live long enough
        7 |     }
          |     - `x` dropped here while still borrowed
        8 | 
        9 |     println!("The value of r is {}", r);
          |                                      - borrow later used here
        
        For more information about this error, try `rustc --explain E0597`.
```

Rust is telling us that the borrowed value x does not live long enough. What does this mean? In Rust, scope is very
important. The curly brace sub-scope in the `main()` function is not just for show - everything defined inside that
scope will no longer exist once the scope has ended. This applies to all scopes. When Rust is compiled, it checks to
see if any references have been defined inside a scope that will no longer exist. By the time we leave the scope, `x`
will no longer be in memory but `r` still references it. How do we solve this? In this instance, we only need to
remove the sub-scope to keep `x` in scope for as long as we need it

```
fn main() {
    let r;

    let x = 5;
    r = &x;

    println!("The value of r is {}", r);
}

The value of r is 5
```

Now, this is a very simple example. But let's consider something more complicated. Let's consider when we need to help the compiler out in understanding the lifetimes.

```
fn main() {
    let abcd = "abcd".to_string();
    let xyz = "xyz".to_string();

    let result = longest(&abcd, &xyz);
    println!("The longest is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}
```

Here we are trying to figure out which of two strings are longer. We pass references to those strings into the `longest()` function. Compiling this code does not work though, and gives us the following error:

```
error[E0106]: missing lifetime specifier
         --> src/main.rs:9:33
          |
        9 | fn longest(x: &str, y: &str) -> &str {
          |               ----     ----     ^ expected named lifetime parameter
          |
          = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
        help: consider introducing a named lifetime parameter
          |
        9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
          |           ++++     ++          ++          ++
```

Rust is telling us that it does not know if the lifetime is supposed to be coming from `x` or `y`. This makes sense, as the return can vary. In this instance, we need to help the compiler out in understanding what the lifetimes should be. If you look at the bottom of this error message, Rust is also showing us how to fix the error, one of the nice things about the language. It's asking us to add this `'a` into the function signature. This is a lifetime parameter. They always start with an apostrophe, and they help specify the lifetime of arguments. In this instance, we are telling Rust "the lifetime of what comes out of this function is going to be equal to the lifetime of the shortest lives argument".

Let's add this fix, and run the program.

```
fn main() {
    let abcd = "abcd".to_string();
    let xyz = "xyz".to_string();

    let result = longest(&abcd, &xyz);
    println!("The longest is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

The longest is abcd
```

In this instance, both variables live for the same length of time. So what Rust is checking is the lifetime of result. As long as the shortest lifetime which result could refer to is still valid by the time it goes out of scope, then we're fine. Let's make some new scopes to test this out

```
fn main() {
    let abcd = "abcd".to_string();

    {
        let xyz = "xyz".to_string();

        let result = longest(&abcd, &xyz);
        println!("The longest is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}
```

Here, everything is valid. `xyz` and `result` go out of scope at the same time, and abcd lives longer. Here we can guarantee that result has the same lifetime of the shortest lived variable. Let's break this by moving around the scope

```
fn main() {
    let abcd = "abcd".to_string();
    let result;
    {
        let xyz = "xyz".to_string();

        result = longest(&abcd, &xyz);
    }

    println!("The longest is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}
```

This fails to compile, and the reason is easy to see. We can not guarantee that result will be referencing something that is still in scope, so it is not valid. Let's change this up a little

```
fn main() {
    let abcd = "abcd".to_string();

    let xyz = "xyz".to_string();

    let result = longest(&abcd, &xyz);

    println!("The longest is {}", result);
}

fn longest<'a>(x: &str, y: &str) -> &'a str {
    // Some code
    let pqr = "pqr".to_string();
    return &pqr;
}
```

Here I'm trying to return a reference to a string. Notice how I removed the lifetimes from x and y - they aren't needed anymore. I get the following error

```
error[E0515]: cannot return reference to local variable `a`
          --> src/main.rs:14:12
           |
        14 |     return &a;
           |            ^^ returns a reference to data owned by the current function
        
        For more information about this error, try `rustc --explain E0515`.
```

This leads us on nicely to ownership. We can't return a reference to `pqr` since the data is owned by `longest()`. `pqr` will be out of scope when `longest()` exists. We can fix this by returning a `String`, which will transfer ownership of the data out of the function, like this 

```
fn main() {
    let abcd = "abcd".to_string();

    let xyz = "xyz".to_string();

    let result = longest(&abcd, &xyz);

    println!("The longest is {}", result);
}

fn longest(x: &str, y: &str) -> String {
    // Some code
    let pqr = "pqr".to_string();
    return pqr;
}
```

That works for variables leaving functions, but what about for variables going into them? Consider the following program

```
fn main() {
    let abcd = "abcd".to_string();

    foo(abcd);

    println!("{}", abcd);
}

fn foo(a: String) {
    println!("{}", a);
}
```

This is not a valid program in Rust since when we call `foo()` in `main()`, `abcd` is transferred to be owned by `foo()` rather than `main()`. When `foo()` ends, abcd is now out of scope and can no longer be used. There are two ways around this.

```
fn main() {
    let abcd = "abcd".to_string();

    foo(abcd.clone());

    bar(&abcd);

    println!("{}", abcd);
}

fn foo(a: String) {
    println!("{}", a);
}

fn bar(a: &str) {
    println!("{}", a)
}
```

I've shown both ways around this problem here. `foo()` remains the same, but rather than transfer ownership, I totally copy the variable. This makes a whole new variable in memory that is entirely separate to the original variable. I also make a function `bar()` which takes a reference to a string. Both of these are valid, and which one you use depends on the situation. Just note that `clone()` is a much more expensive operation than a simple reference, so try and avoid it where you can.

All of these things seem to make Rust challenging to understand, but if you approach them slowly, they actually make a lot of sense. These are the features that make Rust a memory safe language - so it's well worth learning how they work! 