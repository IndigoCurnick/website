## Introduction and Motivation

Differentiation is an essential skill for anyone who wants to get into physics, mathematics, computer science, engineering or any technical field.

Why do we want to differentiate? What purpose can it serve? The reason for differentiation is that we want to know the gradient, or steepness, of a function at any given point.

For straight line functions, this is almost uneccessary. You are probably familiar with the straight line function

\\[y = mx + c\\]
In this case the gradient of this function is \\(m\\) everywhere. However, how do we know this? In this case, it's fairly trivial. A brief consideration is all it takes to realise the slope of this line is \\(m\\). 

As you will soon learn, differentiation will tell us that 

\\[\frac{dy}{dx} = m\\]
which is how we formally know the gradient is \\(m\\) everywhere.

However, what if the function is more complex? What about

\\[y = ax^4 + bx^3 - cx^2 + dx - e\\]
Can we do this by simple inspection? No, and the main reason is because the gradient of this function changes as \\(x\\) changes.

Okay, but why we want to know the gradient of a function? Why do we even care about this? Let's just quickly look at three examples from different fields. 

In physics, differentiation is used in the study of heat. The temperature of a metal which is suddenly exposed to heat on one side is actually a function and it's gradient can tell us things like how that heat is being distributed through the metal and how fast.

In civil engineering structures like supports in buildings are constantly under stress and load, and we use differentiation to understand those forces and make sure that those buildings don't collapse.

In computer science we also use differentiation to compress images. An image can actually be seen as a kind of function - a surface - with different values. By exploiting this, we can actually use differentiation to help us reduce the size of an image without really impacting its appearance much (in fact, I use such image compression on this website to give really fast loading times!).

So, differentiation shows up all of the time. We best learn how to do it!

## Differentiating Polynomials

Polynomials are very simple to differentiate. Here we use what we call the power rule. Consider

\\[y = x^n\\]

Now, the way I often remember this is "bring the power down, reduce the power by one". So

\\[\frac{dy}{dx} = n x^{n-1}\\]

As for a proper example, consider 

\\[y = 4x^2 - 2x + 3\\]

This would become

\\[\frac{dy}{dx} = 8x - 2\\]

Notice how the \(+3\) drops away. If you think back to \(y=mx+c\) you can see how moving the line intersect up and down by varying \\(c\\) has no impact on the gradient. In other words, for gradients, constants don't matter. 

We can actually differentiate multiple times. Let's do it again

\\[\frac{d^2 y}{d x^2} = 8\\]

You could keep going forever like this, though in this case the third derivative and all those following would be 0. Notice how the squared is placed very specifically - over the \\(d\\) in in the numerator and over the \\(x\\) in the denominator.

Some notes on notation. The \\(\frac{dy}{dx}\\) means we are differentiating \\(y\\) by, or with respect to, \\(x\\). Now there are a few competing ways to do differentiation notation. Often when differentiating functions like \\(f(y)\\) it's common to write \\(f^\prime (y)\\). What if you want the second derivative? Very simple \\(f^{\prime \prime} (y)\\). Now this can obviously be a bit unwieldy by the time you arrive at \\(f^{\prime \prime \prime \prime \prime} (y)\\). But thankfully, we rarely need anything past the second derivative. In physics it's very common to see a dot notation. So if the position of some object is the function \\(x\\), it's common to describe the derivative of that function i.e. the velocity as \\(\dot{x}\\). The derivative of that i.e. the acceleration is \\(\ddot{x}\\). You see this notation sometimes too in computer science or engineering, it's almost always with position, velocity and acceleration though.

### Problems

Try out these problems! Differentiate all of the following with respect to \\(x\\). Solutions in the companion article

1. \\(y = 2x^2 + 3\\)
2. \\(y = 3x^3 + 4x^2 - x + 6\\)
3. \\(y = -2x^4 - 3x^3 - 7x\\)
4. \\(y = 5x^5 + x^4\\)
5. \\(y = 12\\)

## Differentiation From First Principles

I think it's helpful to do some differentiation first with a rule to get comfortable with the idea. However, from where does this power rule we just studied actually come from?

The general idea behind differentiation is to find the gradient of some arbitrarily shaped function at a specific point. If we consider some curvy function like \\(y = x^2\\) the gradient is obviously not constant. Even worse, those curves don't have any nice straight lines we can apply \\(y =mx + c\\) to. But, what if there was a straight line? 

Sounds weird, but what if we took too points along the curve, \\(x_0\\) and \\(x_1\\) and we took the the gradient of the straight line between those two points? Now that's not especially helpful as it won't follow the curve. But, here comes the BIG idea: what if \\(x_0\\) and \\(x_1\\) were *really* close together? What if they were only an infinitesimal distance apart? This is essentially the foundational idea of calculus. I won't go into full detail on limits in this article, but the idea here is to take the gradient between these two points as the limit of the distance between them goes to zero. That will give us a gradient at that point!

As exciting as that idea is, how do we actually do it?

It's not too hard - we simply take some point and add some tiny amount. We call this infinitesimal distance \\(\Delta x\\). \\(\Delta\\) is the Greek letter delta - which is where the \\(d\\) in \\(\frac{dy}{dx}\\) comes from, too. It's standard to use the characters \\(\Delta\\), \\(\delta\\) (lower case delta) or \\(d\\) to mean "a really small thing".

So to be exact we do

\\[f^\prime (x) \equiv \frac{df(x)}{dx} \equiv \lim_{\Delta x \rightarrow 0} \frac{f(x + \Delta x) - f(x)}{\Delta x}\\]

So let's see that in an actual example. We'll differentiate our old friend \\(f(x) = mx + c\\)

\\[f^\prime (x) = \lim_{\Delta x \rightarrow 0} \frac{(m(x + \Delta x) + c) - (mx + c)}{\Delta x}\\]

\\[f^\prime (x) = \lim_{\Delta x \rightarrow 0} \frac{mx + m \Delta x + c - mx - c}{\Delta x}\\]

Let's just cancel out all the "normal" stuff here but we'll leave the \\(\Delta x\\) for just a moment

\\[f^\prime (x) = \lim_{\Delta x \rightarrow 0} \frac{m \Delta x}{\Delta x}\\]

While the \\(\lim_{\Delta x \rightarrow 0}\\) might seem scary, just ignore it for now. In all cases, \\(\frac{\Delta x}{\Delta x} = 1\\). So

\\[f^\prime (x) = \lim_{\Delta x \rightarrow 0} m\\]

What's the limit of \\(m\\) as \\(\Delta x\\) approaches 0? Well, it's just \\(m\\)! It doesn't depend upon \\(\Delta x\\) at all.

\\[f^\prime (x) = m\\]

That is essentially how we know for sure that \\(m\\) is the gradient of \\(y = mx + c\\)!

### Problems

Differentiate from first principles with respect to \\(x\\) the following polynomials

1. \\(y = 3x\\)
2. \\(y = 2x^2 + 5x\\)
3. \\(y = x^3 - x\\)
4. \\(y = c\\)

## Finding Stationary Points with Differentiation

A key use of differentiation is to find so called stationary points. These are points on a function where the gradient is 0. The graphs below show some of these stationary points. There's three types: maxima, minima and inflection points. How can we find these for any function?

It's actually very simple. All we have to do is set

\\[\frac{dy}{dx} = 0\\] and then solve for \\(x\\). 

How do we know whether we've found a maxima, minima or inflection point? We look at the second derivative. If the second derivative at \\(x\\) is negative, we have a maxima. If the second derivative at \\(x\\) is positive, we have a minima. If the second derivative at \\(x\\) is 0 we have an inflection point. Let's work out an example. Consider 

\\[y = 3x^4 + 4x^3 - 12x^2 + 6\\]

We start by taking the derivative

\\[\frac{dy}{dx} = 12x^3 + 12x^2 - 24x\\]

Set it equal to 0 and solve for \\(x\\)

\\[12x^3 + 12x^2 - 24x = 0\\]
\\[x^3 + x^2-2x = 0\\]
\\[x (x^2 + x - 2)=0\\]
\\[x (x + 2)(x-1) = 0\\]

So we have \\(x=0, x= -2, x=1\\)

We now need the second derivative

\\[\frac{d^2 y}{dx^2} = 36x^2 + 24 x - 24\\]

Which we now evaluate at each of the solutions

\\[x=0, \frac{d^2 y}{dx^2} = -24\\]

\\[x = -2, \frac{d^2 y}{dx^2} = 36 \cdot (-2)^2 + 24 \cdot -2 - 24 = 72\\]

\\[x=1, \frac{d^2 y}{dx^2} = 36 + 24 - 24 = 36\\]

So in summary \\(x=0\\) is a maxima and \\(x = -2\\) and \\(x = 1\\) are minima

If you want to find the exact points, you can substitute the \\(x\\) values into the original function. So for us 

\\[x=0, y = 6\\]

\\[x = -2, y = 3(-2)^4 + 4(-2)^3 - 12(-2)^2 + 6 = -26\\]

\\[x = 1, y = 1\\]


### Problems

Find the stationary points for the following functions

1. \\(y = 2x^2 + x - 1\\)
2. \\(y = -3x^3 - 2x\\)
3. \\(y = x^4 + 3x^3 - 10x^2 -24x\\)

## Differentiating Some Special Functions

Let's look at some special functions of interest. 

The exponential function is the easiest possible to differentiate

\\[\frac{d}{dx} e^x = e^x\\]

In other words, the derivative of \\(e^x\\) is \\(e^x\\)!

The natural logarithm is 

\\[\frac{d \ln(x)}{dx} = \frac{1}{x}\\]

However, this is only valid when \\(x>0\\)

We can also look at the trigonometric functions 

\\[\frac{d \sin(x)}{dx} = \cos(x)\\]
\\[\frac{d \cos(x)}{dx} = - \sin(x)\\]
\\[\frac{d \tan(x)}{dx} = \sec^2(x)\\]

## Differentiation Product Rule

The product rule concerns when we have two functions multiplied

\\[\frac{d}{dx} f(x) g(x) = f^\prime (x) g(x) + f(x) g^\prime (x)\\]

So a worked example, let's try differentiating \\(y = x \sin(x)\\)

\\[\frac{d}{dx} x \sin(x) = \sin(x) + x \cos(x)\\]

Let's try another

\\[\frac{d}{dx} x^2 e^x = 2x e^x + x^2 + e^x\\]


### Problems

Differentiate the following functions with respect to \\(x\\)

1. \\(y = x^3 \cos(x)\\)
2. \\(y = (2x^2 - x) \sin(x)\\)
3. \\(y = (5x^3) e^x\\)
4. \\(y = 6x^3 \ln(x)\\)
5. \\(y = e^x \tan(x)\\)
6. Prove the product rule from first principles. Hint: when taking limits, think about what the definition of differentiation is. You'll need to use a clever trick.

## Differentiation Quotient Rule

The quotient rule is as follows

\\[\frac{d}{dx} \frac{f(x)}{g(x)} = \frac{g(x)f^\prime(x) - f(x)g^\prime(x)}{(g(x))^2}\\]

So let's try an example

\\[y = \frac{6x^2}{2-x}\\]
\\[\frac{dy}{dx} = \frac{(2-x)12x + 6x^2}{(2-x)^2}\\]
\\[\frac{dy}{dx} = \frac{24x - 6x^2}{(2-x)^2}\\]

And another

\\[y = \frac{e^x}{\sin(x)}\\]
\\[\frac{d}{dx} \frac{e^x}{\sin(x)} = \frac{\cos(x) e^x - e^x \sin(x)}{e^{2x}}\\]
\\[\frac{d}{dx} \frac{e^x}{\sin(x)} = \frac{e^x (\cos(x) - \sin(x))}{e^{2x}}\\]

### Problems

Differentiate the following functions with respect to \\(x\\)

1. \\(y = \frac{\sin(x)}{x}\\)
2. \\(y = \frac{\tan(x)}{e^x}\\)
3. \\(y = \frac{x^2 - x}{2x^3 + x^2}\\)
4. \\(y = \frac{e^x}{5x^3 + x^2}\\)
5. Prove the quotient rule

## Differentiation Chain Rule

The chain rule is for a function of a function

\\[\frac{d}{dx} f(g(x)) = f^\prime(g(x)) g^\prime (x)\\]

Let's try an example 

\\[y = (6x^2 + 7x)^4\\]

\\[\frac{dy}{dx} = 4(6x^2 + 7x)^3 (12x + 7)\\]

Another example

\\[y = \sin(x^2)\\]

\\[\frac{d \sin(x^2)}{dx} = 2x \cos(x^2)\\]


### Problems

Differentiate the following functions by \\(x\\)

1. \\(y = \tan(10x + 4)\\)
2. \\(y = e^{x^2}\\)
3. \\(y = e^{1-\cos(x)}\\)
4. \\(y = \sin(2 + \cos(x))\\)

## Assorted Differentiation Problems

### Stationary Point Problems

Find the stationary points of the following functions. Are they maxima, minima or inflection points?

1. \\(y = x^2 - \ln(x)\\)
2. \\(y = 2x^4 - 10x^2 +13x\\)
3. \\(y = x^3 + 9x^2 -48x + 2\\)

### Gradient Problems

Find the Gradient of the functions at the specified position

1. \\(y = e^x tan(x), x=1\\)
2. \\(y = \frac{x^2 - 2x}{e^x}, x=-1\\)
3. \\(y = 2x^3 + 5x^2 - 5x + 2, x = 5\\)
4. \\(y = (x^3 + 2x)^3, x=3\\)
5. \\(y = sin(x^2 + e^x), x=1\\)

### Position of an Object

The position of the object is defined by \\(x = 3t^4 - 12t^3 + 40t^2\\)

1. Determine the velocity of the object at any time \\(t\\)
2. Determine the acceleration of the object at any time \\(t\\)
3. Does the object ever stop moving? When?

### Population of Bacteria

The population of a colony of bacteria is given by \\(p = e^{t-5} + 12\\)

1. How fast is the population growing at \\(t=15\\)
2. What is the population at \\(t = 20\\)?

### Competing Bacteria

Two bacteria colonies have populations defined by \\(a = x - \frac{x^2}{200} + 2\\) and \\(b = 4x - \frac{x^3}{500} + 3\\). All of these colonies are destined to die out.

1. Which colony peaks with the higher population? Which peaks sooner?
2. Which colony dies out first? Which colony dies out last?
3. What is the starting population of each colony?
