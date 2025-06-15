
## Introduction

In a previous blog we discussed integration of the form $\int^b\_a f(x) dx$, and discussed how this was working out the area under the curve of the function $f(x)$ between the bounds $[a,b]$. In a certain sense, we can think of the integration previously as taking place over the line given by $(a,0)$ to $(b,0)$ - a line on the $x$-axis.

Now we want to think about performing an integral over any curve in this plane which means we need to introduce a new concept of the *line integral* which we will be exploring in this blog.

There are two kinds of line integrals - scalar line integrals and vector line integrals. You can probably guess what the difference between them is. We'll consider scalar line integrals first then vector line integrals second.

First some quick notation and ideas revision. Skip this if you already know them.

First, the norm which is denoted by $\lVert \mathbf{x}(t) \rVert$ which is nothing more than the magnitude of the vector. In other words if $\mathbf{x}(t) = (x\_1(t), x\_2(t), x\_3(t))$ then 

$$\lVert \mathbf{x} (t) \rVert = \sqrt{x\_1(t)^2 + x\_2(t)^2 + x\_3(t)^2}$$

The second concept we'll use a lot is parameterisation. The basic idea here is to turn some arbitrary path (which is certainly impossible or at least very difficult to integrate) into something much, much easier to integrate. So, we want to describe every point along the curve as the *image* of a parameter.

So a simple example is the unit circle - integrating over a circle directly is kinda tricky, but if we introduced some *parameter* (hence, the name) $t$ we can say $\mathbf{t}(t) = (\cos(t), \sin(t))$ which is literally trivial to integrate. We'll see more examples of doing this in practice.


## Scalar Line Integrals

Consider a smooth curve $\mathbf{x}$ in the plane and let $f$ be a function with a domain which includes $\mathbf{x}$. We can divide $C$ into small pieces. For each piece, we can choose $\mathbf{x}(t)$ in the piece and evaluate $f$ at $\mathbf{x}(t)$. We can multiply $f(\mathbf{x}(t))$ by $\Delta s$, the arc length of the piece, and sum the products $f(\mathbf{x}(t)) \Delta s$. If we allow the number of pieces to approach infinity, in other words, take the limit, we will have completed the line integration. 

To be more formal, let $\mathbf{x}: [a,b] \rightarrow \mathbb{R}^3$ be a smooth, continuous path. Let $f: X \subseteq \mathbb{R}^3 \rightarrow \mathbb{R}$ be a continuous function whose domain $X$ contains the image of $\mathbf{x}$ (such that $f(\mathbf{x}(t))$ is defined). Let

$$a = t\_0 < t\_1 < \cdots t\_k < \cdots < t\_n = b$$
be a partition of $[a,b]$. Let $t^\*\_k$ be some arbitrary point in the *k*th sub-interval $[t\_{k-1},t\_k]$ of the partition. We can consider the sum

$$\sum^n\_{k=1} f(\mathbf{x}(t^\*\_k)) \Delta s\_k$$
Where 
$$\Delta s\_k = \int^{t\_k}\_{t\_{k-1}} \lVert \mathbf{x}^\prime (t) \rVert dt$$
is the length of the *k*th segment of $\mathbf{x}$. 

Let's consider this path as representing an idealised wire, then $f(\mathbf{x}(t^\*\_k))$ represents the electrical charge density at $\mathbf{x}(t^\*\_k)$, and so the product $f(\mathbf{x}(t^\*\_k)) \Delta s\_k$ represents the charge contributed by this section of the curve. If we summed all of these, we would obtain the total charge on the wire, $C$.

$$C = \lim\_{\Delta s\_k \rightarrow 0} \sum^n\_{k=1} f(\mathbf{x}(t^\*\_k)) \Delta s\_k$$
$$C = \lim\_{\Delta t\_k \rightarrow 0} \sum^n\_{k=1} f(\mathbf{x}(t^\*\_k)) \Delta s\_k$$

There is some number $t^{\*\*}\_k$ in the interval $[t\_{k-1},t\_k]$ such that

$$\Delta s\_k = \int\_{t\_{k-1}}^{t\_k} \lVert \mathbf{x}^\prime \rVert dt = (t\_k - t\_{k-1}) \lVert \mathbf{x}^\prime (t^{\*\*}\_k) \rVert = \lVert \mathbf{x}^\prime (t^{\*\*}\_k) \rVert \Delta t\_k$$

Since $t^\*\_k$ is some arbitrary point in $[t\_{k-1},t\_k]$ we may set it equal to $t^{\*\*}\_k$. Therefore, by substituting for $\Delta s\_k$ in $C = \lim\_{\Delta t\_k \rightarrow 0} \sum^n\_{k=1} f(\mathbf{x}(t^\*\_k)) \Delta s\_k$ and setting $t^\*\_k$ to $t^{\*\*}\_k$ we have 

$$C = \lim\_{\Delta t\_k \rightarrow 0} \sum\_{k=1}^n f(\mathbf{x}(t^{\*\*}\_k)) \lVert \mathbf{x}^\prime (t^{\*\*}\_k) \rVert \Delta t\_k$$
$$C = \int\_a^b f(\mathbf{x}(t)) \lVert \mathbf{x}^\prime \rVert dt$$

The *scalar line integral* of $f$ along the path $\mathbf{x}$ is 

$$\int\_a^b f(\mathbf{x}(t)) \lVert \mathbf{x}^\prime (t) \rVert dt$$
Which we can denote as $\int\_{\mathbf{x}} f ds$


### Example 1

Let's fine the value of the line integral $\int\_C 2 ds$ where $C$ is the upper half of the unit circle.

The function of interest is $f(x,y) = 2$. Notice how this will give us a sheet along the curve $C$, which if flattened out would be a rectangle with width $\pi$ and height 2. 

We let $\mathbf{x}(t)$ be the parameterisation of $C$, then $f(\mathbf{x}(t\_i)) = 2$ for all $t\_i$ in the domain of $\mathbf{x}$. Therefore

$$\int\_C f ds = \lim\_{n \rightarrow \infty} \sum^n\_{i=1} f(\mathbf{x}(t^\*\_i)) \Delta s\_i$$
$$\int\_C f ds = \lim\_{n \rightarrow \infty} \sum^n\_{i=1} 2 \Delta s\_i$$
$$\int\_C f ds = 2 \lim\_{n \rightarrow \infty} \sum^n\_{i=1}  \Delta s\_i$$
Notice how $\lim\_{n \rightarrow \infty} \sum^n\_{i=1}  \Delta s\_i$ is actually just the length of $C$, or in other words

$$\int\_C f ds = 2 \pi$$

### Example 2

Let $\mathbf{x} : [0, 2\pi] \rightarrow \mathbb{R}^3$ be the helix $\mathbf{x}(t) = ( \cos(t), \sin(t), t)$ and let $f(x,y,z) = xy + z$. We can compute 

$$\int\_{\mathbf{x}} f ds = \int\_0^{2 \pi} f(\mathbf{x}(t)) \lVert \mathbf{x}^\prime (t) \rVert dt$$
So easily

$$\mathbf{x}^\prime(t) = (- \sin(t), \cos(t), 1)$$
$$\lVert \mathbf{x}^\prime (t) \rVert = \sqrt{\sin^2(t) + \cos^2(t) + 1} = \sqrt{2}$$
Recalling that $\sin^2(x) + \cos^2(x) = 1$. We also have

$$f(\mathbf{x}(t)) = \cos(t) \sin(t) + t = \frac{1}{2} \sin(2t) + t$$

Recalling the double angle formula: $\sin(2x) = 2 \sin(x) \cos(x)$. 

Putting it all together

$$\int\_{\mathbf{x}} f ds = \int\_0^{2 \pi} \left( \frac{1}{2} \sin(2t) + t \right) \sqrt{2} dt$$
$$\int\_{\mathbf{x}} f ds = \sqrt{2} \int\_0^{2 \pi} \left( \frac{1}{2} \sin(2t) + t \right) dt$$
$$\int\_{\mathbf{x}} f ds = \sqrt{2} \left[- \frac{1}{4} \cos(2t) + \frac{1}{2} t^2 \right]\_0^{2 \pi} = 2 \sqrt{2} \pi^2$$

### Example 3

Let $f(x,y) = y-x$ and let $\mathbf{x}: [0,3] \rightarrow \mathbb{R}^2$ be the path

$$\mathbf{x}(t) = \begin{cases}
(2t,t) & 0 \leq t \leq 1, \\
(t + 1, 5-4t) & 1 < t \leq 3.
\end{cases}$$
We call this *piecewise* and we can perform these as two independent segments we'll call $\mathbf{x}\_1$ and $\mathbf{x}\_2$. In other words

$$\int\_{\mathbf{x}} f ds = \int\_{\mathbf{x}\_1} f ds + \int\_{\mathbf{x\_2}} f ds$$
Where $\mathbf{x}\_1 (t) = (2t,t)$ for $0 \leq t \leq 1$ and $\mathbf{x}\_2 (t) = (t+1,5-4t)$ for $1 < t \leq 3$. You can easily show that

$$\lVert \mathbf{x}\_1^\prime (t) \rVert = \sqrt{5}$$
$$\lVert \mathbf{x}^\prime\_2 (t) \rVert = \sqrt{17}$$

 And so
$$\int\_{\mathbf{x}\_1} f ds = \int\_0^1 f(\mathbf{x}\_1(t)) \lVert \mathbf{x}\_1^\prime (t) \rVert dt$$
$$\int\_{\mathbf{x}\_1} f ds = \int\_0^1 (t-2t) \cdot \sqrt{5} dt$$
$$\int\_{\mathbf{x}\_1} f ds = \left[ - \frac{\sqrt{5}}{2} t^2 \right]^1\_0 = - \frac{\sqrt{5}}{2}$$

In the same way

$$\int\_{\mathbf{x}\_2} f ds = \int\_1^3 f(\mathbf{x}\_2(t)) \lVert \mathbf{x}^\prime\_2 (t) \rVert dt$$
$$\int\_{\mathbf{x}\_2} f ds = \int^3\_1 ((5-4t) - (t+1)) \sqrt{17} dt$$
$$\int\_{\mathbf{x}\_2} f ds = \sqrt{17} \left[ 4t - \frac{5}{4} t^2 \right]^3\_1 = -12 \sqrt{17}$$

And so 

$$\int\_{\mathbf{x}} f ds = -\frac{\sqrt{5}}{2} - 12 \sqrt{17}$$

## Vector Line Integrals

The basic idea behind the vector line integral is more or less the same as in the scalar line integral case. The only difference now is we introduce a vector field. 

Let $\mathbf{x}: [a,b] \rightarrow \mathbb{R}^n$ be a smooth continuous curve. Taking the vector field $\mathbf{F}$ we can say the *vector line integral* of $\mathbf{F}$ along $\mathbf{X}: [a,b] \rightarrow \mathbb{R}^n$, which we denote as $\int\_{\mathbf{x}} \mathbf{F} \cdot d \mathbf{s}$ is

$$\int\_{\mathbf{x}} \mathbf{F} \cdot d \mathbf{s} = \int\_a^b \mathbf{F} (\mathbf{x}(t)) \cdot \mathbf{x}^\prime (t) dt$$
### Example 1

A good intuition builder for a vector line integral is work. We know that work is given by

$$dW = \mathbf{F} \cdot d \mathbf{r}$$

Suppose an object moves along some path, where the force acting varies as it moves. We want to find the total work done on the object, which is to sum up the work done in each infinitesimally small section of the curve. 

For example, given $\mathbf{F} = xy \mathbf{i} - y^2 \mathbf{j}$ find the work done along the path $x = 2t^3, y = t^2$ from $(0,0)$ to $(2,1)$. We can combine those into a new vector like $\mathbf{x}(t) = 2t^3 \hat{\mathbf{i}} + t^2 \hat{\mathbf{j}}$. Easily $\mathbf{x}^\prime (t) = 6t^2 \hat{\mathbf{i}} + 2t \hat{\mathbf{j}}$.

We were given the start and end coordinates in $x,y$, but we need to integrate along $t$. We can use the fact that 

$$t = \sqrt[3]{\frac{x}{2}}$$
to see that $x=0, t=0; x=2, t=1$, which gives us our limits.

We also need to parameterise $\mathbf{F}$ in terms of $\mathbf{x}(t)$, so $\mathbf{F}(\mathbf{x}(t)) = 2t^5 \hat{\mathbf{i}} - t^4 \hat{\mathbf{j}}$

Finally then 

$$W = \int\_0^1 (2t^5 \hat{\mathbf{i}} - t^4 \hat{\mathbf{j}}) \cdot (6t^2 \hat{\mathbf{i}} + 2t \hat{\mathbf{j}}) dt $$
$$W = \int^1\_0 (12t^7 - 2t^5) dt$$
$$W = \left[ \frac{12}{8} t^8 - \frac{2}{6}t^6 \right]^1\_0 = \frac{7}{6}$$

### Example 2

Let $\mathbf{F}$ be the vector field given by $\mathbf{F} = x \hat{\mathbf{i}} + y \hat{\mathbf{j}} + z \hat{\mathbf{k}}$ (this vector field is obviously over $\mathbb{R}^3$). Let $\mathbf{x} : [0,1] \rightarrow \mathbb{R}^3$ be the path $\mathbf{x}(t) = (t, 3t^2, 2t^3)$. We can easily say $\mathbf{x}^\prime = (1, 6t, 6t^2)$ and so 

$$\int\_{\mathbf{x}} \mathbf{F} \cdot d \mathbf{s} = \int\_0^1 \mathbf{F}(\mathbf{x}(t)) \cdot \mathbf{x}^\prime(t) dt$$
$$\int\_{\mathbf{x}} \mathbf{F} \cdot d \mathbf{s} = \int\_0^1 (t \hat{\mathbf{i}} + 3t^2 \hat{\mathbf{j}} + 2t^3 \hat{\mathbf{k}}) \cdot (\hat{\mathbf{i}} + 6t \hat{\mathbf{j}} + 6t^2 \hat{\mathbf{k}}) dt $$
$$\int\_{\mathbf{x}} \mathbf{F} \cdot d \mathbf{s} = \int\_0^1 (t + 18t^3 + 12t^5) dt = \left[ \frac{1}{2}t^2 + \frac{9}{4}t^4 + 2t^6 \right]^1\_0 = 7$$

### Example 3

Find the integral $\int\_{\mathbf{x}} \mathbf{F} \cdot d \mathbf{s}$ where $\mathbf{x}$ is the semicircle parameterised by $\mathbf{x}(t) = (\cos(t), \sin(t))$ in $0 \leq t \leq \pi$ and $\mathbf{F} = -y \hat{\mathbf{i}} + x \hat{\mathbf{j}}$. We can find $\mathbf{F}(\mathbf{x}(t)) = -\sin(t) \hat{\mathbf{i}} + \cos(t) \hat{\mathbf{j}}$ and $\mathbf{x}^\prime(t) = -\sin(t) \hat{\mathbf{i}} + \cos(t) \hat{\mathbf{j}}$.

Thus

$$\int\_{\mathbf{x}} \mathbf{F} \cdot d \mathbf{s} = \int\_0^\pi (-\sin(t) \hat{\mathbf{i}} + \cos(t) \hat{\mathbf{j}}) \cdot (-\sin(t) \hat{\mathbf{i}} + \cos(t) \hat{\mathbf{j}}) dt$$

$$\int\_{\mathbf{x}} \mathbf{F} \cdot d \mathbf{s} = \int\_0^\pi \sin^2(t) + \cos^2 (t) dt$$
$$\int\_{\mathbf{x}} \mathbf{F} \cdot d \mathbf{s} =\int\_0^\pi 1 dt = \pi$$


## Summary

Scalar line integral:

$$\int\_a^b f(\mathbf{x}(t)) \lVert \mathbf{x}^\prime (t) \rVert dt$$

Vector line integral:


$$\int\_{\mathbf{x}} \mathbf{F} \cdot d \mathbf{s} = \int\_a^b \mathbf{F} (\mathbf{x}(t)) \cdot \mathbf{x}^\prime (t) dt$$


## References

Herman, E., Strang, G. (2015). *Calculus Volume 3*. OpenStax

Colley, S., J. (2012) *Vector Calculus (4th ed.)*. Pearson Education

Boas, M., L. (2005). *Mathematical Methods in the Physical Sciences (3rd ed.)*. Wiley
