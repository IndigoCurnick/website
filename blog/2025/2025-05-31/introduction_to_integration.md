**tl;dr** If you just want a quick reference find Appendix A at the end of the article which summarises the main integration rules. Appendix B contains a bunch of different integrals which are more challenging solved for lookup.

Integration can be thought of in two ways, which as we will see are really two ways of stating the same thing. On the one hand, integration is the reverse operation of differentiation. On the other, integration lets us find the area under a graph. 

Let's consider a simple example. Suppose someone runs at a constant speed of $v = 10m/s$ for $t = 10s$. Then

$$d = v \cdot t = 100m$$

Very trivial.

For reasons we shall see in a moment, the integral of $v$ is

$$\int v dx = vx + C = 10x + C$$

If we "evaluate" (subtract the bottom range from the top) this at $t_0 = 0$ and $t_1 = 10$ i.e. the start and end of the period of interest, we find that

$$(10 \cdot 10 + C) - (10 \cdot 0 + c) = 100$$
Which represents the 100m. In other words, this integral found the area under this graph of the runner's speed. In this case, integration was massive overkill since the graph is simply a rectangle.

What does the $10x$ represent? That represents the distance ran by the runner. For example, at $t=0$, $10 \cdot 0 = 0$ i.e. before he begins running he has not moved. At $t=5$, $10 \cdot 5 = 50$ i.e. at 5 seconds in he has run 50m. What if we differentiated?

$$\frac{d}{dx} 10x = 10$$
which gets us back to the speed of $10m/s$. Notice how the differentiation has "undone" the integral.

Integration has many uses in the physical sciences and computer sciences. Therefore, we have to master it if we want to be accomplished physicists!

One thing to know about integration is that unlike differentiation there is not always an analytic solution. This is very frustrating, to say the least. There's many times when we just can't integrate a function. There's a few ways around this, which leads us onto the focus of this blog. We're going to look at solving integrals by hand. Most integrals can be solved by numerical methods on computers, which don't give us closed forms but rather approximations.

Therefore, one important skill to develop with integration is *finding the correct technique which will help*. So far as I am aware, there is no guaranteed "meta-technique" which selects the correct technique (or indeed, tells you if a function can not be integrated analytically). This is intuition. 

## Integrating Polynomials

Integrating a polynomial *is* actually quite simple though - we simply increase the power by 1, and divide by that new power (you might notice, exactly the opposite of differentiation.)

$$\int x^n dx = \frac{x^{n+1}}{n+1} + C$$

For example

$$\int x^2 dx = \frac{x^3}{3} + C$$
And for a sanity check

$$\frac{d}{dx} \left( \frac{x^3}{3} + C \right) = x^2$$

So what is this $+C$ I keep adding? Remember how we said that integration was the opposite of differentiation? Imagine the following differentiation

$$\frac{d}{dx} (2x + 1) = 2$$
Let's say we want to recover the original function by integrating

$$\int 2 dx = 2x + C$$
Notice how we can't *quite* "go back" to the original function. Some information has been lost, that's because e.g.

$$\frac{d}{dx} (2x + 1) = \frac{d}{dx} (2x + 2) = \frac{d}{dx} (2x + 100) = 2$$

All constants will simply fall out when differentiated, so we don't know which one it was. This might seem like integration is useless, but thankfully in its main use case we actually don't have to worry at all about the $+C$. 

That use case is integrating between two limits. So we saw that in the earlier runner example. Just as another example, let's evaluate $2x^2 + x$ from 1 to 2

$$\int^2_1 (2x^2 + x) dx = \left[ \frac{2x^3}{3} + \frac{x^2}{2} + C \right]^2_1 = 
\left(\frac{2 \cdot 2^3}{3} + \frac{2^2}{2} + C \right) - \left(\frac{2 \cdot 1^3}{3} + \frac{1^2}{2} + C \right) = \frac{37}{6} \approx 6.1667$$

A few things to note. We use the $[ f(x)]^a_b$ notation to indicate $f(a) - f(b)$.

Second, notice how sums in an integral can just be broken down.

$$\int (a + b) dx = \int a dx + \int b dx$$

## Integration Laws

Like differentiation, there's a few common laws we have for integration that you should know. If you already know the relevant differentiation laws, you know these by default as they are just in reverse. 

Exponentials are as always the simplest

$$\int e^x dx = e^x + C$$

The trigonometric laws again mirror their differentiation counterparts


$$\int \sin(x) dx = -\cos(x) + C$$
$$\int \cos(x) dx = \sin(x) + C$$
$$\int \sec^2(x) dx = \tan(x) + C$$

And the exponentials

$$\int \frac{1}{x} dx = \ln(n) + C$$
$$\int a^x dx = \frac{a^x}{\ln(a)} + C$$
$$\int \ln(x) dx = x \ln(x) - x + C$$


## Integration by Parts

If we return to the product rule from differentiation

$$(u(x)v(x))^\prime = u^\prime (x) v(x) + u(x) v^\prime (x)$$

We can integrate both sides with respect to $x$

$$\int (u(x)v(x))^\prime dx = \int u^\prime (x) v(x) dx + \int u(x) v^\prime (x) dx$$

And so

$$u(x)v(x) = \int u^\prime (x) v(x) dx + \int u(x) v^\prime (x) dx$$

We we normally rearrange into the more useful form

$$\int u(x) v^\prime (x) dx = u(x)v(x) - \int u^\prime (x) v(x) dx $$

A more compact version which is easier to parse and memorise is 

$$\int u dv = uv - \int v du$$

A few examples will help illuminate the usage 

### Example 1

Find 

$$\int x \cos(x) dx$$
We let

- $u = x$
- $du = 1$
- $dv = \cos(x)$
- $v = -\sin(x)$

So

$$\int x \cos(x) dx = x \sin(x) - \int -\sin(x) \cdot 1 dx$$

Notice how the second integral is now very easy to solve!

$$\int x \cos(x) dx = x \sin(x) + \cos(x) + C$$

So with integration by parts your goal is to try and select $v$ and $u$ such that the new integral you find is much simpler.

### Example 2

Find

$$\int e^x x dx$$

Let 

- $u = x$
- $dv = e^x$
- $du = 1$
- $v = e^x$

$$\int e^x x dx = x e^x - \int e^x \cdot 1 dx = x e^x - e^x = e^x (x - 1)$$


### Example 3

Find 

$$\int e^x \sin(x) dx$$

Let 

- $u = \sin(x)$
- $du = \cos(x)$
- $dv = e^x$
- $v = e^x$

$$\int e^x \sin(x) dx = \sin(x) e^x - \int e^x \cos(x) dx$$

Hmm, well at first glance we seem to have just gone in circles. But there's an important lesson in integration - sometimes you have to apply a technique more than once, or sometimes combinations of techniques. Let's try solving that integral by parts again as a subproblem.

Find

$$\int e^x \cos(x) dx$$

Let

- $u = \cos(x)$
- $du = - \sin(x)$
- $dv = e^x$
- $v = e^x$

$$\int e^x \cos(x) = \cos(x) e^x - \int - e^x \sin(x)$$

Okay, what if we substituted this back into what we had above?

$$\int e^x \sin(x) dx = \sin(x) e^x - \left(\cos(x) e^x - \int - e^x \sin(x) \right)$$

$$\int e^x \sin(x) dx = \sin(x) e^x - \cos(x) e^x - \int e^x \sin(x)$$

$$2 \int e^x \sin(x) dx = \sin(x) e^x - \cos(x) e^x$$

$$\int e^x \sin(x) dx = \frac{\sin(x) e^x - \cos(x) e^x}{2}$$

## Integration by Substitution

In the same way that integration by parts was a different statement of the product rule from differentiation, integration by substitution is simply running the chain rule in reverse.

I say "simply" rather surreptitiously though. In practice, choosing the appropriate substitution is in practice sometimes rather challenging. We will see some examples soon. 

Let's walk through the proof. We recall the chain rule as

$$\frac{d}{dx} F(g(x)) = F^\prime (g(x)) g^\prime (x)$$
So we can inverse this with an integration

$$\int F^\prime (g(x)) g^\prime (x) dx = F(g(x)) + C$$

Integration by substitution has a hint in the name - we will set $u = g(x)$

$$\frac{d}{dx} (F(u)) = F^\prime (u) u^\prime$$
Since $du = g^\prime (x) dx$ then we can finally write

$$\int F^\prime (g(x)) g^\prime (x) dx = \int F^\prime (u) du = F(u) + C = F(g(x)) + C$$

## Example 1

Find

$$\int x \sin(x^2 + 5) dx$$

Here we let $u = x^2 + 5$. Such a substitution is not always the best, or even correct, but often the "inside function" of a trigonometric function is a good place to start!

Then $du = 2x dx$, and so $\frac{1}{2} du = x dx$. What must be done is now clear to see - we can replace the $x dx$ inside the integral with our new expression for $du$

$$\int x \sin(x^2 + 5) dx = \int \frac{1}{2} \sin(u) du = -\frac{1}{2} \cos(u) + C = - \frac{1}{2} \cos(x^2 + 5) + C$$


## Example 2

We'll look at using limits in this example - it is important to replace them, too. 

Find

$$\int_1^3 (9 + x)^2 dx$$

Again, a good candidate for $u$ is the "inside function", in this case $u = 9 + x$, $du = 1 dx$

So the integral is

$$\int^{x=3}_{x=1} u^2 du$$

I wrote the upper and lower limits in terms of $x$ to emphasise the point - the limits are going over $x$ but we are now integrating over $u$. You've probably already worked out how to replace them. When $x=1, u=10$ and when $x=3, u=12$. So we can write

$$\int_{u=10}^{u=12} u^2 du = \left[ \frac{1}{3} u^3 \right]^{12}_{10} = \frac{728}{3}$$

If you prefer, you could also undo the substitution at the end instead of changing the limits

$$\int u^2 du = \frac{1}{3} u^3 + C = \frac{1}{3} (9 + x)^3 + C$$

$$\int_1^3 (9 + x)^2 dx = \left[ \frac{1}{3} (9 + x)^3 \right]^3_1 = \frac{728}{3} $$
## Example 3

Find 

$$\int x \sqrt{x + 3} dx$$

Let $u = x + 3$ and $du = dx$, thus

$$\int x \sqrt{x + 3} dx = \int x \sqrt{u} du$$

It is not possible at the moment to solve this, but we can actually do a section substitution because $x = u -3$

$$\int x \sqrt{u} du = \int (u - 3) u^{\frac{1}{2}} du$$
There's only a few steps now

$$\int (u - 3) u^{\frac{1}{2}} du = \int (u^{\frac{3}{2}} - 3 u^{\frac{1}{2}}) du = \frac{2}{5} u^{\frac{5}{2}} - 2u^{\frac{3}{2}} + C$$

We can also substitute back in our original formula for $u$

$$\int x \sqrt{x + 3} dx = \frac{2}{5} (x + 3)^{\frac{5}{2}} - 2 (x + 3)^{\frac{3}{2}} + C$$
A good habit to get into is differentiating to check you got the right answer. This one is a little trickier, so I'll walk the steps here

$$\frac{d}{dx} \left( \frac{2}{5} (x+3)^{\frac{5}{2}} - 2 (x+3)^{\frac{3}{2}} \right) = (x+3)^{\frac{3}{2}} - 3(x+3)^{\frac{1}{2}}$$

$$(x+3)^{\frac{3}{2}} - 3(x+3)^{\frac{1}{2}} = (x+3) \sqrt{x+3} - 3 \sqrt{x+3}$$
$$(x+3) \sqrt{x+3} - 3 \sqrt{x+3} = \sqrt{x+3} (x + 3 - 3) = x \sqrt{x + 3}$$


## References

Spivak, M. (2008). *Calculus (4th ed.)*. Publish or Perish, Inc.

Apostol, T., M. (1967) *Calculus Volume 1 (2nd ed.)*. John Wiley & Sons, Inc.

Gradshteyn, I., S. & Ryzhik, I., M. (2015) *Table of Integrals, Series and Products (8th ed.)*. Elsevier

## Appendix A: Basic Integration Laws

### Power Law

$$\int x^n dx = \frac{x^{n+1}}{n+1} + C$$

### Integration by parts

$$\int u dv = uv - \int v du$$

### Integration by substitution

$$\int F^\prime (g(x)) g^\prime (x) dx = \int F^\prime (u) du = F(u) + C = F(g(x)) + C$$

### Most common integrals

$$\int e^x dx = e^x + C$$
$$\int \sin(x) dx = -\cos(x) + C$$
$$\int \cos(x) dx = \sin(x) + C$$
$$\int \sec^2(x) dx = \tan(x) + C$$
$$\int \frac{1}{x} dx = \ln(n) + C$$
$$\int a^x dx = \frac{a^x}{\ln(a)} + C$$
$$\int \ln(x) dx = x \ln(x) - x + C$$

## Appendix B: Some Common Integrations

$$\DeclareMathOperator\arctanh{arctanh}$$
$$\DeclareMathOperator\arcsinh{arcsinh}$$
$$\DeclareMathOperator\arccosh{arccosh}$$
$$\DeclareMathOperator\cosec{cosec}$$

$$\int\frac{dx}{1 + x^2} = \arctan(x) + C$$
$$\int \frac{dx}{1 - x^2} = \arctanh(x) + C$$
$$\int \frac{dx}{\sqrt{1-x^2}} = \arcsin(x) + C = -\arccos(x) + C$$
$$\int \frac{dx}{\sqrt{x^2 +1}} = \arcsinh(x) + C$$
$$\int \frac{dx}{\sqrt{x^2 - 1}} = \arccosh(x)$$


### Trigonometric Functions

#### Power Reduction Laws

$$\int \sin^n(x) dx = -\frac{1}{n} \sin^{n-1}(x) \cos(x) + \frac{n-1}{n} \int \sin^{n-2} dx$$
$$\int \cos^n(x) dx = \frac{1}{n} \cos^{n-1}(x) \sin(x) + \frac{n-1}{n} \int \cos^{n-2} dx$$

$$\int \sec^n(x) dx = \frac{1}{n-1} \sec^{n-1}(x) \sin(x) + \frac{n-2}{n-1} \int \sec^{n-2}(x) dx$$
$$\int \csc^m(x) dx = -\frac{1}{n-1} \csc^{n-1}(x) \cos(x) + \frac{n-2}{n-1} \int \csc^{n-2}(x) dx$$

$$\int \tan^n(x) dx = \frac{\tan^{n-1}(x)}{n-1} - \int \tan^{n-2}(x) dx$$
$$\int \cot^n(x) dx = \frac{\cot^{n-1}(x)}{n-1} - \int \cot^{n-2}(x) dx$$

#### Assorted Trigonometric Functions

$$\int \sin^2(2x) = - \frac{\sin(4x) - 4x}{8} + C$$
$$\int \cos^2(2x) = \frac{\sin(4x) + 4x}{8} + C$$
$$\int \frac{\sin(x)}{\cos^2(x)} dx = \sec(x) + C $$
$$\int \frac{\cos(x)}{\sin^2(x)} dx = -\cosec(x) + C$$
$$\int \tan(x) dx = -\ln(\cos(x)) + C$$
$$\int \cot(x) dx = \ln(\sin(x)) + C$$
$$\int \frac{dx}{\sin(x)} = \ln(\tan(x/2)) + C$$
$$\int \frac{dx}{\cos(x)} = \ln \left(\tan \left(\frac{\pi}{4} + \frac{x}{2}\right) \right) + C = \ln(\sec(x) + \tan(x)) + C$$
$$\int \frac{dx}{\sin^2 (x)} = -\cot(x) + C$$
$$\int \frac{dx}{\cos^2(x)} = \tan(x) + C$$

### Hyperbolic Trigonometric Functions

$$\int \sinh(x) dx = \cosh(x) + C$$
$$\int \cosh(x) dx = \sinh(x) + C$$
$$\int \frac{dx}{\sinh^2 (x)} = -\coth(x) + C$$
$$\int \frac{dx}{\cosh^2(x)} = \tanh(x) + C$$
$$\int \tanh(x) dx = \ln(\cosh(x)) + C$$
$$\int \coth(x) dx = \ln(\sinh(x)) + C$$
$$\int \frac{dx}{\sinh(x)} = \ln(\tanh(x/2)) + C$$





