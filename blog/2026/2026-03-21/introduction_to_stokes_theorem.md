This is my final blog on vector calculus for some time. In this one, we'll be covering Stoke's theorem, which has a few pre-requisites in line integrals found [here](/blog/2025-06-07/introduction-to-line-integrals) and double integrals found [here](/blog/2025-12-06/introduction-to-double-integrals), as well as Green's theorem found [here](/blog/2026-01-03/introduction-to-greens-theorem).

Stoke's theorem can be stated as: Let $S$ be a bounded, piecewise smooth,orientated surface in $\mathbb{R}^3$. Suppose that $\partial S$ is made of finitely many piecewise $C^1$, simple, closed curves each of which is orientated consistently with $S$. Let $\mathbf{F}$ be a vector field of class $C^1$ whose domain includes $S$. Then

$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = \iint_{S} (\nabla \times \mathbf{F}) \cdot \mathbf{n} d S$$

You will often see the slightly alternative form on the RHS

$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = \iint_{S} (\nabla \times \mathbf{F}) \cdot d \mathbf{S}$$

Since very simply $\mathbf{n} d S = d \mathbf{S}$. We often flick between them as convenient.

Stoke's theorem relates an integral over an open surface to the line integral around the curve bounding the surface. Imagine taking some surface and chopping it up into loads of pieces and consider a piece near the middle. Consider one edge of that piece. Its curl will rotate you in one direction, but the adjacent edge from the neighbour will rotate in the opposite direction. When you sum up everything, just the contributions from the edge actually remain.

It's important that the surface is *two sided*. You can actually easily construct surfaces with only one side: a Moebius strip is a particularly famous example. But, with only one side, we can't construct a normal vector. In mathematical terms, we call this "simple" - the surface can't pass through itself.

Much like in Divergence Theorem, the point of Stoke's Theorem is to simplify computations. In Divergence Theorem, we want to compute the flux and we get a choice of which way is easier given the problem. Stoke's is the same: we want to compute this circulation and we get two choices for how we can do it. Depending on how the problem is set up, maybe the LHS or RHS is easier to compute.

## Example 1

Given $\mathbf{F} = 4y \mathbf{\hat{i}} + x \mathbf{\hat{j}} + 2 z \mathbf{\hat{k}}$, let's prove both sides of Stoke's theorem are the same over the hemisphere $x^2 + y^2 + z^2 = a^2, z \geq 0$

The curl can easily be found to be

$$\nabla \times \mathbf{F} = -3 \mathbf{\hat{k}}$$

Finding the normal as

$$\mathbf{n} = \frac{\mathbf{r}}{\vert \mathbf{r} \vert} = \frac{\mathbf{r}}{a} = \frac{x \mathbf{\hat{i}} + y \mathbf{\hat{j}} + z \mathbf{\hat{k}}}{a}$$

And so

$$(\nabla \times \mathbf{F}) \cdot \mathbf{n} = -3 \mathbf{\hat{k}} \cdot \frac{\mathbf{r}}{a} = -3 \frac{z}{a}$$

In spherical coordinates, we have

$$z = r \cos(\theta)$$
$$d S = r^2 \sin(\theta) d \theta d \phi$$

For this surface $r=a$, so the integral is

$$\int_{\phi = 0}^{2 \pi} \int_{\theta = 0}^{\pi / 2} = -3a^2 \int_0^{2 \pi} d \phi \int_{0}^{\pi / 2} \sin(\theta) \cos(\theta) d \theta = -3a^2 \cdot 2 \pi \cdot 1/2 = -3 \pi a^2$$

For the other side of Stoke's, we want to compute the line integral $\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s}$ around the circle at $z=0$

$$\mathbf{r}(t) = a \cos(t) \mathbf{\hat{i}} + a \sin(t) \mathbf{\hat{j}}$$
$$d \mathbf{r} = (-a \sin(t) \mathbf{\hat{i}} + a \sin(t) \mathbf{\hat{j}}) dt$$
And so

$$\mathbf{F}(\mathbf{r}) = 4a \sin(t) \mathbf{\hat{i}} + a \cos(t) \mathbf{\hat{j}}$$

$$\mathbf{F}(\mathbf{r}) \cdot \frac{d \mathbf{r}}{dt} = -4a^2 \sin^2(t) + a^2 \cos^2(t)$$
So we have the parameterised circle, and obviously this needs to go from 0 to $2 \pi$
$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = \int_0^{2 \pi} -4a^2 \sin^2(t) + a^2 \cos^2(t) dt$$
$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = -4a^2 \int_0^{2 \pi} \sin^2 (t) dt + a^2 \int^{2 \pi}_0 \cos^2(t) dt$$

Both $\int_0^{2 \pi} \sin^2 (t) dt$ and $\int^{2 \pi}_0 \cos^2(t) dt$ happen to be pretty standard integrals, and while I won't go through both of them here, they're both equal to $\pi$. So

$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = -4 a^2 \pi + a^2 \pi = -3 a^2 \pi$$

Which checks out!

## Example 2

Stoke's theorem is particularly useful in electromagnetics. For example, Ampère's law says

$$\oint_C \mathbf{H} \cdot d \mathbf{r} = I$$

Where $\mathbf{H} = \mathbf{B} / \mu_0$, $\mathbf{B}$ is the magnetic field, $\mu_0$ is some constant (called the *permeability of free space*), $C$ is some closed curve and $I$ is the current crossing any surface bounded by $C$.

Consider a long straight wire carrying a current $I$. At a distance $r$ from the wire, $\mathbf{H}$ is tangent to the circle of radius $r$ in the plane perpendicular to the wire. The symmetry of this system means that $\vert \mathbf{H} \vert$ is the same at all points along the circle. So

$$\oint_C \mathbf{H} \cdot d \mathbf{r} = \int_0^{2 \pi} \vert \mathbf{H} \vert r d \theta = \vert \mathbf{H} \vert r \cdot 2 \pi = I$$

or 

$$\vert \mathbf{H} \vert = \frac{I}{2 \pi r}$$

We can now think about $\mathbf{J}$, which is the current density, then $\mathbf{J} \cdot \mathbf{n} d S$ is the current across a surface element $dS$. Furthermore, $\iint_S \mathbf{J} \cdot \mathbf{n} dS$ over any surface $S$ bounded by $C$ is the total current across $C$. By Ampère's law

$$\oint_C \mathbf{H} \cdot d \mathbf{r} = \iint_S \mathbf{J} \cdot \mathbf{n} d S$$

and by Stoke's 

$$\oint_C \mathbf{H} \cdot d \mathbf{r} = \iint_S (\nabla \times \mathbf{H}) \cdot \mathbf{n} dS$$
Therefore

$$\iint_S (\nabla \times \mathbf{H}) \cdot \mathbf{n} d S = \iint_S \mathbf{J} \cdot \mathbf{n} dS$$

Since this holds for all $S$ then we have

$$\nabla \times \mathbf{H} = \mathbf{J}$$
which is one of the Maxwell equations.

## Example 3

Let $\sigma$ be the paraboloid defined by $z = 9 -x^2 - y^2$ defined over the disc in the $xy$-plane of radius 3. So we can essentially think of $\sigma$ as the butterfly net from earlier, but the arc of the net is pointing up. $\partial \sigma$ consists of the circle

$$C = \{ (x,y,z) | x^2 + y^2 =9, z=0 \}$$
We'll have the normal vector pointing out of the surface. Suppose the vector field is given by

$$\mathbf{F} = (2z-y) \mathbf{\hat{i}} + (x+y) \mathbf{\hat{j}} + (3x - 2y) \mathbf{\hat{k}}$$
We calculate the curl as

$$\nabla \times \mathbf{F} = (-2 -1) \mathbf{\hat{i}} + (2-3) \mathbf{\hat{j}} + (1 + 1) \mathbf{\hat{k}} = -3i -j + 2k$$

We also need a normal vector, which can be obtained by taking the gradient.

$$\nabla \mathbf{F} = \mathbf{N} =  2x \mathbf{\hat{i}} + 2y \mathbf{\hat{j}} + \mathbf{\hat{k}}$$

We can replace $\mathbf{n} d S$ by $\mathbf{N} dx dy$

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \iint_{D} (-3 \mathbf{\hat{i}} - \mathbf{\hat{j}} + 2 \mathbf{\hat{k}}) \cdot (2x \mathbf{\hat{i}} + 2y \mathbf{\hat{j}} + \mathbf{\hat{k}}) dx dy$$
Where $D = \{ (x,y) | x^2 + y^2 \leq 9 \}$

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \iint_D (-6x -2y + 2) dx dy$$

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \iint_D -6x dx dy - \iint_D 2y dx dy + \iint_D 2 dx dy$$

Since $D$ has symmetry and $-6x$ and $2y$ are off functions, the first two double integrals are 0. The last integral simply gives twice the area of $D$. Thus

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = 2 \cdot \pi 3^2 = 18 \pi$$
Let's now solve it the other way, to prove they are equal.

We can parameterise the boundary of $S$ as

$$\begin{cases} 
x = 3 \cos(t) \\
y = 3 \sin(t) & 0 \leq t \leq 2\pi \\
z = 0
\end{cases}$$

Then

$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = 
\int_0^{2 \pi} \mathbf{F} (\mathbf{X}(t)) \cdot \mathbf{X}^\prime (t) dt$$

$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = 
\int^{2 \pi}_0 (0 - 3 \sin(t), 3 \cos(t) + 0, 9 \cos(t) - 6 \sin(t)) \cdot (-3 \sin(t), 3 \cos(t), 0) dt$$

$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = 
\int^{2 \pi}_0 (9 \sin^2 (t) + 9 \cos^2 (t)) dt = \int^{2\pi}_0 9 dt = 18 \pi$$

So we see they are the same!

## Example 4

Consider the surface $S$ defined by the equation $z = e^{-(x^2 + y^2)}$ for $z \geq 1 / e$. In other words, $S$ is the graph of $f(x,y) = e^{-(x^2 + y^2)}$ defined over $D = \{(x,y) | x^2 + y^2 \leq 1\}$. Consider the field

$$\mathbf{F} = (e^{y+z} - 2y) \mathbf{\hat{i}} + (x e^{y+z} +y) \mathbf{\hat{j}} + e^{x+y} \mathbf{\hat{k}}$$

Suppose we take the upward pointing normal vector

$$\mathbf{N} = 2x e^{-(x^2 + y^2)} \mathbf{\hat{i}} + 2y e^{-(x^2 + y^2)} \mathbf{\hat{j}} + \mathbf{\hat{k}}$$
Then, because 

$$\nabla \times \mathbf{F} = (e^{x+y} - x e^{y+z}) \mathbf{\hat{i}} + (e^{y+z} - e^{x+y}) \mathbf{\hat{j}} + 2 \mathbf{\hat{k}}$$
And so 

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = 
\iint_D (
2x e^{-(x^2 + y^2)} (e^{x+y} - xe^{y+z}) +
2 y e^{-(x^2 + y^2)}(e^{y+z} - e^{x+y}) + 2
) dx dy$$
To be completely honest, I have absolutely no idea whatsoever how to solve this integral. I have a sense such an integral simply can not be computed analytically, and if it can, only with extraordinary difficulty.

Let's try the other way around.

$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = 
\int^{2 \pi}_0 (e^{\sin(t) + 1/e} - 2 \sin(t), \cos(t) e^{\sin(t) + 1/e} + \sin(t), e^{\cos(t) + \sin(t)}) \cdot
(-\sin(t), \cos(t), 0) dt
$$

$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = \int^{2 \pi}_0 (2 \sin^2(t) - \sin(t) e^{\sin(t) + 1/e} + \cos^2(t) e^{\sin(t) + 1/e} + \cos(t) \sin(t)) dt$$

Once again, if such an integral can be computed I certainly do not know how to do it.

Are we dead in the water? Not quite. The beauty in Stoke's theorem is we can actually use *any* orientable piecewise smooth surface whose boundary is the same as the original boundary. In toher words, we can choose $S^\prime$ and perform the calculation on that surface assuming that $\partial S = \partial S^\prime$. 

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = \oint_{\partial S^\prime} \mathbf{F} \cdot d \mathbf{s} = \iint_{S^\prime} \nabla \times \mathbf{F} \cdot d \mathbf{S}$$

In other words, as long as we are careful, we can literally just ignore the surface given in a problem and just make up any surface we like (so long as the boundaries and normals all line up of course!!). In other words, the surface "behind" the loop is basically irrelevant to Stoke's theorem.

To use this fact to our advantage in this case, we can notice that $\nabla \times \mathbf{F}$ has a very simple $\mathbf{\hat{k}}$ component, so let $S^\prime$ be the unit disc at $z = 1/e$ given by

$$S^\prime = \{(x,y,z) | x^2 + y^2 \leq 1, z = 1/e\}$$

If we orient $S^\prime$ by the unit normal vector $\mathbf{n} = + \mathbf{\hat{k}}$ we have

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \iint_{S^\prime} \nabla \times \mathbf{F} \cdot d \mathbf{S}$$
$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \iint_{S^\prime} (\nabla \times \mathbf{F} \cdot \mathbf{n}) dS$$
$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \iint_{S^\prime} 2 dS = 2 \pi$$

### Relation to Green's Theorem

Green's theorem is actually a special case of Stoke's theorem. I have a blog on Green's theorem here, if you need a refresher.

As we know, Green's theorem is in 2D, but Stoke's theorem is in 3D. In other words, Green's theorem is a special case of Stoke's where $\mathbf{F} = (L,M,0)$ i.e. no $z$ component. Let's begin from the LHS of Green's theorem

$$\oint_{\partial S} (L \ dx + M \ dy) = \oint_{\partial S} (L, M, 0) \cdot (dx, dy, dz) = \oint_{\partial S} \mathbf{F} \cdot d \mathbf{r}$$

We use a slightly alternative representation of Stoke's theorem here

$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{r} = \iint_{S} (\nabla \times \mathbf{F}) \cdot \mathbf{n} \ dS$$

We define the unit normal to have a positive $z$ component to match the positive orientation definition.

$$\nabla \times \mathbf{F} \cdot \mathbf{n} = \left[
\left( \frac{\partial 0}{\partial y} - \frac{\partial M}{\partial z} \right) \hat{\mathbf{i}} + 
\left( \frac{\partial L}{\partial z} - \frac{\partial 0}{\partial x} \right) \hat{\mathbf{j}} + 
\left( \frac{\partial M}{\partial x} - \frac{\partial L}{\partial y} \right) \hat{\mathbf{k}} + 
\right] \hat{\mathbf{k}}$$
$$\nabla \times \mathbf{F} \cdot \mathbf{n} = \left( \frac{\partial M}{\partial x} - \frac{\partial L}{\partial y} \right)$$

Which instantly gives rise to the RHS of Green's

$$\iint_S \nabla \times \mathbf{F} \cdot \mathbf{n} \ dS = \iint_D \left( \frac{\partial M}{\partial x} - \frac{\partial L}{\partial y} \right) \ dA$$


### Proof

We can begin by establishing a special case of the theorem where the vector field $\mathbf{F} = M(x,y,z) \hat{\mathbf{i}}$ (i.e. an $\hat{\mathbf{i}}$ component only) where the surface $S$ is the graph of $z = f(x,y)$ where $f$ is of class $C^1$ on a domain $D$ in the plane that is a type 1 elementary region, in other words, $D$ satisfies 

$$\{ (x,y) | \gamma(x) \leq y \leq \delta(x), a \leq x \leq b \}$$

Where $\gamma$ and $\delta$ are continuous functions. We assume that $S$ is oriented by the upward-pointing unit normal.

We begin by evaluating $\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s}$. The boundary $\partial S$ consists of (at most) four smooth pieces parametrised as follows

$$C_1 : \begin{cases}
x = t \\
y = \gamma(t) \\
z = f(t, \gamma(t))
\end{cases} \quad a \leq t \leq b$$
$$C_2 : \begin{cases}
x = b \\
y = t \\
z = f(b,t)
\end{cases} \quad \gamma(b) \leq t \leq \delta(b) $$
$$C_3 : \begin{cases}
x = t \\
y = \delta(t) \\
z = f(t, \delta(t))
\end{cases} \quad a \leq t \leq b$$
$$C_4 : \begin{cases} 
x = a \\
y = t \\
z = f(a,t)
\end{cases} \quad \gamma(a) \leq t \leq \delta(a) $$

Therefore

$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = \int_{C_1} \mathbf{F} \cdot d \mathbf{s} + \int_{C_2} \mathbf{F} \cdot d \mathbf{s} - \int_{C_3} \mathbf{F} \cdot d \mathbf{s} - \int_{C_4} \mathbf{F} \cdot d \mathbf{s}$$

Let's zoom in on the integral over $C_1$. Since $\mathbf{F}$ has only an $\hat{\mathbf{i}}$ component, then

$$\int_{C_1} \mathbf{F} \cdot d \mathbf{s} = \int_{C_1} M \ dx + 0 \ dy + 0 \ dz = \int_a^b M(t, \ \gamma(t), \ f(t, \ \gamma(t) \ )) \ dt$$

By analogy for $C_3$

$$\int_{C_3} \mathbf{F} \cdot d \mathbf{s} = \int_a^b M(t, \ \delta(t), \ f(t, \ \delta(t) \ )) \ dt $$

For $C_2$ $x$ is held constant

$$\int_{C_2} \mathbf{F} \cdot d \mathbf{s} = \int_{C_2} M \ dx = 0$$

and also for $C_4$

$$\int_{C_4} \mathbf{F} \cdot d \mathbf{s} = 0$$

And so 

$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = \int_a^b M(t, \ \gamma(t), \ f(t, \ \gamma(t) \ )) \ dt - \int_a^b M(t, \ \delta(t), \ f(t, \ \delta(t) \ )) \ dt$$

We also change the variable of integration

$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = \int_a^b [M(x, \ \gamma(x), \ f(x, \ \gamma(x) \ )) - M(x, \ \delta(x), \ f(x, \ \delta(x) \ ))] dx$$

Now compare this with the surface integral $\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S}$. For $\mathbf{F} = M(x,y,z) \hat{\mathbf{i}}$ we have $\nabla \times \mathbf{F} = M_z \hat{\mathbf{j}} - M_y \hat{\mathbf{k}}$ so 

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \iint_D (M_z \hat{\mathbf{j}} - M_y \hat{\mathbf{k}}) \cdot (-f_x \hat{\mathbf{i}} - f_y \hat{\mathbf{j}} + \hat{\mathbf{k}}) \ dx \ dy$$

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \int_a^b \int_{\gamma(x)}^{\delta(x)} \left( - \frac{\partial M}{\partial z} \frac{\partial f}{\partial y} - \frac{\partial M}{\partial y} \right) \ dy \ dx$$

The chain rule shows us that

$$\frac{\partial}{\partial y} (M(x, y, f(x,y))) = \frac{\partial M}{\partial y} + \frac{\partial M}{\partial z} \frac{\partial f}{\partial y}$$
By the fundamental theorem of calculus we have

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \int_a^b \int_{\gamma(x)}^{\delta(x)} - \frac{\partial}{\partial y} (M(x, y, f(x,y))) \ dy \ dx$$


$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \int_a^b [-M(x,y,f(x,y))]^{y = \delta(x)}_{y = \gamma(x)} dx$$

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \int_a^b [M(x, \ \gamma(x), \ f(x, \ \gamma(x) \ )) - M(x, \ \delta(x), \ f(x, \ \delta(x) \ ))] \ dx$$

Which matches what we expect.

Let's extend the proof. We can keep $\mathbf{F} = M \hat{\mathbf{i}}$, noting that the argument so far works equally well for surfaces of the form $y = f(x,z)$ - just exchange the roles of $y$ and $z$ so far.

It is not difficult to see that if $S$ is a portion of the plane where $x = c$ (i.e. $x$ is constant) then Stoke's theorem for $\mathbf{F} = M \hat{\mathbf{i}}$ holds for this case too. 

For such a plane, $\mathbf{n} = \pm \hat{\mathbf{i}}$ (depending on orientation) and $\nabla \times \mathbf{F} = M_z \hat{\mathbf{j}} - M_y \hat{\mathbf{k}}$ and so 

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \iint_S (\nabla \times \mathbf{F} \cdot \mathbf{n}) dS = \iint_S 0 dS = 0$$

Also, since $\mathbf{F}$ has only an $\hat{\mathbf{i}}$ component, then $\mathbf{F}$ is always parallel to $\mathbf{n}$ and so perpendicular to any tangent vector to $S$, including vectors tangent to any boundary curves of $S$. Therefore, 

$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = 0$$

Now let us suppose that $S = S_1 \cup S_2$, where $S_1$ and $S_2$ are each one of the graph of $z = f(x,y)$, the graph of $y = f(x,z)$ or a portion of the plane $x=c$. Let's assume $S_1$ and $S_2$ are fused together on part of their boundary (you could imagine two bubbles being pushed together to form a 3D $\infty$ shape, for example). The surfaces $S_1$ and $S_2$ have orientations which match with $S$. If $C$ denotes the common part of $\partial S_1$ and $\partial S_2$, then we can write $\partial S_1 = C_1 \cup C$ and $\partial S_2 = C_2 \cup C$. I'll say that $C$ denotes the curve oriented so as to agree with the orientation of $\partial S_1$, which means that $\partial S_2$ is oriented the opposite way (you could do it the other way around, it does not matter, so long as you are consistent).

Stoke's theorem with $\mathbf{F} = M \hat{\mathbf{i}}$ holds on both $S_1$ and $S_2$, on $S_1$ we have

$$\iint_{S_1} \nabla \times \mathbf{F} \cdot d \mathbf{S} = \oint_{\partial S_1} \mathbf{F} \cdot d \mathbf{s} = \int_{C_1} \mathbf{F} \cdot d \mathbf{s} + \int_C \mathbf{F} \cdot d \mathbf{s}$$

and on $S_2$

$$\iint_{S_2} \nabla \times \mathbf{F} \cdot d \mathbf{S} = \oint_{\partial S_2} \mathbf{F} \cdot d \mathbf{s} = \int_{C_2} \mathbf{F} \cdot d \mathbf{s} + \int_C \mathbf{F} \cdot d \mathbf{s}$$

Now we can consider $S = S_1 \cup S_2$ and note that $C$ is not a part of $\partial S$, we see that

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \iint_{S_1} \nabla \times \mathbf{F} \cdot d \mathbf{S} + \iint_{S_2} \nabla \times \mathbf{F} \cdot d \mathbf{S}$$

And so

$$\int_{C_1} \mathbf{F} \cdot d \mathbf{s} + \int_{C_2} \mathbf{F} \cdot d \mathbf{s} = \oint_{\partial S} \mathbf{F} \cdot d \mathbf{s}$$

This provides a mostly complete proof of Stoke's theorem. This works for when $S$ can be described as a finite union of $S_1 \cup S_2 \cup \cdots \cup S_n$ of the special surfaces just described. In practice, most shapes (especially shapes you'll ever encounter in physical problems) will satisfy this condition. However, there are some convoluted shapes which don't fulfil this condition. A proof for those cases is well beyond the scope of this blog.

Finally, we just need to repeat this process for the cases where $\mathbf{F} = N(x,y,z) \hat{\mathbf{j}}$ and $\mathbf{F} = P(x,y,z) \hat{\mathbf{k}}$. Curl is an additive process so 

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \iint_S \nabla \times (M \hat{\mathbf{i}} + N \hat{\mathbf{j}} + P \hat{\mathbf{k}}) \cdot d \mathbf{S}$$
$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \iint_S \nabla \times (M \hat{\mathbf{i}}) \cdot d \mathbf{S} + \iint_S \nabla \times (N \hat{\mathbf{j}}) \cdot d \mathbf{S} + \iint_S \nabla \times (P \hat{\mathbf{k}}) \cdot d \mathbf{S} $$

We can see that 

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} = \oint_{\partial S} M \hat{\mathbf{i}} \cdot d \mathbf{s} + \oint_{\partial S} N \hat{\mathbf{j}} \cdot d \mathbf{s} + \oint_{\partial S} P \hat{\mathbf{k}} \cdot d \mathbf{s}$$

$$\iint_S \nabla \times \mathbf{F} \cdot d \mathbf{S} \oint_{\partial S} (M \hat{\mathbf{i}} + N \hat{\mathbf{j}} + P \hat{\mathbf{k}}) \cdot d \mathbf{s} = \oint_{\partial S} \mathbf{F} \cdot d \mathbf{s}$$

As desired.

### Conclusion 

If you've been following along with my blogs on vector calculus, you now have everything you need for a basic use of vector calculus, and most of what you need for a typical undergraduate course. As a reminder, Stoke's theorem is 

$$\oint_{\partial S} \mathbf{F} \cdot d \mathbf{s} = \iint_{S} (\nabla \times \mathbf{F}) \cdot \mathbf{n} d S$$

## References

Boas, M., L. (2005). *Mathematical Methods in the Physical Sciences (3rd ed.)*. Wiley

Colley, S., J. (2012) *Vector Calculus (4th ed.)*. Pearson Education

Riley, K., F. (1974). *Mathematical Methods for the Physical Sciences: An Informal Treatment for Students of Physics and Engineering*. Cambridge University Press

Marsden, J., E., Tromba, A. (2012) *Vector Calculus (6th ed.)*. W. H. Freeman and Company.