Handling Errors in Rust
=======================

Many people who come to Rust for the first time struggle with error handling. There is no concept of the try-catch block as there is in many other languages like Java or Python. How do you handle errors? This is done via four concepts – `Result`, `Option`, `Unwrap` and `Match`.

Result
------

Result is the core concept of error handling. Since Rust is a functional language, we obviously expect to use many functions. If each of those functions returns a result we can radically reduce the chances of our program crashing. How? Let's use an example

    
            pub fn divide(x: f64, y: f64) -> Result<f64, String> {
                if y == 0.0_f64 {
                    return Err("Can not divide by zero".to_string());
                } else {
                    return Ok(x / y);
                }
            }

Phew. There's A LOT going on here – let's try and break it down step by step.

We have a public function called divide that takes two `float64` arguments. This function will divide `x` by `y`. Simple enough, but there is a danger zone when `y` is 0 – since we can not divide by zero, our program will crash. How can we avoid this? In something like Python or Java, we would probably use a Try-Catch block. Here, we can use a `Result`.

We see this function returns a `Result<f64, String>`. `Result<_,_>` is simply a type like any other. Specifically it is an `Enum` type that contains two values – an `Ok` type and an `Error` type. In my example the `Ok` type is `float64` and the `Error` type is `String`. You could put any types in there.

This function has two branches, and we obviously must return from both of them. Notice how we "wrap" what we return in either `Ok(_)` or `Err(_)`. This is how we tell Rust which is an Ok value and which is an Error value. Whatever is in `Ok(_)` must match the first type nestled inside `Result<_,_>` and whatever in `Err(_)` the second.

As a note here too, the reason why I must use the `.to_string()` function is because Rust has two different kinds of strings. It has `String` which is more like how strings work in most other languages – this is basically a char array under the hood. The other type is `&str` – this is a reference to a string in memory – a "string literal". Since a string I type in between quotes is always a string literal, I do not have the string itself but only a reference to it. I won't get specifically into the reasons why now, but in order to pass this string literal reference out of this function we would need to introduce one of the most unique (and challenging) concepts in Rust – the lifetime.

Option
------

`Option` is similar to `Result`, but it either contains a value or contains `None`. Since we've already covered a similar concept, let's just dive right in with an example

    
            pub fn contains(text: &str, target_char: char) -> Option<String> {
                if text.contains(target_char) {
                    return Some(text.to_string());
                } else {
                    return None;
                }
            }

This function checks to see if the `target_char` is inside text, if it is, it returns the text converted to a `String`. Again, we see this "wrapping" principle at play – the type we want to return is now wrapped inside `Some(_)`. In the case where there is nothing, we just return `None` – with no wrapping.

Unwrap
------

Ok, we have discussed `Result` and `Option` which "wrap up" what's inside them, so we can safely pass around errors and null values. How do we actually get to the thing inside of it? We use the function unwrap. With a `Result` or `Option` type, we can unwrap to get the value inside. So, for example

    
            let foo: Option<f64> = Some(1_f64);
            let bar: f64 = foo.unwrap(); // bar is equal to 1_f64 here

or for another example\*

    
            let foo: Result<f64, String> = Ok(1_f64);
            let bar: f64 = foo.unwrap(); // bar is equal to 1_f64 here

However, we want to stay away from using `.unwrap()` unless we are _really_ sure that the result is not an error or the `Option` is not `None`. Why? Because if we unwrap a result to find an `Error`, our program will "panic" – that is, crash. How then can we actually deal with `Result` and `Option`? We introduce the next concept - `Match`

Match
-----

`Match` is a kind of statement in Rust, similar to an `If` statement but with one important difference – a `Match` statement must cover all possible values of the type. Let's use two examples first, then talk a little more theoretically.

Let's consider the functions we wrote earlier.

    
            let x = 10_f64;
            let y = 0_f64;
            let div = divide(x, y);
            let num = match div {
                Ok(x) => x,
                Err(y) => panic!("{}", y)
            };
            let text = "Hello";
            let target = 'e';
            let text_res = contains(text, target);
            match text_res {
                Some(x) => println!("{}", x),
                None => println!("Target char not found"),
            };

In the first example we use the divide function we wrote. We can see that since y is 0, the function will return an `Error` – so when we `Match` the result the second arm will fire. In this example we assign from the `Match` statement. There are two variants for `Result<_,_>` - `Ok(value)` and `Err(y)`. Notice how I have x and y wrapped inside `Ok(_)`Ok(\_) and `Err(_)` - these are NOT the same x and y defined further up in the function as they have a different scope, these are just temporary variables.

In the second example, we do not assign from the `Match` statement. Instead, we simply execute code. `Option<_>` has two variants, `Some(value)` and `None`, which make the two branches of our `Match` statement. In this case, "Hello" does contain 'e' so the `Some(_)` arm will execute and print "Hello" to the console.

`Match` statements have the special property that ALL possible values for the type being matched are covered. For `Result` and `Option`, this is easy since they only have two values. We can `Match` on any type though. Here's an example of `Match` on an integer.

    
            let x = 6_i64;
            match x {
                1_i64 => println!("Value is one"),
                2_i64 | 3_i64 | 4_i64 => println!("Value is either two, three or four"),
                5_i64..=10_i64 => println!("Value is between five and ten"),
                _ => println!("Value is something else")
            };

In this example we `Match` an integer. It is very unlikely that I will type out every single possible integer value or range, so instead we use `_` to mean "any other values for this type that is not explicitly already mentioned". I usually find that when I `Match` on an integer I think it should only be able to have a set number of possible values and often `panic` on the `_`. An example might be matching on the length of a `Vector`Vector. If we only ever want the vector length to be 10, 11 or 12 we could `panic` and print an error on all other values ("Incorrect number of elements in vector").

So why do it this way? What's wrong with a good old Try-Catch? The answer is simple – Try-Catch is dysgenic. Got a troublesome program? Just whack the whole thing inside a Try-Catch. I would be surprisingly wealthy if I had a pound for every time I saw this. By using `Match` it forces us to think really carefully about all of the possible places where our program could crash – and nothing, no Try-Catch or error handling, can overcome how important it is to think.

A Few Notes
-----------

Since `Result` and `Option` are just types, we can pass them around and into functions. Now, I have never passed around a `Result`, but `Option` is more or less how Rust does key word arguments. Let's look at an example.

    
            pub fn process(x: f64, y: f64, z: Option<f64>) -> f64 {
                let mut res = x * y;
                match z {
                    Some(p) => res += p,
                    None => {}
                };
                return res;
            }

In this example, the function will multiply together x and y, and if z is something it will add it to the result. Notice how we simply use `{}` to indicate that nothing needs to be done if z is `None` – remember `Match` statement MUST cover all possible values of the type.

While we looked at the `Match` statement which is one of the most powerful concepts in Rust, we are also provided with some functions to check which we could use in an if statement. These are `is_some()`, `is_none()` (which apply to `Option`) and `is_ok()`, `is_err()` (which apply to `Result` type). I leave it as an exercise for the reader to figure out what these do (if you have understood the article it should hopefully be obvious!!!!)

I'll show you one example without commentary

    
            pub fn process(x: f64, y: f64, z: Option<f64>) -> f64 {
                let mut res = x * y;
                if z.is_some() {
                    res += z.unwrap(); // Is it safe to unwrap here? Why?
                }
                return res;
            }

\*While I have used manual type annotations for clarity, in this particular example it is necessary, as Rust can not infer the type of foo. Remember that the type in this case is BOTH parts of `Result<_,_>`. While it can infer `Result<f64,_>`, being unable to infer the error state makes this unable to be compiled.