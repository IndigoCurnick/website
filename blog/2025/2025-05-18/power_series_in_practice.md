## Introduction to Power Series

We saw in previous blogs ([here](/blog/2025-05-04/testing-infinite-series-for-convergence) and [here](/blog/2025-04-12/infinite-series-and-the-zeno-paradox)) about infinite series of just numbers. In this blog we will look at a similar idea, but now the series will be of a variable. Here, the variable will be \\(x\\) or \\(x - a\\), which is called a *power series*. Power series are actually even more useful in physics than general infinite series, as we shall see.

By definition then, a power series is of the following two forms:

\\[\sum^\infty_{n=0} a_n x^n = a_0 + a_1 x + a_2 x^2 + a_3 x^3 + \cdots\\]

\\[\sum^\infty_{n=0} a_n (x-a)^n = a_0 + a_1 (x-a) + a_2 (x-a)^2 + a_3 (x-a)^3 + \cdots\\]

Where \\(a_n\\) are constants.

For a given power series, whether it converges depends on what value of \\(x\\) we are evaluating. Recall [from earlier blogs](/blog/2025-05-04/testing-infinite-series-for-convergence) the ratio test, where we divide the term \\(n+1\\) by \\(n\\) and take the absolute value of this to get \\(\rho_n\\) and then take the limit of \\(\rho_n\\) as \\(n\\) approaches infinity to get \\(\rho\\). Then if \\(\rho < 1\\) the series converges and if \\(\rho > 1\\) the series diverges. 

Consider the following series

\\[1 - \frac{x}{2} + \frac{x^2}{4} - \frac{x^3}{8} + \cdots \frac{(-x)^n}{2^n} + \cdots\\]

The ratio here would be 

\\[\rho_n = \left\vert \frac{(-x)^{n+1}}{2^{n+1}} \div \frac{(-x)^n}{2^n} \right\vert = \left\vert \frac{x}{2} \right\vert\\]

Therefore as \\(n \rightarrow \infty\\) trivially we have

\\[\rho = \left\vert \frac{x}{2} \right\vert\\]

Remember that the boundaries for convergence and divergence happen when \\(\rho >1\\) and \\(\rho < 1\\), so for this series it converges when \\(\vert x \vert < 2\\) and diverges when \\(\vert x \vert > 2\\). In other words, we can think of this series as converging between \\(x = -2\\) and \\(x=2\\), this concept is known as the *interval of convergence*.


Power series have some interesting properties:

1. A power series may be differentiated or integrated term by term; the resulting series converges to the derivative or integral of the function represented by the original series within the same interval of convergence as the original series
2. Two power series may be added, subtracted, or multiplied; the resultant series converges at least in the common interval of convergence. You may divide two series if the denominator series is not zero at \\(x = 0\\), or if it is and the zero is cancelled by the numerator. The resulting series will have some interval of convergence (which can be found by the ratio test)
3. One series may be substituted in another provided that the values of the substituted series are in the interval of convergence of the other series
4. The power series of a function is unique, that is, there is just one power series of the form \\(\sum^\infty_{n=0} a_n x^n\\) which converges to a given function

It takes some more advanced calculus than we can really go into here to prove these four properties, so that will be left for another day.

## Taylor Series

A Taylor series is a method of approximating a function which is complicated as a simpler polynomial. In effect, we can turn a function like \\(\sin(x)\\) into a power series. Later on, we'll see some of the uses of such a technique.

We'll work by "expanding" (hence *Taylor expansion*) the function \\(\sin(x)\\), and we will start by assuming that such a series exists (in a moment we will discuss this point in a bit more detail). The series would be

\\[\sin(x) = a_0 + a_1 x + a_2 x^2 + \cdots + a_n x^n + \cdots\\]

This must hold when \\(x=0\\), so we can substitute that in, and we find that the LHS is 0, and the RHS is just \\(a_0\\), thus \\(a_0 = 0\\). The next step is to differentiate the whole series:

\\[\cos(x) = a_1 + 2a_2 x + 3a_3 x^2 + \cdots\\]

Where once more we can substitute in \\(x = 0\\) and find \\(a_1 = 1\\). Once more we differentiate to get

\\[-\sin(x) = 2a_2 + 6a_3 x + 12 a_4 x^2 + \cdots\\]

\\[a_2 = 0\\]

We can continue the process like so:

\\[-\cos(x) = 3 \cdot 2 a_3 + 4 \cdot 3 \cdot 2 a_4 x + \cdots\\]

\\[a_3 = - \frac{1}{3!}\\]

\\[\sin(x) = 4 \cdot 3 \cdot 2 a_4 + 5 \cdot 4 \cdot 3 \cdot 2 a_5 x + \cdots\\]

\\[a_5 = \frac{1}{5!}\\]

Which gives us

\\[\sin(x) = x - \frac{x^3}{3!} + \frac{x^5}{5!} - \cdots\\]

A series obtained in this way is called a *Maclaurin series*, named after Colin Maclaurin who used this series a lot during the 18th century. The Maclaurin series is actually a special case of the *Taylor series*, introduced by Brook Taylor. In the Maclaurin series, we consider the derivatives at \\(x=0\\), but in general we can also consider the derivative at \\(x = a\\).

Let's now do a generic Taylor series for \\(f(x)\\) (assuming it exists).

\\[f(x) = a_0 + a_1 (x-a) + a_2 (x-a)^2 + a_3 (x-a)^3 + a_4(x-a)^4 + \cdots + a_n (x-a)^n + \cdots\\]
\\[f^\prime (x) = a_1 + 2a_2 (x-a) + 3a_3 (x-a)^2 + 4a_4 (x-a)^3 + \cdots + na_n (x-a)^{n-1} + \cdots\\]
\\[f^{\prime \prime}(x) = 2a_2 + 3 \cdot 2 a_3 (x-a) + 4 \cdot 3a_4 (x-a)^2 + \cdots + n(n-1)a_n(x-a)^{n-2} + \cdots\\]
\\[f^{\prime \prime \prime}(x) = 3! a_3 + 4 \cdot 3 \cdot 2 a_4 (x-a) + \cdots + n(n-1)(n-2) a_n (x-a)^{n-3} + \cdots\\]
\\[\vdots\\]
\\[f^{(n)}(x) = n(n-1)(n-2) \cdots 1 \cdot a_n + \cdots\\]

By the way, as a helpful piece of advice, it's best to not multiply out numbers. For example, keep it of the form \\(4 \cdot 3 \cdot 2\\) because factorials are very very common in Taylor series and this will help you spot them.

We can now substitute \\(x=a\\)

\\[f(a) = a_0\\]
\\[f^\prime (a) = a_1\\]
\\[f^{\prime \prime} (a) = 2a_2\\]
\\[f^{\prime \prime \prime} (a) = 3! a_3\\]
\\[f^{(n)} (a) = n! a_n\\]

Thus the Taylor series about \\(x=a\\) is 

\\[f(x) = f(a) + (x-a) f^\prime (a) + \frac{1}{2!} (x-a)^2 f^{\prime \prime} (a) + \cdots \frac{1}{n!} (x-a)^n f^{(n)}(a) + \cdots\\]

And then the specific Maclaurin series when \\(x=0\\) gives

\\[f(x) = f(0) + x f^\prime (0) + \frac{x^2}{2!} f^{\prime \prime}(0) + \frac{x^3}{3!} f^{\prime \prime \prime} (0) + \cdots + \frac{x^n}{n!} f^{(0)} (0) + \cdots\\]

## Alternative Methods for Finding Taylor Series

While it's helpful to have these in general for finding series, we often have a few simpler methods we can try. Recall we said earlier that there is a unique power series for each function: therefore, no matter what method we use to obtain it we can be sure we are getting the exact same series. 

For using these alternative methods, you usually begin with a power series you already know. The appendix at the end of this blog contains some common series, which you should review!

### Multiply a Known Series by Another Polynomial

The first technique is to multiply an already known series by a polynomial or another series. For example, when finding \\((x+1) \sin(x)\\) we can actually just multiply the series we already found by \\(x+1\\)

\\[(x+1) \sin(x) = (x+1) \left( x - \frac{x^3}{3!} + \frac{x^5}{5!} - \cdots \right) = x + x^2 - \frac{x^3}{3!} - \frac{x^4}{3!} + \cdots\\]

### Divide a Known Series by Another Polynomial

The second technique is division by a polynomial or series. If we want to find \\((1/x) \ln(1+x)\\) then we get

\\[\frac{1}{x} \ln(1+x) = 1 - \frac{x}{2} + \frac{x^2}{3} - \frac{x^3}{4} + \cdots\\]

(again, see appendix!)

### Using the Binomial Theorem

The third technique is using the binomial theorem. In the appendix I list the expansion for \\((1+x)^p\\) which you might notice is basically the binomial theorem. Let's try expanding \\(\sqrt{1+x}\\)

\\[(1+x)^{\frac{1}{2}} = \sum^\infty_{n=0} \binom{1/2}{n} x^n\\]
\\[(1+x)^{\frac{1}{2}} = 1 + 
\frac{1}{2}x + \frac{\frac{1}{2} \left(- \frac{1}{2} \right) }{2!} x^2 +
\frac{\frac{1}{2} \left( -\frac{1}{2}  \right) \left( - \frac{3}{2} \right) }{3!} x^3 +
\frac{\frac{1}{2} \left( -\frac{1}{2} \right) \left( - \frac{3}{2} \right) \left( - \frac{5}{2} \right)}{4!} x^4 + \cdots\\]
\\[\sqrt{1+x} = 1 + \frac{1}{2}x - \frac{1}{8} x^2 + \frac{1}{16} x^3 - \frac{5}{128} x^4 + \cdots\\]

### Substitution 

The fourth method is to substitute a polynomial or series for a variable in another series. For example, we know the series for \\(e^x\\) (see appendix), so to find \\(e^{-x^2}\\) we really only have to replace \\(x\\) with \\(-x^2\\)

\\[e^{-x^2} = 1 - x^2 + \frac{(-x^2)^2}{2!} + \frac{(-x^2)^3}{3!} + \cdots\\]
\\[e^{-x^2} = 1 - x^2 + \frac{x^4}{2!} - \frac{x^6}{3!} + \cdots\\]

### Finding General Taylor Series by Using Maclaurin Series

The final series we shall look at is finding a Taylor series using a Maclaurin series. For example, finding the Taylor series for \\(\ln(x)\\) about \\(x=1\\) we can start by writing 

\\[\ln(x) = \ln(1 + (x-1))\\]

And then use the series from the appendix with \\(x\\) replaced by \\(x-1\\)

\\[\ln(x) =
\ln(1 + (x-1)) =
(x-1) - \frac{1}{2}(x-1)^2 + \frac{1}{3} (x-1)^3 - \frac{1}{4}(x-1)^4 + \cdots\\]

## Limitations of Taylor Series

So far, we've essentially been assuming that a Taylor series for a given function exists. As part of this assumption, we've also been assuming this Taylor series converges to the original function. However, this is unfortunately not always the case.

Since this is a practical article, we won't delve deeply into the theory here, but for reference there is a concept called the Lagrange's Remainder Theorem. This theorem helps us understand how good an approximation to the original function we are making. I will cover this in detail in a later article.

The big take away is that not all functions can be approximated to arbitrary precision everywhere by a Taylor series expansion. Some can. Some can be approximated to arbitrary precision near to some points. But in general, the approximation is only an approximation.

## Applications of Taylor Series

### Numerical Computation

One of the primary uses of the Taylor series is computing functions which are under normal circumstances tricky to do. Let's try evaluating \\(f(x) = \ln(\sqrt{(1+x)/(1-x)}) - \tan(x)\\) at \\(x=0.0015\\\). If you try this on a calculator, it will likely get the answer wrong. Let's compute it (mostly) by hand

\\[\ln(\sqrt{(1+x)/(1-x)}) = x + \frac{x^3}{3} + \frac{x^5}{5} + \frac{x^7}{7} + \cdots \approx 0.001500001125001518752441\\]

\\[\tan(x) = x + \frac{x^3}{3} + \frac{2x^5}{15} + \frac{17x^7}{315} + \cdots \approx 0.001500001125001012500922\\]
Thus

\\[f(x) = \frac{x^5}{15} + \frac{4x^7}{25} + \cdots \approx 5.0625 \times 10^{-16}\\]

The difference is in the 16th decimal place of those two numbers, so any computer making any kind of approximation before then will lose precision and get it wrong!

### Approximating Integrals

Integration is HARD, and even some "trivial" integrals do not have nice neat analytic solutions. Consider the Fresnel integrals (\\(\sin(x^2)\\) and \\(\cos(x^2)\\)). However, if we use Taylor series we can actually do it fairly easily

\\[\int^t_0 \sin(x^2) dx = \int^t_0 \left( x^2 - \frac{x^6}{3!} + \frac{x^10}{5!} + \cdots \right) dx = \frac{t^3}{3} - \frac{t^7}{7 \cdot 3!} + \frac{t^11}{11 \cdot 5!}- \cdots\\]

### Differential Equations

Taylor series are often solutions to ordinary differential equations and partial differential equations. I won't give an example here as this is WELL beyond the scope of this blog, but rest assured once I have written a blog on series solutions to differential equations I will link it here! Just be aware that this is commonly seen in physics.

As a taste of this, let's see reducing a complex equation into a simpler equation. 

The motion of a pendulum is given by

\\[\ddot{\theta} + \frac{g}{L} \sin(\theta) = 0\\]

Which is pretty nasty to deal with! But what if we could simplify it? As we know

\\[\sin(\theta) = \theta - \frac{\theta^3}{3!} + \frac{\theta^5}{5!} - \cdots\\]

Notice how if \\(\theta\\) is pretty small, then \\(\theta^3\\) is triply small, not to mention divided by \\(3!\\). So *as long as the angle is small* we can make the following approximation

\\[\sin(\theta) = \theta\\]

and so 

\\[\ddot{\theta} + \frac{g}{L} \theta = 0\\]

Which is so much easier to deal with!

## Appendix: Common Maclaurin Series Expansions

This appendix contains some common Taylor series about \\(x=0\\). It also contains the convergence limits. If you are a student of the physical sciences or mathematics, you would do well to memorise the expansions of \\(\sin(x)\\), \\(\cos(x)\\), \\(e^x\\), \\(\ln(1+x)\\) and \\((1+x)^p\\)

\\[\sin(x) = \sum^\infty_{n=0} \frac{(-1)^n x^{2n +1}}{(2n+1)!} = x - \frac{x^3}{3!} + \frac{x^5}{5!} - \frac{x^7}{7!} + \cdots, \quad \text{all } x\\]

\\[\cos(x) = \sum^\infty_{n=0} \frac{(-1)^n x^{2n}}{(2n)!} = 1 - \frac{x^2}{2!} + \frac{x^4}{4!} - \frac{x^6}{6!} + \cdots, \quad \text{all } x\\]

\\[e^x = \sum^\infty_{n=0} \frac{x^n}{n!} = 1 + x + \frac{x^2}{2!} + \frac{x^3}{3!} + \frac{x^4}{4!} + \cdots, \quad \text{all } x\\]

\\[\ln(1 + x) = \sum^\infty_{n=1} \frac{(-1)^{n+1}x^n}{n} = x - \frac{x^2}{2} + \frac{x^3}{3} - \frac{x^4}{4} + \cdots, \quad -1 < x \leq 1\\]

\\[(1+x)^p = \sum^\infty_{n=0} \binom{p}{n} x^n = 1 + px + \frac{p(p-1)}{2!}x^2 + \frac{p(p-1)(p-2)}{3!}x^3 + \cdots, \quad \lvert x \rvert < 1\\]

\\[\frac{1}{1-x} = \sum^\infty_{n=0} x^n = 1 + x + x^2 + x^3 + \cdots, \quad -1 \leq x < 1\\]

\\[\frac{1}{1+x} = \sum^\infty_{n=0} (-1)^n x^n = 1-x+x^2 - x^3 + \cdots + (-1)^n x^n + \cdots, \quad -1 < x \leq 1\\]

\\[\ln(1-x) - \sum^\infty_{n=1} \frac{1}{n} x^n = -x - \frac{x^2}{2} - \frac{x^3}{3} - \cdots - \frac{x^n}{n} - \cdots, \quad -1 < x \leq 1\\]

\\[\tan^{-1}(x) = \sum^\infty_{n=0} (-1)^n \frac{x^{2n+1}}{2n+1} = x - \frac{x^3}{3} + \frac{x^5}{5} - \frac{x^7}{7} + \frac{x^9}{9} - \cdots, \quad 1 \leq x \leq 1\\]


## References

Boas, M., L. (2005). *Mathematical Methods in the Physical Sciences (3rd ed.)*. Wiley

Abbott, S. (2015). *Understanding Analysis (2nd ed.)*. Springer

