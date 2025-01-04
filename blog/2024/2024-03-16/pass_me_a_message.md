Some time ago I wrote an [article on object orientated programming](/blog/2022-02-06/i-hate-oo) which I am now not particularly proud of. I think that article was poorly researched, and didn't work adequately to define what "object orientation" is. In this article I want to deeply explore what an "object" is, and what it means to program in an "object orientated" style, rather than functional or imperative.

First, why do we need to go through the effort of defining "object orientation"? Unfortunately, it's a term that's gone through quite a lot of semantic broadening. Semantic broadening is the process by which a term accumulates new meanings as times goes on. The issue with this is it makes debates on these subjects rather challenging - I might say "object orientation" and mean one thing, while you say "object orientation" and mean another.

Before we can begin to understand "object orientation", let's start with an "object". While objects are now typically thought of as being instantiations of classes, but that isn't really the case.

In fact, if we turn to the old classic *Structure and Interpretation of Computer Programs* we find a very different understanding of an object. Let's look at the development of the idea of an object in three different ways in Rust.

We want to make a bank account protected by a password. This account will have a balance, be able to make deposits and withdrawals, not be allowed to make a withdrawal if the balance would fall below 0, and have a password. Let's look at the SICP style of making this (in Rust)

```
pub fn make_password_protected_account(
    password: String,
) -> impl FnMut(String, String, f64) -> Result<f64, String> {
    let mut balance = 0.0;

    return move |pass, message, value| {
        if pass != password {
            return Err("Incorrect password".to_string());
        }

        if message == "deposit" {
            balance += value;
            return Ok(balance);
        } else if message == "withdraw" {
            if balance - value < 0.0 {
                return Err("Insufficient funds".to_string());
            } else {
                balance -= value;
                return Ok(balance);
            }
        } else {
            return Err("Unknown message".to_string());
        }
    };
}
```

The book describes this as the "message passing" style - since we need to pass around a literal message as a string. To me, this is clearly not a very suitable way to do something like this. We have this pretty unusual `FnMut` type to deal with, it's quite messy, and we can't get any help from the compiler in actually figuring out what messages are available. Notice though how this "object" is in fact a closure - a function. 

You could make an entire codebase in this style (using lots and lots of functions) but I would not describe it as functional programming. Functional programming is about the elimination of state. This message passing closure style is just one way of managing state - but there is state.

Obviously, the drawbacks of this style are immediately obvious. Maybe we can do a little better. Let's try using the Rust `struct` to clean up the state just a little.

```
struct PasswordProtectedAccount {
    balance: f64,
    password: String,
}

impl PasswordProtectedAccount {

    pub fn new(password: String) -> Self {
        return Self { balance: 0.0, password: password };
    }

    pub fn pass_message(
        &mut self,
        password: String,
        message: String,
        value: f64,
    ) -> Result<f64, String> {
        if self.password != password {
            return Err("Incorrect password".to_string());
        }

        if message == "deposit" {
            self.balance += value;
            return Ok(self.balance);
        } else if message == "withdraw" {
            if self.balance - value < 0.0 {
                return Err("Insufficient funds".to_string());
            } else {
                self.balance -= value;
                return Ok(self.balance);
            }
        } else {
            return Err("Unknown message".to_string());
        }
    }
}
```

While this maintains the message passing style, there's a couple of advantages here. The main one is when we make an instance of this `struct` we are no longer going to get an opaque type of `FnMut` but instead a concrete type `PasswordProtectedAccount`. That makes things much, much easier to read, in my mind. But, could we do even better?

```
struct BetterPasswordProtectedAccount {
    balance: f64,
    password: String,
}

impl BetterPasswordProtectedAccount {
    pub fn new(password: String) -> Self {
        return BetterPasswordProtectedAccount {
            balance: 0.0,
            password: password,
        };
    }

    pub fn deposit(&mut self, password: String, value: f64) -> Result<f64, String> {
        if self.password != password {
            return Err("Incorrect password".to_string());
        }

        self.balance += value;
        return Ok(self.balance);
    }

    pub fn withdraw(&mut self, password: String, value: f64) -> Result<f64, String> {
        if self.password != password {
            return Err("Incorrect password".to_string());
        }

        if self.balance - value < 0.0 {
            return Err("Insufficient funds".to_string());
        } else {
            self.balance -= value;
            return Ok(self.balance);
        }
    }
}
```

Now, we get something even better. While we still get the concrete type benefit, we now also get the assistance of concrete methods, too. While the Rust `struct` isn't *exactly* a class, it's easy to see how this could be reproduced in something like Java, C# or C++. 

So it seems that the first example was what we started with in LISP back in the early 1960s, and some aspects of "object orientation" evolved independently of classes, and to some extent classes evolved independently of "object orientation". 

Which brings me to one point about "object orientation" - it isn't just using a class. Often when I program I identify units of code which make sense as `structs` storing state for a long time, maybe even mutable state. But I wouldn't describe this as "object orientation", ultimately because the code isn't orientated around objects. 

But if using objects (in any form) is not enough to make something object orientated, then what is? In *The Object-Orientated Thought Process* Matt Weisfeld defines the following four principles as object orientation 

- Encapsulation
- Inheritance
- Polymorphism
- Composition

But there remains a problem here: none of these are aspects of object orientation.

Inheritance is certainly popular with object orientated code, but for at least twenty years or so the object orientated gurus have admitted this leads to confusion and is generally discouraged. They admit this as much in *The Object-Orientated Thought Process*. So, inheritance might be an optional feature that comes along with object orientation, but it isn't itself object orientation.

Polymorphism isn't a uniquely object orientated principle. Polymorphism is just the idea of different behaviour with different types, and you can do this right now in Rust without any objects to speak of. I really suggest this [blog post](https://oswalt.dev/2021/06/polymorphism-in-rust/) by Matt Oswalt to cover the basics of object orientation in Rust.

Composition is also not a particularly object orientated in practice. Rust `structs` can have other `structs` as fields and `enums` can contain other complex data type (one of my favourite Rust features!). For example you could have something like the following to replace message strings in our bank account example 

```
enum Message {
	Deposit(String, f64),
	Withdraw(String, f64)
}
```

`enums` can even contain other `enums` or `structs`, which themselves can contain `enums` and `structs` and so on.

This just leaves us with encapsulation. This is the idea of implementation details of some object are hidden behind a public interface. We can control this in Rust using the `pub` keyword in `impl` blocks on `structs`. But, at the same time, you can also use the `pub` keyword to control which functions are visible from a module. In this way, modules contain encapsulation, and so do functions. At the limit, we can even define functions inside of functions. So, again, encapsulation doesn't really have anything to do with objects.

Okay, so what *is* the definition of object orientation? The [Rust docs themselves deal with this topic](https://doc.rust-lang.org/book/ch17-01-what-is-oo.html#objects-contain-data-and-behavior)

>The book Design Patterns: Elements of Reusable Object-Oriented Software by Erich Gamma, Richard Helm, Ralph Johnson, and John Vlissides (Addison-Wesley Professional, 1994), colloquially referred to as The Gang of Four book, is a catalog of object-oriented design patterns. It defines OOP this way:
>
>>Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data. The procedures are typically called methods or operations.
>
>Using this definition, Rust is object-oriented: structs and enums have data, and `impl` blocks provide methods on structs and enums. Even though structs and enums with methods aren’t called objects, they provide the same functionality, according to the Gang of Four’s definition of objects.

But, by this definition, having any amount of objects in the code base makes it object orientated, which again doesn't feel right. Maybe in Smalltalk we could use this definition since the language only has objects and messages between them.

This takes me back to something which I talked about in my previous post on the topic. If you need to make *everything* an object, then there are plenty of times when this feels hamfisted. Why would I wrap up pure functions which require no state into an object? 

Dr. Alan Kay was the inventor of the term, let's look at how [he defines it](https://www.purl.org/stefan_ram/pub/doc_kay_oop_en)

>- I thought of objects being like biological cells and/or individual computers on a network, only able to communicate with messages (so messaging came at the very beginning -- it took a while to see how to do messaging in a programming language efficiently enough to be useful).
>- I wanted to get rid of data. The B5000 almost did this via its almost unbelievable HW architecture. I realized that the cell/whole-computer metaphor would get rid of data, and that "<-" would be just another message token (it took me quite a while to think this out because I really thought of all these symbols as names for functions and procedures.
>- My math background made me realize that each object could have several algebras associated with it, and there could be families of these, and that these would be very very useful. The term "polymorphism" was imposed much later (I think by Peter Wegner) and it isn't quite valid, since it really comes from the nomenclature of functions, and I wanted quite a bit more than functions. I made up a term "genericity" for dealing with generic behaviors in a quasi-algebraic form.
>- I didn't like the way Simula I or Simula 67 did inheritance (though I thought Nygaard and Dahl were just tremendous thinkers and designers). So I decided to leave out inheritance as a built-in feature until I understood it better.

It's still somewhat unclear what Dr Alan is actually trying to get at here. This idea of communicating by messages just seems messy, unless he has some other idea other than string matching. I think Smalltalk has some idea like this which is alleged to be different to just calling methods of classes, but in what substantial way it is escapes me.

This really leaves us in a tough spot when it comes to object orientation. What even is it? In a [different article of mine](/blog/2024-01-07/object-orientation-is-religion), I make the case that object orientation is essentially akin to a religion. Thankfully, Casey Muratori of Molly Rocket agrees with me [in this video](https://www.youtube.com/watch?v=7YpFGkG-u1w). 

So, what's the conclusion here? Am I ever going to use the string message passing style of coding in production? No. Never. It's cute and useful, and I do suggest doing the problems from SICP. Will I ever make code "object orientated"? Not if I can avoid it. 

