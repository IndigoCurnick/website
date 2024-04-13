# SICP Chapter 1 Part 1

I'm currently attempting to do every problem in _Structure and Interpretation of Computer Programs_. This post will give solutions to the wordy problems in the firt chapter. You can find the repo where I'm solving the problems [https://github.com/IndigoCurnick/structure_interpretation_computer_programs](https://github.com/IndigoCurnick/structure_interpretation_computer_programs)

## Exercise 1.5

We are given the following JavaScript code

```
function p() { return p(); }

function test(x, y) {
	return x === 0 ? 0 : y;
}
```

Which we then evaluate with 

```
test(0, p());
```

We are asked what behaviour we get with a applicative-order evaluation vs a normal-order evaluation.

Applicative order is "eager" - this means that all parameters are evaluated before they passed to functions. If we take a look at the following LISP code and consider what happens in applicative order

```
(define (square x) (* x x))
(square (+ 2 3))
```

First, `(+ 2 3)` is evaluated to 5, then passed to the function `square`.

On the other hand, normal order is "lazy" in that arguments are not evaluated until they are needed. If we consider the LISP code again, the expression `(+ 2 3)` is passed to the function `square` and then the whole expression `(* (+ 2 3) (+ 2 3))` is evaluated. 

If we consider the JavaScript code again, then we can see that `p()`  is a recursive function in the most literal sense - it just returns itself. Therefore, if we are in applicative order when we run `test(0, p())` then `p()` is evaluated, giving `p()`, which is evaluated giving `p()` which is...

Therefore, in applicative order, we get an infinite loop.

In normal order, `p()` is not immediately evaluated, so in that case `test` will just return 0.

## Exercise 1.6

In exercise 1.6, we are again asked a question about normal-order and applicative-order. We write a new kind of function which does conditional evaluation 

```
function conditional(predicate, then_clause, else_clause) {
	return predicate ? then_clause : else_clause;
}
```

Which can be used as so 

```
conditional(2 === 3, 0, 5); // 5
conditional(1 === 1, 0, 5); // 0
```

However, consider the following function 

```
function sqrt_iter(guess, x) {
	return conditional(is_good_enough(guess, x),
						guess,
						sqrt_iter(improve(guess, x)), x)
					);
}
```

What happens when we evaluate this? If the interpreter is applicative-order, then the `sqrt_iter` function is evaluated before the conditional can be evaluated, leading to an infinite regression.

## Exercise 1.9

We are presented with two versions of the `plus` function

```
function plus(a, b) {
	return a === 0 ? b : inc(plus(dec(a), b));
}
```

and 

```
function plus(a, b) {
	return a === 0 ? b : plus(dec(a), inc(b));
}
```

where

```
function inc(a) {
	return a + 1;
}

function dec(a) {
	return a - 1;
}
```

Let's write out how these will evaluate 

```
plus(4, 5);
inc(plus(3, 5));
inc(inc(plus(2, 5)));
inc(inc(inc(plus(1, 5))));
inc(inc(inc(inc(plus(0, 5)))));
inc(inc(inc(inc(5))));
inc(inc(inc(6)));
inc(inc(7));
inc(8);
9;
```

So the first version is recursive

```
plus(4, 5);
plus(3, 6);
plus(2, 7);
plus(1, 8);
plus(0, 9);
9;
```

So the second version is iterative.

## Exercise 1.13

<p>In this exercise, we are prove that \(Fib(n)\) is the closest integer to \(\phi^n / \sqrt{5}\), where \(\phi = (1 + \sqrt{5}) / 2\)</p>

First, let's define the following constant (to make life easier)

<p>\[\psi = \frac{1 - \sqrt{5}}{2} \]</p>

We want to prove that 

<p>\[Fib(n) = \frac{\phi^n - \psi^n}{\sqrt{5}}\]</p>

<p>We can see for \(n=0,1\) we have the trivial cases</p>

<p>\[ Fib(0) = \frac{\phi^0 - \psi^0}{\sqrt{5}} = 0  \]</p>

<p>\[ Fib(1) = \frac{\phi^1 - \psi^1}{\sqrt{5}} = 1\]</p>

<p>Now consider some \(k \in N, k \geq 2\). Then for \(n = k + 2\)</p>

<p>\[ Fib(k+2) = Fib(k) + Fib(k+1) \]</p>

<p>\[ Fib(k+2) = \frac{\phi^k - \psi^k}{\sqrt{5}} + \frac{\phi^{k+1} - \psi^{k+1}}{\sqrt{5}} \]</p>

<p>\[Fib(k+2) = \frac{\phi^k (\phi+1) - \psi^k (\psi + 1)}{\sqrt{5}}\]</p>

<p>Note that \(\phi\) and \(\psi\) satisfy the following equations</p>

<p>\[ \phi^2 = \phi + 1 \]</p>

<p>\[ \psi^2 = \psi + 1 \]</p>

Therefore, we can write

<p>\[Fib(k+2) = \frac{\phi^k \phi^2 - \psi^k \psi^2}{\sqrt{5}}\]</p>

<p>\[Fib(n) = \frac{\phi^{k + 2} - \psi^{k + 2}}{\sqrt{5}}\]</p>

<p>Therefore, \(Fib(n) = \frac{\phi^n - \psi^n}{\sqrt{5}}\) is true for all \(k \in N\)</p>

Now, we want to prove 

<p>\[ \left|\frac{\psi^n}{\sqrt{5}}\right| < \frac{1}{2} \]</p>

<p>At least for all \(n \in N\)</p>

Note that 

<p>\[ \frac{ \sqrt{5} - 1 }{2} < \frac{1}{2} \]</p>

<p>\[ | \psi^{n+1} | = | \psi^n | \frac{ \sqrt{5} - 1 }{2} \]</p> 

Thus, 

<p>\[ | \psi^{n+1} | < | \psi^n | \]</p>

And since 

<p>\[ \frac{|\psi^0|}{ \sqrt{5} } < \frac{1}{2} \]</p>

<p>We can conclude that \(\left|\frac{\psi^n}{\sqrt{5}}\right| < \frac{1}{2}\) is true</p>

Therefore,

<p>\[ \left| \frac{\psi^n}{\sqrt{5} } - Fib(n) \right| = \frac{\psi^n}{ \sqrt{5} < \frac{1}{2} } \]</p>

<p>Therefore, \(Fib(n)\) is the closet integer to \(\frac{\phi^n}{\sqrt{5}}\)</p>

## Exercise 1.20

In this exercise we are asked to look at the function 

```
function gcd(a, b) {
	return b === 0 ? a : gcd(b, a % b);
}
```

We're asked to consider how many times the `%` operator is used in normal order vs applicative order when evaluating `gcd(206, 40)`.

Consider normal order first. The process looks like

```

gcd(206, 40);

gcd(40, 206 % 40);

gcd(40, 6);

gcd(6, 40 % 6);

gcd(6, 4);

gcd(4, 6 % 4);

gcd(4, 2);

gcd(2, 4 % 2);

gcd(2, 0);

2
```

So that `%` is evaluated 4 times.

Now consider applicative order, the process looks like

```
gcd(206, 40);

gcd(40, 206 % 40);

gcd(206 % 40, 40 % (206 % 40));

gcd(40 % (206 % 40), (206 % 40) % (40 % (206 % 40)));

return (206 % 40) % (40 % (206 % 40)) === 0 ? 40 % (206 % 40) : gcd((206 % 40) % (40 % (206 % 40)), (40 % (206 % 40)) % ((206 % 40) % (40 % (206 % 40))))

(206 % 40) % (40 % (206 % 40)) === 0 ? 40 % (206 % 40) // the first expression evaluates to 0 so only need to consider this part

2
```

So `%` is evaluted 6 times

## Exercise 1.25

In earlier questions we wrote a function called `expmod`, here's my Rust implementation

```
pub fn square(x: i64) -> i64 {
    return x * x;
}

pub fn expmod(base: i64, exp: i64, m: i64) -> i64 {
    if exp == 0 {
        return 1;
    }

    if even(exp) {
        square(expmod(base, exp / 2, m)).rem(m)
    } else {
        (base * expmod(base, exp - 1, m)).rem(m)
    }
}
```

This question asks why not just write the following function

```
function expmod(base, exp, m) {
	return fast_expt(base, exp) % m;
}
```

And you could, and it would work, but in this implementation, for very large inputs, `fast_expt` would produce a very large output, which might cause performance issues or floating point errors!

## Exercise 1.26 

In this question someone has written the `expmod` function with a multiplication rather than calling `square`

```
function expmod(base, exp, m) {
	return exp === 0 ? 1 : is_even(exp)
			? expmod(base, exp / 2, m) * expmod(base, exp / 2, m) % m
			: base * expmod(base, exp - 1, m) % m;
}
```

and asked why this performs so much worse? The issue is that `expmod(base, exp / 2, m)` is called twice, causing a massive performance impact since the exact same value is calculated twice before being multiplied together. I think some compilers, like the Rust compiler, could optimise that mistake away but generally interpreted languages won't be able to!

