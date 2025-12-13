
## Introduction

I've already covered [double integrals in a previous blog](/blog/2025-12-06/introduction-to-double-integrals). Assuming that material has been understood, the triple integral is actually not a huge leap forward.

In the double integral blog, we began by imagining wanting to compute the mass of a 2D shape, given its mass density function. Let's just say the shape is the unit square. We can imagine the mass density function above the unit square, and the mass of the square would then be the volume under the curve. 

We thought about slicing up the square into a series of curves, and integrating each. We also thought about doing it with Riemann sums. Eventually we were able to use Fubini's theorem to convert the double integral into an iterated integral which we could solve fairly easily, once the bounds had been set.

In triple integration, we're essentially making the same set of arguments, and the general structure or idea is the same. We can slice up the shape into smaller pieces we can integrate, take the limits for the integration. Or we can start from Riemann sums. Or, we can again apply Fubini's theorem, since it generalises to three dimensions. I'm sure it'll come as no surprise that you can once again transform a triple integral into an iterated integral of three integration steps.

One thing I do want to draw attention to before beginning is that triple integration can be used in two broad ways.

First, suppose I have some 3D shape $D$. What does the triple integral 

$$\iiint_D dD$$

represent? In this instance where there is no integrand (or, rather, the integrand is 1), this would actually compute the volume of the same. 

If we set the integrand however to some function, let's take for example, a mass density function like so 

$$\iiint_D \rho(x,y,z) \, dD$$

this would give us the mass of the shape. One thing to think about is how physical representations do start to break down now. If we take an analogy to the double integration blog, and consider that 2D shape: in this case, we actually can't physically represent the volume under the curve. Since the shape is already 3D, the density function would extend into the fourth dimension. In other words, for a 3D shape, the mass is the "volume" under the 4th dimension of the graph. Unfortunately we can't easily represent that graphically, but thankfully the mathematics works out exactly the same.

## Triple Integrals via Riemann Sums

Just to hammer home the idea, we can begin by thinking about integrating over a box with Riemann sums. Let $B$ be a closed box in $\mathbb{R}^3$ with faces parallel to coordinate planes, in other words

$$B = \{ (x,y,z) \in \mathbb{R}^3 | a \leq x \leq b, c \leq y \leq d, p \leq z \leq q \}$$

We can also write

$$B = [a,b] \times [c,d] \times [p,q]$$
A partition of $B$ of order $n$ consists of three collections of partition points that break up $B$ into a union of $n^3$ subboxes. That is, for $i,j,k=0,...,n$ we can introduce the collections $\{x_i\}, \{y_j\}, \{z_k\}$ such that

$$a = x_0 < x_1 < \cdots x_{i-1} < x_i < \cdots < x_n = b$$
$$c = y_0 < y_1 < \cdots y_{i-1} < y_i < \cdots < y_n = d$$
$$p = z_0 < z_1 < \cdots z_{i-1} < z_i < \cdots < z_n = q$$
In addition, for $i,j,k = 1,...,n$ let

$$\Delta x_i = x_i - x_{i-1}, \quad \Delta y_j = y_j - y_{j-1}, \quad \Delta z_k = z_k - z_{k-1}$$


Let $f$ be any function defined on $B = [a,b] \times [c,d] \times [p,q]$. Partition $B$ in some way. Let $\mathbf{c}_{ijk}$ be any point in the subbox

$$B_{ijk} = [x_{i-1}, x_i] \times [y_{j-1}, y_j] \times [z_{k-1}, z_k], \quad (i,j,k = 1,...,n)$$

Then the quantity 

$$S = \sum^n\_{i,j,k=1} f(\mathbf{c}\_{ijk}) \Delta V\_{ijk}$$

Where $\Delta V_{ijk} = \Delta x_i \Delta y_j \Delta z_k$ is the volume of $B_{ijk}$, which is the Riemann Sum of $f$ on $B$ corresponding to the partition.

As in the double integrals blog, we won't actually work out a Riemann sum by hand. Even if we split each dimension into just 5 pieces that gives us 125 boxes! Save the thought for another day though, as computers love crunching huge numbers.

The Riemann sum is essentially a weighted sum of volumes of subboxes of $B$, the weighting given by the value of the function $f$ at particular test points in each subbox.

 The triple integral of $f$ on $B$ which is denoted by

$$\iiint_B f \, dV \quad \text{or} \quad \iiint_B f(x,y,z) \, dV \quad \text{or} \quad \iiint_B f(x,y,z) \, dx \, dy \, dz$$

is the limit of the Riemann sum $S$ as the dimensions $\Delta x_i, \Delta y_j, \Delta z_k$ of the subboxes $B_{ijk}$ all approach zero. In other words

$$\iiint_B f \ dV = \lim_{\Delta x_i, \Delta y_j, \Delta z_k \rightarrow 0} \sum^n_{i,j,k = 0} f(\mathbf{c}_{ijk}) \ \Delta x_i \ \Delta y_j \ \Delta z_k$$

Once more, provided that the function is integrable, we can apply Fubinis Theorem to convert the triple integral into a triple iterated integral to actually solve these without doing a Riemann sum.

## Triple Integrals Over Arbitrary Shapes

Now we've been able to mathematically shed ourselves of the Riemann sum, we want to be able to integrate over arbitrary shapes, and not just cuboids. As with double integration, the mathematics can balloon to outrageous degrees if we truly integrate over absolutely any shape so we'll focus on shapes where the projection in one axis forms a elementary region.

We say that $W$ is an elementary region in space if it can be described as a subset of $\mathbb{R}^3$ of one of the following four types

**Type 1**

$$W = \{ (x,y,z) | \phi(x,y) \leq z \leq \psi(x,y), \gamma(x) \leq y \leq \delta(x), a \leq x \leq b \}$$
or

$$W = \{ (x,y,z) | \phi(x,y) \leq z \leq \psi(x,y), \alpha(y) \leq \beta(y), c \leq y \leq d \}$$

**Type 2**

$$W = \{ (x,y,z) | \alpha(y,z) \leq x \leq \beta(y,z) | \gamma(z) \leq y \leq \delta(z), p \leq z \leq q \}$$
or

$$W = \{ (x,y,z) | \alpha(y,z) \leq x \leq \beta(y,z), \phi(y) \leq z \psi(y), x \leq y \leq d  \} $$

**Type 3**

$$W = \{ (x,y,z) | \gamma(x,z) \leq y \leq \delta(x,z), \alpha(z) \leq x \leq \beta(z), p \leq z \leq q \}$$

or

$$W = \{ (x,y,z) | \gamma(x,z) \leq y \delta(x,z), \phi(x) \leq z \leq \psi(x), a \leq x \leq b \}$$

**Type 4**

All of the above

To visualise these types, just think about 2D elementary regions. Type 1 is a 3D shape such that its projection onto the $xy$ plane is a 2D elementary region (in other words, in the $xy$ plane its shadow has two parallel lines). Type 2 is the same but for the $yz$ plane, and type 3 for the $xz$ plane.

Suppose $W$ is an elementary region in $\mathbb{R}^3$ and $f$ is a continuous function on $W$. Then we can define the extension (just like in double integration!) by

$$f^{ext} (x,y) = \begin{cases} 
f(x,y) & \text{if } (x,y) \in W,\\
0 & \text{if } (x,y) \notin W.
\end{cases}$$

Under assumptions that $W$ is an elementary region and $f$ is continuous on $W$, we define the triple integral

$$\iiint_W f dV = \iiint_B f^{ext} dV$$

The point of this is we will be integrating some function in the box $B$. It could be, say, a mass density function like $x+y+z$. That function is valid across $\mathbb{R}^3$, but we only care about the bit of it inside $B$. Really this is a very rigorous way to say we are ignoring everything outside of $B$, which of course we are, we want to integrate $B$!

## Examples

### Example 1

Let 

$$B = [-2, 3] \times [0,1] \times [0,5], \quad f(x,y,z) = x^2 e^y + xyz$$

We can see that $f$ is continuous and so satisfies Fubini's theorem

Therefore, it is simply

$$\iiint_B (x^2 e^y + xyz) \ dV = \int^3_{-2} \int_0^1 \int^5_0 (x^2 e^y + xyz) \ dz \ dy \ dx$$
Some algebra here you can solve in your own time
$$\iiint_B (x^2 e^y + xyz) \ dV = \frac{175}{3}(e-1) + \frac{125}{8}$$


### Example 2

Let $W$ denote the tetrahedron with vertices at $(0,0,0), (1,0,0), (0,1,0), (0,0,1)$. Suppose the mass density inside is given by $f(x,y,z) = 1+xy$. Find the total mass, which is given by the expression

$$M = \iiint_W (1+xy) dV$$

This shape is elementary - there is only one slanted face whose coordinates are given by $x+y+z=1$, so we can integrate with respect to that face first by integrating with respect to $z$ and holding $x$ and $y$ constant

$$M = \iint_s \left( \int_0^{1-x-y} (1+xy) dz \right) ds$$
Where I use $s$ to denote the shadow - we can return to that in a moment

$$M = \iint_s (1+xy)(1-x-y) ds$$
$$M = \iint_s (1-x-y+xy-x^2y - xy^2) ds$$

The shadow in the $z=0$ plane is just a right angle triangle, with a slanted line given by $x+y=1$. We now have a familiar double integration

$$M = \int_0^1 \int_0^{1-x} (1-x-y+xy-x^2y - xy^2) dy dx$$
$$M = \int^1_0 ((1-x) - x(1-x) - 1/2 (1-x)^2 + 1/2 x(1-x)^2 - 1/2 x^2 (1-x)^2 - 1/3 x (1-x)^3 ) dx$$

Unfortunately triple integrals have a tendency to balloon terms.

$$M = \int_0^1 (1/2 - 5/6 x + 1/2 x^3 - 1/6 x^4) dx = 7/40$$

### Example 3

Find the volume between $z = 2x+3y+6$ and $z=2x+7y+8$ over the square with vertices $(0,0), (1,0), (0,1), (1,1)$ in the $xy$ plain.

Setting up the iterated integral in this case is pretty straightforward.

$$\iiint_v dV = \int_0^1 \int_0^1 \int_{2x+3y+6}^{2x+7y+8} \ dz \ dy \ dx$$

$$\int_{2x+3y+6}^{2x+7y+8} dz = 2x +7y +8 - 2x -3y - 6 = 4y + 2$$

$$\int_0^1 4y + 2 dy = \left[ \frac{4y^2}{2} + 2y \right]^1_0 4$$

$$\int_0^1 4 dx = [4x]^1_0 = 4$$

### Example 4

Find the volume between $z = 2x+3y+6$ and $z=2x+7y+8$ over the triangle with vertices $(0,0), (3,0), (2,1)$ in the $xy$ plain.

Here the bounds are a little more complex. Now the triangle is formed of two sloped lines, which will actually become the $x$ limits. The two sloped lines for the triangle, as well as them rearranged for $x$ are

$$y = 1/2 x; \quad x = 2y$$
$$y = -x + 3; \quad x = 3 - y$$
Which you can find easily from $y=mx+c$ and graphing out the vertices.

So that gives the $x$ limits; the $z$ limits are given in the question, and the $y$ limits are trivially between 0 and 1.

So the triple integral is 

$$\iint_V dV = \int_0^1 \int_{2y}^{3-y} \int_{2x+3y+6}^{2x+7y+8} dx \ dy \ dz$$

From here it's just rote calculus, which I'll whizz through

$$\int_{2x+3y+6}^{2x+7y+8} dz = 2x +7y +8 - 2x -3y - 6 = 4y + 2$$

$$\int_{2y}^{3-y} 4y + 2 dx = [4yx + 2x]^{3-y}_{2y} = -12y^2 + 6y + 6$$

$$\int^1_0 -12y^2 + 6y + 6 dy = \left[ -\frac{12}{3} y^3 + \frac{6}{2} y^2 + 6y \right]^1_0 = 5$$
## Closing Thoughts

Congratulations! You can now perform triple integrals. If you've been following along with my blog you should also be able to do double integrals and line integrals by this point. This means you have all of the necessary mathematical tools to engage with vector calculus. From here, you should be able to study Green's Theorem, Stoke's Theorem and Divergence Theorem, as well as further topics in vector calculus!

## References

Colley, S., J. (2012) *Vector Calculus (4th ed.)*. Pearson Education

Boas, M., L. (2005). *Mathematical Methods in the Physical Sciences (3rd ed.)*. Wiley