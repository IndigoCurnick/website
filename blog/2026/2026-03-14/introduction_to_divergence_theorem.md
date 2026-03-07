I've been publishing blogs on vector calculus for some time now. This is really one of the crowning jewels of vector calculus, and it builds upon lots of previous work. So for pre-requisites you'll need surface integrals [here](/blog/2026-03-06/introduction-to-surface-integrals), triple integrals [here](/blog/2025-12-06/introduction-to-triple-integrals) and also Green's theorem [here](/blog/2026-01-03/introduction-to-greens-theorem).

This now deals with Divergence theorem, sometimes known as Gauss's theorem. This is a very important theorem in physics especially.

Divergence theorem relates the total divergence within some shape to the flux passing out of its surface. The physical intuition for this is to imagine a bucket with a hose and a hole at the bottom. The water flowing out of the bucket from the top is the water flowing in via the hose minus the water flowing out via the drain. 

Formally,


$$ \oiint_{\partial D} \mathbf{F} \cdot d \mathbf{S} = \iiint_D \nabla \cdot \mathbf{F} dV$$

Where $D$ is a bounded solid region in $\mathbb{R}^3$ (think a solid 3d shape like a cube, sphere, cylinder, other complicated shapes...) whose boundary $\partial D$ consists of finitely many piecewise smooth, closed orientable surfaces, each of which is orientated by unit normals that point out of $D$. $\mathbf{F}$ is a vector field of class $C^1$ (it is smooth and has smooth derivatives i.e. we can compute its divergence!) whose domain contains $D$. 

Let's break that down again just to get more intuition. The LHS surface integral is like the top of that bucket - we're looking at the flux, the amount of "stuff", coming out of the surface. The RHS volume integral is like the hoses and drains inside - we're summing all of the sources and sinks within the solid.

And that's kind of the crux of divergence theorem. Let's look at some examples.

## Example 1

Let's take a cylinder $D$ of radius $a$ and height $b$, aligned with the $z$ axis such that the bottom of the cylinder is at $z=0$ (the top is thus at $z=b$). Let's also say there's a field $\mathbf{F}$ given by $x \mathbf{\hat{i}} + y \mathbf{\hat{j}} + z \mathbf{\hat{k}}$.

We have three surfaces to tackle here. The bottom ($S_1$) and top ($S_2$) of the cylinder, which are both discs, and then the middle portion ($S_3$) which is like a toilet paper tube. 

$$\oiint_{\partial D} \mathbf{F} \cdot d \mathbf{S} = 
\oiint_{\partial D} \mathbf{F} \cdot \mathbf{S} + 
\oiint_{\partial D} \mathbf{F} \cdot \mathbf{S} + 
\oiint_{\partial D} \mathbf{F} \cdot \mathbf{S}$$

We'll set up this integral slowly piece by piece (also to preserve horizontal space).

For the bottom disc:
$$\iint_{S_1} \mathbf{F} \cdot \mathbf{S} = \iint_{S_1} (x \mathbf{\hat{i}} + y \mathbf{\hat{j}} + z \mathbf{\hat{k}}) \cdot -\mathbf{\hat{k}} d S = \iint_{S_1} -z dS = 0$$
$-\mathbf{\hat{k}}$ because the normal needs to point *out* from the shape, and as the bottom piece that means *down*. It's 0 in the end because it is at $z=0$

The top disc is basically a mirror image:

$$\iint_{S_2} \mathbf{F} \cdot \mathbf{S} = \iint_{S_2} (x \mathbf{\hat{i}} + y \mathbf{\hat{j}} + z \mathbf{\hat{k}}) \cdot \mathbf{\hat{k}} d S = \iint_{S_1} z dS = \iint_{S_1} b dS = b \pi a^2$$
The final result is $b$ times the area of the disc, $\pi a^2$.

And finally the middle tube:

$$\iint_{S_3} \mathbf{F} \cdot d \mathbf{S} = \iint_{S_2} (x \mathbf{\hat{i}} + y \mathbf{\hat{j}} + z \mathbf{\hat{k}}) \cdot \left( \frac{x \mathbf{\hat{i}} + y \mathbf{\hat{j}}}{a} \right) dS = \iint_{S_3} \frac{x^2 + y^2}{a} dS = \iint_{S_3} a dS = a(2\pi ab) = 2 \pi a^2 b$$

First, $x^2 + y^2 = a^2$ because of basic Pythag - $a$ is the radius of the cylinder so imagine a right angle triangle going to the edge of the cylinder. The $\left( \frac{x \mathbf{\hat{i}} + y \mathbf{\hat{j}}}{a} \right)$ part is the unit vector pointing radially outward at all points. If we were in a spherical system we could use $\mathbf{\hat{r}}$ - it's the same thing! We can compute it here by taking the gradient of $x^2 + y^2 = a^2$ and normalising. $\nabla (x^2 + y^2) = 2x \mathbf{\hat{i}} + 2y \mathbf{\hat{j}}$. Normalise that: $\frac{x \mathbf{\hat{i}} + y \mathbf{\hat{j}}}{\sqrt{x^2 + y^2}} = \frac{x \mathbf{\hat{i}} + y \mathbf{\hat{j}}}{a}$. Finally the area of this surface is $2 \pi ab$. 

So in total

$$\iint_{\partial D} \mathbf{F} \cdot d \mathbf{S} = \iint_{S_1} \mathbf{F} \cdot \mathbf{S} + \iint_{S_1} \mathbf{F} \cdot \mathbf{S} + \iint_{S_1} \mathbf{F} \cdot \mathbf{S} = b \pi a^2 + 2 \pi a^2 b = 3 \pi b a^2$$

Okay, now for the right hand side:

$$\nabla \cdot \mathbf{F} = \frac{\partial x}{\partial x} + \frac{\partial y}{\partial y} + \frac{\partial z}{\partial z} = 3$$

$$\iiint_D \nabla \cdot \mathbf{F} dV = \iiint_D 3 dV = 3 \pi a^2 b$$

We can see they are the same! In this case it was *much* easier to compute the volume integral.

## Example 2

Let's compute the flux of $\mathbf{F} = x^2 \mathbf{\hat{i}} + y^2 \mathbf{\hat{j}} + z^2 \mathbf{\hat{k}}$ of the surface of the unit cube bounded by $0 \leq x,y,z \leq 1$.

If we were to do the surface, we'd need to do 

- $\mathbf{F} \cdot d \mathbf{S}$ on each of the six faces
- Figure out the normal correctly on each face
- Set up 6 integrals
- Manually compute all of that (without making a mistake!)

Instead let's just do the volume 

$\nabla \cdot \mathbf{F} = \frac{\partial x^2}{\partial x} + \frac{\partial y^2}{\partial y} + \frac{\partial z^2}{\partial z} = 2x + 2y + 2z$

Integrate over the cube

$$\iiint_D (2x + 2y + 2z) dV = 2 \iiint_D (x + y + z) dV$$

This splits cleanly into 

$$2 \left( 
\int^1_0 x dx \int^1_0 dy \int^1_0 dz +
\int^1_0 y dy \int^1_0 dx \int^1_0 dz +
\int^1_0 z dz \int^1_0 dx \int^1_0 dy
\right)$$

Now I know this looks like a lot but this is trivial. A little practice with triple integrals and you can probably skip this step - for instance $\int^1_0 x dx$ is obviously just $\frac{1}{2}$ a and $\int^1_0 dx$ is obviously just 1. So this integral is just $\frac{3}{2}$.

So finally 

$$2 \left( \frac{1}{2} + \frac{1}{2} + \frac{1}{2} \right) = 2 \cdot \frac{3}{2} = 3$$

## Example 3

Let's now do one where the volume is nasty but the surface is easier. We'll compute the flux of 

$$\mathbf{F} = \frac{\overrightarrow{r}}{r^3}$$
over the surface of a sphere radius $a$ centred at the origin.

Now the problem presents itself instantly for using the volume:

$$\nabla \cdot \left( \frac{\overrightarrow{r}}{r^3} \right)$$
But what is the divergence at $r=0$? Obviously this is undefined and we'd need to get into the Dirac delta function and all sorts of nonsense to compute it. 

However, let's do it the other way.

The surface element is $dA = a^2 \sin(\theta) \\ d \theta \\ d \phi$ and $\mathbf{F} = \frac{\overrightarrow{r}}{r^3} = \frac{\mathbf{\hat{r}}}{a^2}$ because $\mathbf{\hat{r}} = \frac{\overrightarrow{r}}{\lvert \overrightarrow{r} \rvert}$. And we are evaluating at $r=a$. So

$$\mathbf{F} \cdot d \mathbf{A} = 
\frac{1}{a^2} \mathbf{\hat{r}} \cdot (a^2 \sin(\theta) \\ d \theta \\ d \phi) = 
\sin(\theta) \\ d \theta \\ d \phi$$

Therefore

$$\iint_D \mathbf{F} \cdot d \mathbf{A} = \int^{2 \pi}_0 \int^\pi_0 \sin(\theta) \\ d \theta \\ d \phi = 4 \pi$$

Which is pretty easy!

## Relation to Green's Theorem

Green's theorem is actually a special case of Divergence theorem. As we know, Green's theorem is in 2D, but Divergence theorem is in 3D.

Consider the unit normal $\mathbf{n}$. In Green's theorem $d \mathbf{r} = (dx, dy)$ is a vector pointing tangential along the curve, and the curve $C$ is the positively oriented curve alongside the boundary, an outward normal would be e.g. $(dy, -dx)$. The length of the vector is $\sqrt{dx^2 + dy^2} = ds$, so

$$(dy, -dx) = \mathbf{n} ds$$

Starting from the LHS of Green's 

$$\oint_{\partial S} (L \ dx + M \ dy) = \oint_{\partial S} (M, -L) \cdot (dy, -dx) = \oint_{\partial S} (M, -L) \cdot \mathbf{n} ds$$
Applying Divergence theorem now with $\mathbf{F} = (M, -L)$ we get

$$\oint_{\partial S} (M, -L) \cdot \mathbf{n} ds = \iint_{D} (\nabla \cdot (M, -L)) dA = \iint_D \left( \frac{\partial M}{\partial x} - \frac{\partial L}{\partial y} \right) dA$$

### Proof

We first prove a special case of Divergence theorem where $\mathbf{F} = P \hat{\mathbf{k}}$ (where $P(x,y,z)$ is of class $C^1$ on a domain that includes the solid region $D$) and where $D$ is an elementary region of type 1, in other words, $D$ satisfies 

$$\{ (x,y) | \gamma(x) \leq y \leq \delta(x), a \leq x \leq b \}$$

We let $D$ be a shape comprised of three regions: a bottom region ($S_1$), a top region ($S_2$) and a lateral region in between ($S_3$). We assume $S_1$ is given by the equation $z = \phi(x,y)$ where $\phi$ is also class $C^1$ and we let $S_2$ be given by the equation $z = \psi(x,y)$ where $\psi$ is again class $C^1$. 

Orienting $\partial D = S_1 \cup S_2 \cup S_3$ with outward pointing normal vectors, we have

$$\oiint_{\partial D} \mathbf{F} \cdot d \mathbf{S} = \iint_{S_1} \mathbf{F} \cdot d \mathbf{S} + \iint_{S_2} \mathbf{F} \cdot d \mathbf{S} + \iint_{S_3} \mathbf{F} \cdot d \mathbf{S}$$

The orientation normal to $S_1$ should be downward-pointing, hence parallel to $\phi_x \hat{\mathbf{i}} + \psi_y \hat{\mathbf{j}} - \hat{\mathbf{k}}$, which is the opposite of the normal vector which is obtained from the more standard parametrisation of $S_1$. Therefore using 

$$\iint_{\mathbf{X}} \mathbf{F} \cdot d \mathbf{S} = \iint_D \mathbf{F} (x, y, f(x,y)) \cdot (-f_x \hat{\mathbf{i}} - f_y \hat{\mathbf{j}} + \hat{\mathbf{k}}) \ dx \ dy$$

We have

$$\iint_{S_1} \mathbf{F} \cdot d \mathbf{S} = \iint_R P(x,y, \phi(x,y)) \hat{\mathbf{k}} \cdot (\phi_x \hat{\mathbf{i}} + \phi_y \hat{\mathbf{j}} - \hat{\mathbf{k}}) \ dx \ dy$$

$$\iint_{S_1} \mathbf{F} \cdot d \mathbf{S} = - \iint_R P(x,y, \phi(x,y)) \ dx \ dy$$

Similarly, the orientation normal to $S_2$ should be upward-pointing and so

$$\iint_{S_2} \mathbf{F} \cdot d \mathbf{S} = \iint_R P(x,y, \psi(x,y)) \hat{\mathbf{k}} \cdot (-\psi_x \hat{\mathbf{i}} - \psi_y \hat{\mathbf{j}} + \hat{\mathbf{k}}) \ dx \ dy$$

$$\iint_{S_2} \mathbf{F} \cdot d \mathbf{S} = \iint_R P(x,y, \psi(x,y)) \ dx \ dy$$

The lateral surface, if non-zero, is a cylinder over a curve in the $xy$-plane. Hence, $S_3$ is defined by one or more equations of the form $g(x,y) = c$, but either way, it has no $\hat{\mathbf{k}}$ component. Thus

$$\iint_{S_3} \mathbf{F} \cdot d \mathbf{S} = \iint_{S_3} (P \hat{\mathbf{k}} \cdot \mathbf{n}_3) dS = 0$$
Together we find that

$$\oiint_{\partial D} \mathbf{F} \cdot d \mathbf{S} = \iint_R [P(x,y,\psi(x,y)) - P(x,y,\phi(x,y))] \ dx \ dy$$

If $\mathbf{F} = P \hat{\mathbf{k}}$ then $\nabla \cdot \mathbf{F} = \partial P / \partial z$, and so by the fundamental theorem of calculus

$$\iiint_D \nabla \cdot \mathbf{F} \ dV = \iint_R \int_{\phi(x,y)}^{\psi(x,y)} \frac{\partial P}{\partial z} \ dz \ dx \ dy$$

$$\iiint_D \nabla \cdot \mathbf{F} \ dV = \iint_R [P(x,y,\psi(x,y)) - P(x,y,\phi(x,y))] \ dx \ dy$$

And so Divergence theorem holds for this special case.

We can repeat the above by symmetry for elementary regions of type 2, and for $\mathbf{F} = N \hat{\mathbf{j}}$ and $D$ of type 3. If $D$ is an elementary region of type 4 then

$$\oiint_{\partial D} \mathbf{F} \cdot d \mathbf{S} = \oiint_{\partial D} (M \hat{\mathbf{i}} + N \hat{\mathbf{j}} + P \hat{\mathbf{k}}) \cdot d \mathbf{S}$$

$$\oiint_{\partial D} \mathbf{F} \cdot d \mathbf{S} = \oiint_{\partial D} M \hat{\mathbf{i}} \cdot d \mathbf{S} + \oiint_{\partial D} N \hat{\mathbf{j}} \cdot d \mathbf{S} + \oiint_{\partial D} P \hat{\mathbf{k}} \cdot d \mathbf{S}$$
$$\oiint_{\partial D} \mathbf{F} \cdot d \mathbf{S} = \iiint_D \nabla \cdot M \hat{\mathbf{i}} dV + \iiint_D \nabla \cdot N \hat{\mathbf{j}} dV + \iiint_D \nabla \cdot P \hat{\mathbf{k}} dV $$
$$\oiint_{\partial D} \mathbf{F} \cdot d \mathbf{S} = \iiint_D \nabla \cdot \mathbf{F} dV$$


Now suppose that $D = D_1 \cup D_2$, where $D_1$ and $D_2$ are both type 4 elementary regions and that $D_1$ and $D_2$ coincide at a part of their boundaries. We will denote the common boundary of $\partial D_1$ and $\partial D_2$ as $S$. Then $\partial D_1 = S_1 \cup S$ and $\partial D_2 = S_2 \cup S$. We orient $\partial D_1$ and $\partial D_2$ with outward normals so as to satisfy Divergence theorem, $S$ will then be oriented one way as part of $\partial D_1$ and the opposite way for $\partial D_2$. I will take $S$ to be oriented to agree with $\partial D_1$ (arbitrary choice, you could do it the other way around).

Applying Divergence theorem, we get 

$$\iiint_{D_1} \nabla \cdot \mathbf{F} dV = \oiint_{\partial D_1} \mathbf{F} \cdot d \mathbf{S} = 
\iint_{S_1} \mathbf{F} \cdot d \mathbf{S} + \iint_S \mathbf{F} \cdot d \mathbf{S}$$


$$\iiint_{D_2} \nabla \cdot \mathbf{F} dV = 
\oiint_{\partial D_2} \mathbf{F} \cdot d \mathbf{S} = 
\iint_{S_2} \mathbf{F} \cdot d \mathbf{S} - \iint_S \mathbf{F} \cdot d \mathbf{S}$$

By combining these we find that

$$\iiint_D \nabla \cdot \mathbf{F} dV = \iiint_{D_1} \nabla \cdot \mathbf{F} dV + \iiint_{D_2} \nabla \cdot \mathbf{F} dV$$
$$\iiint_D \nabla \cdot \mathbf{F} dV = \iint_{S_1} \mathbf{F} \cdot d \mathbf{S} + \iint_{S_2} \mathbf{F} \cdot d \mathbf{S}$$
$$\iiint_D \nabla \cdot \mathbf{F} dV = \oiint_{\partial D} \mathbf{F} \cdot d \mathbf{S}$$

The above proof can then be extended into regions which are comprised of an arbitrary finite number of type 4 elementary regions. For practical purposes, especially in the physical sciences, this is most shapes you will ever encounter. However, some convoluted shapes do not satisfy such a condition, and a proof for them is beyond the scope of this blog.

## Conclusion

Congratulations! You can now understand and use Divergence Theorem! This is one of the crowning jewels of vector calculus, and usually one of the final topics taught in undergraduate vector calculus. So well done on conquering so many difficult topics! As a reminder, the theorem is

$$ \oiint_{\partial D} \mathbf{F} \cdot d \mathbf{S} = \iiint_D \nabla \cdot \mathbf{F} dV$$


## References


Boas, M., L. (2005). *Mathematical Methods in the Physical Sciences (3rd ed.)*. Wiley

Colley, S., J. (2012) *Vector Calculus (4th ed.)*. Pearson Education

Riley, K., F. (1974). *Mathematical Methods for the Physical Sciences: An Informal Treatment for Students of Physics and Engineering*. Cambridge University Press

Marsden, J., E., Tromba, A. (2012) *Vector Calculus (6th ed.)*. W. H. Freeman and Company.