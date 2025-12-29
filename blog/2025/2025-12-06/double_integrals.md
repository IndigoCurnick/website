
## Introduction

We know from [a previous blog on integration](/blog/2025-05-31/introduction-to-integration) that we can compute the area under a curve using an integral. What happens if we want to integrate under a function $f$ which is a continuous, non-negative valued function of two variables defined on the closed rectangle 

$$R = [a,b] \times [c,d]  = \{ (x,y) \in \mathbb{R}^2 | a \leq x \leq b, c \leq y \leq d \}$$

Let's just stick with this simplistic case for now of an unbroken surface which never dips below the $xy$-plane. To find the volume under such a surface we can employ Cavalieri's Principle which is something we're already familiar with from basic integration. In standard integration we "slice up" the curve into many many little pieces to justify our methods. Here, we can also slice up the shape in the same way. First, we make one series of slices such that we are working out an area under a curve. We already know how to solve that case - it is just standard integration.

Imagine we wanted to work out the total mass of some 2D shape which had some varying mass density across the shape. We could slice the shape up into a series of thin strips, integrate each strip and add them up. Another method (which we will discuss) in a moment is to slice it up into many small squares and take some test point in that square as the value and add them all up. Either way, the point is that the same kinds of arguments and techniques which took us to integration can be used to take us to double integration.

## Double Integrals via Riemann Sums

Practically working though this slicing up, we could divide the region $R$ into small rectangles $R_{ij}$, each with area $\Delta A$ and with sides $\Delta x$ and $\Delta y$. We can do this by splitting $[a,b]$ into $m$ subintervals and $[c,d]$ into $n$ subintervals. In other words

$$\Delta x = \frac{b-a}{m}, \Delta y = \frac{d-c}{n}, \Delta A = \Delta x \Delta y$$
We can now get the volume of this thin box by taking some sample point inside the box which we'll call $(x^\prime_{ij}, y^\prime_{ij})$. This gives us a value here of $f(x^\prime_{ij}, y^\prime_{ij})$. Obviously this will not be the value at all points within the $R_{ij}$th region, but astute observers will see that we'll probably end up taking a limit soon, thus making this approximation basically correct. Therefore, the total volume of $R_{ij}$ is given by  $f(x^\prime_{ij}, y^\prime_{ij}) \Delta A$.

We could then add up all of these volumes to get an approximation of the volume of the solid by

$$V \approx \sum^m_{i=1} \sum^n_{j=1}  f(x^\prime_{ij}, y^\prime_{ij}) \Delta A$$

This is called a double Riemann sum. As I alluded to a moment ago, if we took limits here and made each $R_{ij}$ so tiny that  $f(x^\prime_{ij}, y^\prime_{ij})$ was effectively the value at all points in $R_{ij}$ we wouldn't have an approximation but just the value. We could write that like so

$$V = \lim_{m,n \rightarrow \infty} \sum^m_{i=1} \sum^n_{j=1}  f(x^\prime_{ij}, y^\prime_{ij}) \Delta A = \lim_{\Delta x, \Delta y \rightarrow 0} \sum^m_{i=1} \sum^n_{j=1}  f(x^\prime_{ij}, y^\prime_{ij}) \Delta A$$
We can either think about making the number of subdivisions infinite or we can think about making the sides of $R_{ij}$ infinitely small - the result is the same in either case.

This gives us our definition of a double integral as follows:

$$\iint_R f(x,y) dA = \lim_{m,n \rightarrow \infty} \sum^m_{i=1} \sum^n_{j=1}  f(x^\prime_{ij}, y^\prime_{ij}) \Delta A$$
(You could of course alternatively write the limit in the alternative form given above.)

At this point a lot of textbooks would actually show an example of working out a double integral by doing such a double Riemann sum - I think this is needlessly tedious. Even if $m,n$ are, say, 20, you already have 400 squares to keep track of. That wouldn't even give a very good approximation. I'd, frankly, rather not. Keep this method in mind though as computers would go nuts at the opportunity to crunch it.

## Generalising Double Integrals

In the above section we saw how we could approximate double integrals with Riemann sums, but we ultimately decided that this method is too computationally taxing for us to bother with. We will devote the next few sections to constructing nicer and nicer integrals to help us not have to compute very large sums.

To do this we are going to attack the problem in two ways. First, we'll eliminate Riemann sums. Secondly, you might have noticed that our methods so far only work in the case where the shape we are integrating over is a rectangle from a top down view. We'd actually like to integrate over more arbitrary regions.


### Eliminate Riemann Sums

Just to reiterate here: our goal in this section is to compute a double integral without having to use a Riemann sum. Spoiler: we're eventually going to invoke *Fubini's theorem* to do this, but we shall lay some groundwork first.

Consider $R = [-2,2] \times [-1,3]$ where $f(x,y) =x$. Notice how exactly half of the plane lies above the $xy$-axis and the other half below. If we consider $\iint_R x dA$ as the net volume under the graph of $z=x$, then we can conclude that $\iint_R x dA$ must be zero, if it indeed exists. For another perspective, consider a Riemann sum corresponding to $\iint_R x dA$ obtained by partitioning $R = [-2,2] \times [-1,3]$ symmetrically with respect to the $y$-axis and by choosing the "test points" $\mathbf{c}_{ij}$ symmetrically also. It follows that

$$S = \sum f(\mathbf{c}\_{ij}) \Delta A\_{ij} = \sum x\_{ij} \Delta A\_{ij}$$

is zero because the terms of the sum cancel in pairs. As we shrink the dimensions of the subrectangles to zero to get the integral, we preserve all the symmetry just described. Thus the limit, if it exists, must be zero.

Notice how the phrase "if it exists" has been doing some heavy lifting here. We never proved if $\iint_R f dA$ really exists, and if so, under what conditions. Thankfully, there is the following result (presented without proof) for us to use:

If $f$ is continuous on the closed rectangle $R$, then $\iint_R f dA$ exists.

So for that example, $f(x,y) = x$ is continuous and so the limit exists. 

We can actually generalise the above theorem more - in the case of a function of a single variable there are piecewise continuous functions, which can also be integrated. We can also do the same with functions of two variables, which gives the following result (again, presented here without proof):

If $f$ is bounded on $R$ and if the set of discontinuities of $f$ on $R$ has zero area, then $\iint_R f dA$ exists.

To say that a set has zero area means that we can cover the set $X$ with rectangles $R_1, R_2, \dots, R_n, \dots$ i.e. $X \subseteq \bigcup^\infty_{n=1} R_n$, the sum of whose areas can be made arbitrarily small.

Which leads us to Fubini's theorem (once again, presented without proof): let $f$ be bounded on $R = [a,b] \times [c,d]$ and assume that the set of $S$ of discontinuities of $f$ on $R$ has zero area. If every line parallel to the coordinate axes meets $S$ in finitely many points then

$$\iint_R f dA = \int^b_a \int_c^d f(x,y) dy dx = \int_c^d \int_a^b f(x,y) dx dy$$

The point of Fubini's theorem is that under certain conditions the double integral over a rectangle (i.e. the limit of the Riemann sums) can be calculated by using the iterated integrals, and that such an order does not matter. 

Revisiting $R = [-2,2] \times [-1,3]$ with Fubini's theorem in mind, we can calculate

$$\iint\_R x dA = \int^2\_{-2} \int^3\_{-1} x dy dx = \int^2\_{-2} (xy|^{y=3}\_{y=-1}) dx = \int^2\_{-2} 4x dx = 2x^2 |^2\_{-2} = 0$$

$$\iint\_R x dA = \int^3\_{-1} \int^2\_{-2} x dx dy = \int^3\_{-1} \frac{1}{2} x^2 |^{x=2}\_{x=-2} dy = \int^3\_{-1} (2-2) dy = 0$$

Iterated integrals are obviously much easier to deal with than sums. As you can see in the solutions above, we can do this to obtain an exact answer in one line of calculus.

To build up a bit more intuition about what Fubini's theorem is saying, let's think physically about what's going on. If we imagine a cube, the Riemann sum is iterating over a series of "skyscraper" like structures - tall cuboids. If conditions allow, we can convert this into an iterated integral. In the iterated integral we're splitting the cube into many thin sheets and calculating a volume for each of these sheets.

And to be clear, "when conditions allow" is simply when the function is absolutely integrable on the product space i.e. we can swap a double integral to an iterated integral iff

$$\iint \lvert f(x,y) \rvert dx dy < \infty$$

A bit more formally, imagine slicing the place at $x = x_0$ where $x_0$ is a constant between $a$ and $b$. If we let $A(x_0)$ be the cross sectional area of that slice, then we can see that $A(x_0) dx$ gives the volume of this infinitely thin slice. Therefore

$$V = \int_a^b A(x) dx$$

Is a sum of these thin slabs, and so gives us the volume of the whole shape. $A(x_0)$ is itself nothing more than the area under the curve $z = f(x_0, y)$, which we get from slicing the surface $z = f(x,y)$ with the plane $x = x_0$ and so

$$A(x_0) = \int_c^d f(x_0, y) dy$$
And so we find

$$V = \int_a^b A(x) dx = \int_a^b \left[ \int^d_c f(x,y)dy \right] dx$$

The rhs above is called an iterated integral. In general, with double integration, our goal is to get to an iterated integral, as they can be solved relatively easily by applying two successive "normal" integrations.

As an example, we'll calculate the volume of a cuboid using the above method. We shall bound the box by the planes $z=c, z=0, y = 0, y=b, x=0, x=a$ where $c > 0, b>0, a >0$. In other words, the volume of the cuboid can be found by computing the volume under the graph of $z=c$ for the rectangle 

$$R = \{(x,y) | 0 \leq x \leq a, 0 \leq y \leq b\}$$

So we find that

$$V = \int^a\_0 \int^b\_0 c \\, dy \\, dx = \int^a\_0 (cy |^{y=b}\_{y=0}) \\, dx = \int^a\_0 cb \\, dx = cbx|^{x=a}\_{x=0} = cba$$

Which checks out with what we already know.

A useful property of iterated integrals is that we can evaluate the integrals in whichever order we like.

$$\iint_R f \\, dA = \int_a^b \int_c^d f(x,y) \\, dy \\, dx = \int_c^d \int_a^b f(x,y) \\, dx \\, dy$$

In practicality, we'll choose an order which helps us actually do the integral the easiest. We'll see where that can come in handy in the next section.

### Integrating Over Arbitrary Regions

A completely generalised shape is quite challenging to work through and prove, so we instead turn our attention to elementary regions.

We say that $D$ is an *elementary region* in the plane if it can be described as a subset of $\mathbb{R}^2$ of one of the following three types.

Type 1:

$$D = \{(x,y) | \gamma(x) \leq y \leq \delta(x), a \leq x \leq b \}$$
where $\gamma$ and $\delta$ are continuous on $[a,b]$.

Type 2: 

$$D = \{ (x,y) | \alpha(y) \leq x \leq \beta(y), c \leq y \leq d \}$$

where $\alpha$ and $\beta$ are continuous on $[c,d]$

Type 3: Both type 1 and type 2 at the same time

For example, the unit disc is of type three because it is type 1 because

$$D = \{ (x,y) | -\sqrt{1-x^2} \leq y \leq \sqrt{1-x^2}, -1 \leq x \leq 1 \}$$
and it is type 2 because

$$D = \{ (x,y) | -\sqrt{1-y^2} \leq x \leq \sqrt{1-y^2}, -1 \leq y \leq 1 \}$$

We are now in a position to define $\iint_D f dA$ where $D$ is an elementary region and $f$ is continuous on $D$. We can construct a new function $f^{ext}$ called the "extension of $f$" by

$$f^{ext} (x,y) = \begin{cases} 
f(x,y) & \text{if } (x,y) \in D,\\
0 & \text{if } (x,y) \notin D.
\end{cases}$$

In general $f^{ext}$ is not continuous, but the discontinuities of $f^{ext}$ will all be contained within $\partial D$, which has no area. Hence $f^{ext}$ is integrable on any closed rectangle $R$ that contains $D$. 

The point of the extend function is just a mathematically watertight way to say we're ignoring anything outside of the shape of interest. Thinking again about the mass of the metal plate, the mass density function might be defined outside the boundaries of the plate, but we don't care! We're just ignoring it.

Let $D$ be an elementary region in $\mathbb{R}^2$ and $f$ a continuous function on $D$.

1. If $D$ is of type 1 then

$$\iint_D f dA = \int^b_a \int^{\delta{x}}_{\gamma{x}} f(x,y) \\, dy \\, dx$$
2. If $D$ is of type 2 then

$$\iint_D f dA = \int^d_c \int^{\beta(y)}_{\alpha(y)} f(x,y) \\, dx \\, dy$$

We may prove this theorem by taking $D$ to be described as

$$D = \{ (x,y) | \gamma(x) \leq y \leq \delta(x), a \leq x \leq b \}$$
We have that

$$\iint_D f dA = \iint_R f^{ext} \\, dA$$

where $R$ is any rectangle containing $D$. Let $R = [a^\prime, b^\prime] \times [c^\prime, d^\prime]$, where $a^\prime \leq a, b^\prime \leq b, c^\prime \leq \gamma(x), d^\prime \leq \delta(x)$ for all $x$ in $[a,b]$. Since $f^{ext}$ is zero outside the subrectangle then $R_2 = [a,b] \times [c^\prime, d^\prime]$ and so

$$\iint_R f^{ext} \\, dA \iint_R f^{ext} \\, dA = \int^b_a \int^{d^\prime}\_{c^\prime} f^{ext} (x,y) \\, dy \\, dx$$
by Fubini's theorem. For a fixed value of $X$ between $a$ and $b$, consider the $y$-integral $\int^{d^\prime}\_{c^\prime} f^{ext}(x,y) dy$. Since $f^{ext}(x,y) = 0$ unless $\gamma(x) \leq y \leq \delta(x)$ (in which case $f^{ext} (x,y) = f(x,y)$)

$$\iint^{d^\prime}\_{c^\prime} f^{ext} (x,y) \\, dy = \int^{\delta(x)}\_{\gamma(x)} f(x,y) \\, dy$$
and so

$$\iint\_D f(x,y) \\, dA = \iint\_R f^{ext} \\, dA = \int^b\_a \int^{d^\prime}\_{c^\prime} f^{ext} (x,y) \\, dy \\, dx = \int^b\_a \int^{\delta(x)}\_{\gamma(x)} f(x,y) \\, dy \\, dx$$

Let's see some examples. 

Let $D$ be the region bounded by the parabolas $y=3x^2, y = 4-x^2$ and the $y$-axis. Since $D$ is a type 1 elementary region, we find that

$$\iint_D x^2 y dA = \int_0^1 \int^{4-x^2}_{3x^2} x^2 y \\, dy \\, dx$$

The limits for the inside integration come from the $y$-values of the top and bottom boundary curves of $D$. The limits for the outside integration are the constant $x$-values that correspond to the straight left and right sides of $D$. Once the set up has been done, the integration is quite mechanical so I will skip the steps. Suffice to say

$$\int^1_0 \int^{4-x^2}_{3x^2} x^2 y \\, dy \\, dx = \frac{136}{105}$$


Generally it is useful to think about evaluating double integrals over elementary regions essentially as determining a good order. When the region we need to integrate is a rectangle, Fubini's theorem says that the order is of no significance. 

$$\iint_R f dA = \int^b_a \int^d_c f(x,y) \\, dy \\, dx = \int^d_c \int^b_a f(x,y) \\, dx \\, dy$$

When the region is of type 1 only, we must integrate first with respect to $y$. So

$$\iint_D f \\, dA = \int^b_a \int^{\delta(x)}_{\gamma(x)} f(x,y) \\, dy \\, dx$$

And if type 2 only then

$$\iint_D f dA = \int^d_c \int^{\beta(y)}_{\alpha(y)} f(x,y) \\, dx \\, dy$$

If it is type 3, then we can perform either first. Usually one will be easier than the other, but as many things in integration, it is pure intuition as to which it will be (or simply trying both till you get an answer).
## Properties of Double Integrals

Suppose that $f$ and $g$ are both integrable on the closed rectangle $R$. Then the following properties hold:

1. $f+g$ is also integrable on $R$ and

$$\iint_R (f+g) \\, dA = \iint_R f \\, dA + \iint_R g \\, dA$$
2. $cf$ is also integrable on $R$ where $c \in \mathbb{R}$ is any constant and so

$$\iint_R cf \\, dA = c \iint_R f \\, dA$$

3. If $f(x,y) \leq g(x,y)$ for all $(x,y) \in \mathbb{R}$ then

$$\iint_R f(x,y) \\, dA \leq \iint_R g(x,y) \\, dA$$

4. $\vert f \vert$ is also integrable on $R$ and

$$\left\lvert \iint_R f \\, dA \right\rvert \leq \iint_R \vert f \vert  \\, dA$$



## Practical Double Integration

Let's actually do some of these double integrals now to get a feel for how we can approach and solve these problems.

### Example 1

Integrate 

$$\iint_A (2x - 3y) \\, dA$$
Where $A$ is the triangle with the vertices $(0,0), (2,1), (2,0)$.

First, $2x - 3y$ is a nice defined function we can integrate and a triangle is a nice defined area. We know that Fubini's theorem will apply here, so we can convert this into an iterated integral. 

If you sketch out this triangle, you'll see that it's a type 1 shape, that is, the sides remain the same and the top changes (in this instance, the base is also constant but that doesn't matter).

So, we know the $y$ term will need to be integrated first, and the limits are going to be functions of $x$. The base of the triangle is trivially $y=0$. The top of the triangle is given by the function $y = \frac{1}{2} x$. 

So the iterated integral is simply going to be

$$\int_0^2 \int_0^{\frac{1}{2} x} (2x - 3y) \\, dy \\, dx$$
The set up is done, the rest is just calculus - I'll run through the steps quickly:

$$\int_0^{\frac{1}{2} x} (2x -3y)dy = \left[ 2xy - \frac{3y^2}{2} \right]_0^{\frac{1}{2}x} = \frac{5}{8}x^2$$

$$\int_0^2 \frac{5}{8} x^2 dx = \left[ \frac{5}{8} \frac{x^3}{3} \right]_0^2 = \frac{5}{3}$$

### Example 2

In this example, we'll compute 

$$\iint_A 6y^2 \cos(x) \\, dA$$
Where $A$ is the area enclosed by the curve $y = \sin(x)$, the $x$ axis and the line $x = \frac{\pi}{2}$.

Once again, we can obviously apply Fubini's theorem here - the shape is nice and defined and so is the integral. We'll convert it into an iterated integral.

If you sketch out the shape, you'll once again see this is a type 1 shape, so we'll be integrating $y$ first, and need to find the limits. The $x$ axis bound means the lower limit is $y=0$, and the upper limit bound is given as $y = \sin(x)$.

So the iterated integral is 

$$\int_0^{\frac{\pi}{2}} \int_0^{\sin(x)} 6y^2 \cos(x) \\, dy \\, dx$$
The set up is done now - we really just need to solve this via calculus. I'll run through the steps below

$$\int^{\sin(x)}_0 6y^2 \cos(x) \\, dy = \left[\frac{6}{3} y^3 \cos(x)\right]_0^{\sin(x)} = 2 \sin^3(x) \cos(x)$$

$$\int^{\pi/2}_0 2 \sin^3(x) \cos(x) \\, dx$$ 
does need to be solved by substitution. We can use 

$$u = \sin(x), du = \cos(x) \\, dx$$

To make the integral

$$2\int u^3 du = 2\frac{u^4}{4} + c$$

And then evaluating the original limits 

$$2\left[ \frac{\sin^4 (x)}{4}\right]_0^{\frac{\pi}{2}} = \frac{1}{2}$$

## Closing Thoughts

You can now perform double integrals! Well done, this is an important step in the study of calculus and analysis. From here the logical places to go next are the study of vector calculus, including things like Green's Theorem, Stoke's Theorem and Divergence Theorem.

One niggle which might both some readers is that we didn't prove Fubini's theorem, which is the whole backbone of this thing. My response to this is the classic - it is well beyond the scope of this blog! Maybe one day I'll write a piece proving Fubini's theorem, but today is certainly not that day.

## References

Colley, S., J. (2012) *Vector Calculus (4th ed.)*. Pearson Education

Boas, M., L. (2005). *Mathematical Methods in the Physical Sciences (3rd ed.)*. Wiley

