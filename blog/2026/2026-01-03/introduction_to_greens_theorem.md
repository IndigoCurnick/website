This blog continues my series on vector calculus. In order to understand this blog, you will need to be familiar with both [double integrals](/blog/2025-12-06/introduction-to-double-integrals) and [line integrals](/blog/2025-06-07/introduction-to-line-integrals).

## Introduction

So far in my series on vector calculus we've looked at double integrals and line integrals. As you might be aware if you've tried your hand at solving some problems in these topics yourself, they can become a little difficult to do. Some especially have an explosion of terms in the integral which can be a pain to deal with.

This is where Green's theorem comes in: it gives us an opportunity to, in some cases, substitute a line integral for a double integral. Depending on the physical situation, one might be easier than the other.

Green's theorem is as follows. Let $D$ be a closed bounded region in $\mathbb{R}^2$ whose boundary $C = \partial D$ consists of finitely many simple, closed, piecewise $C^1$ curves. Orient the curves of $C$ such that  $D$ is on the left as one traverses $D$. Let $\mathbf{F}(x,y) = M(x,y) \hat{\mathbf{i}} + N (x,y) \hat{\mathbf{j}}$ be a vector field of class $C^1$ across $D$. Then

$$\oint M dx + N dy = \iint_D \left( \frac{\partial N}{\partial x} - \frac{\partial M}{\partial y}  \right) \ dx \ dy$$


## Examples of Green's Theorem

### Example 1

Verify Green's theorem on $\mathbf{F} = 2x \hat{\mathbf{i}} - 3y \hat{\mathbf{j}}$ around the square with vertices $(0,2), (2,0), (-2,0), (0, -2)$.

Let's begin with the double integral.

$$M dx + N dy = \frac{\partial 2x}{\partial y} -3 \frac{\partial y}{\partial x} = 0$$
So there's no need to go any further - we now know already that the answer has to be 0.

The line integral is  not so simple in this case sadly - we just have to do four different integrals, which we'll call the following:

- A - $(0,2) \rightarrow (-2,0)$
- B - $(2,0) \rightarrow (0,2)$
- C - $(2,0) \rightarrow (0,-2)$
- D - $(0,-2) \rightarrow (-2,0)$

A:

$$a = (-2,0), \quad b = (-2,0)$$

We can then parameterise the line 

$$\gamma(t) = a + t(b-a) = (-2t, 2-2t)$$
$$\gamma(t) = -2t \hat{\mathbf{i}} + (2-2t) \hat{\mathbf{j}} = -2t \hat{\mathbf{i}} + 2 \hat{\mathbf{j}} - 2t \hat{\mathbf{j}}$$
$$\gamma^\prime (t) = -2 \hat{\mathbf{i}} - 2 \hat{\mathbf{j}}$$
$$F(\gamma(t)) = -4t \hat{\mathbf{i}} - 3(2-2t) \hat{\mathbf{j}}$$

Now we have the parameterisation we can go ahead and now perform the integration. Let's recall from a previous blog that 

$$\int_{\mathbf{x}} \mathbf{F} \cdot d \mathbf{s} = \int_a^b \mathbf{F} (\mathbf{x}(t)) \cdot \mathbf{x}^\prime (t) dt$$

Keep this in mind as we will be using it A LOT in this blog (I'm using $\gamma$ today)

$$\int_0^1 F(\gamma(t)) \cdot \gamma^\prime(t) dt = \int^1_0 (-4t \hat{\mathbf{i}} - 3(2-2t) \hat{\mathbf{j}}) \cdot (-2 \hat{\mathbf{i}} - 2 \hat{\mathbf{j}}) dt$$
$$\int_0^1 F(\gamma(t)) \cdot \gamma^\prime(t) dt = \int_0^1 (-8t +12 -12t) dt$$
$$\int_0^1 F(\gamma(t)) \cdot \gamma^\prime(t) dt = \left[ \frac{-8t^2}{2} + 12t - \frac{12t^2}{2} \right]^1_0 = 2$$

B:

$$a = (2,0), \quad b = (0,2)$$
$$\gamma(t) = a + t(b-a) = (2-2t,2t)$$
$$\gamma(t) = (2-2t) \hat{\mathbf{i}} + 2t \hat{\mathbf{j}} = 2 \hat{\mathbf{i}} - 2t \hat{\mathbf{i}} + 2t \hat{\mathbf{j}}$$
$$\gamma^\prime(t) = -2 \hat{\mathbf{i}} + 2 \hat{\mathbf{j}}$$

$$F(\gamma(t)) = 2(2-2t) \hat{\mathbf{i}} - 6t \hat{\mathbf{j}}$$

$$ \int_0^1 F(\gamma(t)) \cdot \gamma^\prime(t) dt = \int_0^1 (4 \hat{\mathbf{i}} - 4t \hat{\mathbf{i}} - 6t \hat{\mathbf{j}}) \cdot (2 \hat{\mathbf{i}} + 2 \hat{\mathbf{j}}) dt$$

$$ \int_0^1 F(\gamma(t)) \cdot \gamma^\prime(t) dt = \int_0^1 (8 -8t -12t) dt$$
$$ \int_0^1 F(\gamma(t)) \cdot \gamma^\prime(t) dt = \left[ 8t - \frac{8t^2}{2} - \frac{12t^2}{2} \right]^1_0 = -2$$

C: 

$$a = (2,0), \quad b = (0,-2)$$
$$\gamma(t) = a+t(b-a) = (2-2t, -2t)$$
$$\gamma(t) = (2-2t) \hat{\mathbf{i}} - 2t \hat{\mathbf{j}} = 2 \hat{\mathbf{i}} - 2t \hat{\mathbf{i}} - 2t \hat{\mathbf{j}}$$
$$\gamma^\prime(t) = -2 \hat{\mathbf{i}} - 2 \hat{\mathbf{j}}$$
$$F(\gamma(t)) = 2(2-2t) \hat{\mathbf{i}} + 6t \hat{\mathbf{j}}$$
$$\int_0^1 F(\gamma(t)) \cdot \gamma^\prime(t) dt = \int_0^1 (2(2-2t) \hat{\mathbf{i}} + 6t \hat{\mathbf{j}}) \cdot (-2 \hat{\mathbf{i}} - 2 \hat{\mathbf{j}}) dt$$
$$\int_0^1 F(\gamma(t)) \cdot \gamma^\prime(t) dt = \int^1_0 (-8 + 8t - 12t) dt$$
$$\int_0^1 F(\gamma(t)) \cdot \gamma^\prime(t) dt = \left[ -8t + \frac{8t^2}{2} - \frac{12t^2}{2} \right]^1_0 = -10$$

D:

$$a = (0,-2),  \quad b = (-2,0)$$
$$\gamma(t) = a+t(b-a) = (-2t, -2+2t)$$
$$\gamma(t) = -2t \hat{\mathbf{i}} + (-2+2t) \hat{\mathbf{j}} = -2t \hat{\mathbf{i}} - 2 \hat{\mathbf{j}} + 2t \hat{\mathbf{j}}$$
$$\gamma^\prime(t) = - 2\hat{\mathbf{i}} + 2 \hat{\mathbf{j}}$$
$$F(\gamma(t)) = -4t \hat{\mathbf{i}} - 3(-2 + 2t) \hat{\mathbf{j}}$$
$$\int_0^1 F(\gamma(t)) \cdot \gamma^\prime(t) dt = \int_0^1 (-4t \hat{\mathbf{i}} - 3(-2 + 2t) \hat{\mathbf{j}}) \cdot (- 2\hat{\mathbf{i}} + 2 \hat{\mathbf{j}}) dt$$
$$\int_0^1 F(\gamma(t)) \cdot \gamma^\prime(t) dt = \int^1_0 (8t + 12 - 12t) dt$$
$$\int_0^1 F(\gamma(t)) \cdot \gamma^\prime(t) dt = \left[ \frac{8t^2}{2} + 12t - \frac{12t^2}{2} \right]^1_0 = 10$$

So in review 

$$A + B + C + D = 2 - 2 - 10 + 10 = 0$$

So we have verified Green's theorem here.

### Example 2

Verify Green's theorem on $\mathbf{F} = -x^2 y \hat{\mathbf{i}} + x y^2 \hat{\mathbf{j}}$ on the disc $x^2 + y^2 \leq 4$.

Let's begin with the double integral

$$\frac{\partial N}{\partial x} - \frac{\partial M}{\partial y} = y^2 + x^2$$

We need to do the double integral on a circle, so switching to polar coordinates is a much better option here.

$$x^2 + y^2 = r^2, \quad dA = dx \\, dy = r \\, dr \\, d \theta$$

$$\iint_D y^2 + x^2 \\, dx \\, dy = \int^{2 \pi}_0 \int_0^2 r^3 \\, dr \\, d\theta$$
$$\int^{2 \pi}_0 \int_0^2 r^3 \\, dr \\, d\theta = \int_0^{2 \pi} 4 \\, d \theta = [4 \theta]^{2 \pi}_0 = 8 \pi$$

Now onto the line integral. Since we have a circle, we'll want to immediately convert to polar coordinates like so

$$x = 2 \cos(t), \quad y = 2 \sin(t), \quad t \in [0, 2 \pi]$$

And we can proceed as usual

$$\gamma(t) = 2 \cos(t) \hat{\mathbf{i}} + 2 \sin(t) \hat{\mathbf{j}}$$
$$\gamma^\prime(t) = - 2 \sin(t) \hat{\mathbf{i}} + 2 \cos(t) \hat{\mathbf{j}}$$
$$F(\gamma(t)) = -8 \cos^2(t) \sin(t) \hat{\mathbf{i}} + 8 \cos(t) \sin^2(t) \hat{\mathbf{j}}$$
$$\int_0^{2 \pi} F(\gamma(t)) \cdot \gamma^\prime(t) dt = (-8 \cos^2(t) \sin(t) \hat{\mathbf{i}} + 8 \cos(t) \sin^2(t) \hat{\mathbf{j}}) \cdot (- 2 \sin(t) \hat{\mathbf{i}} + 2 \cos(t) \hat{\mathbf{j}}) dt$$
$$\int_0^{2 \pi} F(\gamma(t)) \cdot \gamma^\prime(t) dt = \int_0^{2 \pi} 32 \cos^2(t) \sin^2(t) dt$$

Solving this integral is a bit tricky, but we can exploit the trig identities. Since $\sin(a) \cos(b) = \frac{1}{2} (\sin(a + b) + \sin(a - b))$ when $a = b$ we get $\sin(a) \cos(a) = \frac{1}{2} \sin(2a)$

$$\int_0^{2 \pi} F(\gamma(t)) \cdot \gamma^\prime(t) dt = 32 \int_0^{2 \pi} \frac{1}{4} \sin^2(2t)$$
We can solve by substitution $u = 2t, du = 2 dt$

$$\int_0^{2 \pi} \sin^2 (2t) = \frac{1}{2} \int \sin^2(u) du$$
And then we can use another trig rule here. Since $\sin(a) \sin(b) = \frac{1}{2} (\cos(b-a) - \cos(b+a))$ when $a = b$ then $\sin^2(a) = \frac{1}{2} (1- \cos(2a))$

$$\frac{1}{2} \int \sin^2(u) du = \frac{1}{4} \int (1 - \cos(2u)) du$$

Once again we need to do another substitution 

$v = 2u, dv = 2 du$

$$\int \cos(2u) du = \int \frac{1}{2} \cos(v) dv = -\frac{1}{2} \sin(2u)$$

Don't forget the linear parts, and make sure you grab all the factors if you're following along!

In short

$$\frac{1}{2} \int \sin^2(u) du = \frac{u}{4} - \frac{1}{8} \sin(2u) = \frac{t}{2} - \frac{1}{8} \sin(4t) + C$$
And evaluating our limits gives us

$$\left[ \frac{t}{2} - \frac{1}{8} \sin(4t) \right]^{2 \pi}_0 = 8 \pi$$

And so we have completed the verification!


### Example 3

Verify Green's theorem on $\mathbf{F} = (x^2 - y) \hat{\mathbf{i}} + (x + y^2) \hat{\mathbf{j}}$ on the rectangle bounded by $x=0, x = 2, y = 0, y = 1$.

Let's begin with the double integral. This is a very simple geometry - an axis aligned square means we don't need to much work at all to coerce the integral into something we can solve very quickly!

$$\iint_D \left( \frac{\partial N}{\partial x} - \frac{\partial M}{\partial y}  \right) \ dx \ dy = \iint_D 1 -- 1 \\, dD = \iint_D 2 \\, dD$$

$$\iint_D 2 \\, dD = \int_0^2 \int_0^1 = 2 \\, dy \\, dx = 4$$

Now onto the line integrals. One tip I have for you is to not overwork yourself. It might be tempting to get into a flow and just parameterise these again like we did in the previous problems, but if the geometry is axis aligned i.e. only one variable changes along the length of the line, then this is pointless and only adds more work!

In this problem we need to do four integrals for the following lines 

- A: $(2,1) \rightarrow (0,1)$
- B: $(0,1) \rightarrow (0,0)$
- C: $(0,0) \rightarrow (2,0)$
- D: $(2,0) \rightarrow (2,1)$

A:

$$y = 1, dy = 0$$

$$\oint M dx + N dy = \int_2^0 (x^2 -1) dx = - \frac{2}{3}$$

B: 

$$x = 0, dx = 0$$
$$\oint Mdx + N dy = \int_1^0 y^2 dy = -\frac{1}{3}$$

C:

$$y = 0, dy = 0$$
$$\oint Mdx + Ndy = \int_0^2 x^2 dx = \frac{8}{3}$$

D:

$$x=2, dx = 0$$
$$\oint M dx + N dy = \int^1_0 2 + y^2 dy = 2 + \frac{1}{3}$$

And in total we have

$$-\frac{2}{3} - \frac{1}{3} + \frac{8}{3} + 2 + \frac{1}{3} = 4$$

So the problem is solved.

## Alternative Representations of Green's Theorem

### Alternative Representation 1

For a deeper understanding of Green's theorem, it can be helpful to look at it in several different ways.

In the first alternative representation, we will begin by taking a $C^1$ vector field

$$\mathbf{F} = M(x,y) \hat{\mathbf{i}} + N(x,y) \hat{\mathbf{j}}$$

That field is in 2D, but we can define it on $\mathbb{R}^3$ anyway and just define $\hat{\mathbf{k}}$ to be zero. If we do that, we can compute the curl of $\mathbf{F}$ (for a refresher on [curl, click here](/blog/2025-05-10/grad-div-curl))

$$\nabla \times \mathbf{F} = 
\begin{vmatrix} 
\hat{\mathbf{i}} & \hat{\mathbf{j}} & \hat{\mathbf{k}} \\\\
\partial / \partial x & \partial / \partial y & \partial / \partial z \\\\
M & N & 0
\end{vmatrix} =
\left( \frac{\partial N}{\partial x} - \frac{\partial M}{\partial y} \right) \hat{\mathbf{k}}
$$

Since $\hat{\mathbf{k}} \cdot \hat{\mathbf{k}} = 1$ we obtain

$$\iint_D \left( \frac{\partial N}{\partial x} - \frac{\partial M}{\partial y} \right) \ dA = \iint_D (\nabla \times \mathbf{F}) \cdot \hat{\mathbf{k}} \ dA$$

Since 

$$\oint_{\partial D} \mathbf{F} \cdot d \mathbf{s} = \oint_{\partial D} M dx + N dy$$

We can rewrite Green's theorem as

$$\oint_{\partial D} \mathbf{F} \cdot d \mathbf{s} = \iint_D (\nabla \times \mathbf{F}) \cdot \hat{\mathbf{k}} dA$$

What this tells us is that Green's theorem is a special case of the Stoke's theorem when applied to the 2D plane. I haven't yet written a blog on Stoke's theorem, but Stoke's theorem relates the line integral around some 3D shape to the curl within the shape. Later, I'll write a blog on Stoke's theorem and relate it back to Green's theorem.

#### Example 1

Verify the alternative Green's theorem on $\mathbf{F} = y \hat{\mathbf{i}} + x \hat{\mathbf{j}}$ on the square with vertices $(1,1), (-1, 1), (-1, -1), (1, -1)$

Starting with the right hand side. We need to compute the curl. As a quick reminder 

$$\nabla \times \phi = \left( 
\frac{\partial \phi_z}{\partial y} - \frac{\partial \phi_y}{\partial z},
\frac{\partial \phi_x}{\partial z} - \frac{\partial \phi_z}{\partial x},
\frac{\partial \phi_y}{\partial x} - \frac{\partial \phi_x}{\partial y}
\right)$$
Obviously we have no $z$ component here so only one of these could be anything but 0.
$$\nabla \times \mathbf{F} = \frac{\partial x}{\partial x} - \frac{\partial y}{\partial y} = 1-1=0$$
That being said, in this case, the whole right hand side is zero. No need to do anymore work.

For the lines, we will split it up into four integrals

- A: $(1,1) \rightarrow (-1,1)$
- B: $(-1,1) \rightarrow (-1,-1)$
- C: $(-1,-1) \rightarrow (1,-1)$
- D: $(1,-1) \rightarrow (1,1)$

A:

$$y = 1, dy = 0$$
$$\oint M dx = \int_1^{-1} dx = -2$$
B:

$$x = -1, dx = 0$$
$$\oint N dy = \int^{-1}_1 -dy = 2$$

C:

$$y = -1, dy = 0$$
$$\oint M dx = \int^1_{-1} -dx = -2$$

D:

$$x =1, dx = 0$$
$$\oint Ndy = \int^1_{-1} dy = 2$$

So we have 

$$-2+2-2+2=0$$

So we have verified!


#### Example 2

Verify the alternative Green's theorem on $\mathbf{F} = 2y \hat{\mathbf{i}} + x^2 \hat{\mathbf{j}}$ on the semi circle $x^2 + y^2 \leq a^2, y \geq 0$.

We can begin with the double integral

$$\nabla \times \mathbf{F} = \frac{\partial x^2}{\partial x} - \frac{\partial 2y}{\partial y} = 2x - 2$$

We will be doing an integration over a semi-circle, so it makes sense to instantly convert to polar coordinates.

$$x = r \cos(\theta), y = r \sin(\theta), dA = r \\, dr \\, d \theta$$
And also, since we are only doing the part of the circle above the $y$ axis, we need to only integrate the angle from $0$ to $\pi$.

$$\iint_D (\nabla \times \mathbf{F}) \cdot \hat{\mathbf{k}} dA = \int_0^\pi \int_0^a 2r^2 \cos(\theta) - 2r \\, dr \\, d \theta$$

$$\int_0^a 2r^2 \cos(\theta) - 2r \\, d \theta = \frac{2}{3} a^3 \cos(\theta) - a^2$$

$$\int_0^\pi \frac{2}{3} a^3 \cos(\theta) - a^2 \\, d \theta = \left[ \frac{2}{3} a^3 \sin(\theta) - a \theta \right]^\pi_0 = -a^2 \pi$$

For the line integrals, we have two lines - the straight line at $y = 0$ and the arc.

Straight line:

Very simple because $y = 0$ so 

$$\int_{-a}^a 2y \\, dx = 0$$
Arc:

We will be paramaterising this line

$$\gamma(t) = a \cos(t) \hat{\mathbf{i}} + a \sin(t) \hat{\mathbf{j}}$$
$$\gamma^\prime(t) = -a \sin(t) \hat{\mathbf{i}} + a \cos(t) \hat{\mathbf{j}}$$
$$F(\gamma(t)) = 2a \sin(t) \hat{\mathbf{i}} + a^2 \sin^2 (t) \hat{\mathbf{j}}$$
$$\oint_{\partial D} \mathbf{F} \cdot d \mathbf{s} = \int_0^\pi F(\gamma(t)) \cdot \gamma^\prime(t) dt$$
$$\int_0^\pi F(\gamma(t)) \cdot \gamma^\prime(t) dt = \int_0^\pi (2a \sin(t) \hat{\mathbf{i}} + a^2 \sin^2 (t) \hat{\mathbf{j}}) \cdot (-a \sin(t) \hat{\mathbf{i}} + a \cos(t) \hat{\mathbf{j}}) dt$$
$$\int_0^\pi F(\gamma(t)) \cdot \gamma^\prime(t) dt = \int_0^\pi -2a^2 \sin^2(t) + a^3 \sin^2(t) \cos(t) dt$$

We'll solve these one at a time (we've already integrated some of these before in this blog, so we should reuse results to be thrifty!)

$$-2a^2 \int^\pi_0 \sin^2(t) dt =  2a^2  \left[ \frac{-2 \sin(2t) + 2t}{4} \right]^\pi_0 = -a^2 \pi$$

For the other, we can use $u = \sin(t), du = \cos(t) dt$
$$a^3 \int_0^\pi \sin^2(t) \cos(t) dt = a^3 \int u^2 du = a^3 \left[\frac{u^3}{3} \right] = a^3 \left[  \frac{\sin^3(t)}{3} \right]^\pi_0 = 0$$

So we have verified it in this case!

### Alternative Representation 2

Suppose $\mathbf{x}(t) = (x(t), y(t)), a \leq t \leq b$ parameterises a $C^1$ segment of $\partial D$, then along this segment the unit vector $\mathbf{n}$ may be obtained by rotating the tangent vector $\mathbf{x}^\prime (t) /  \Vert \mathbf{x}^\prime (t) \Vert$ clockwise by $90^\circ$. Along this parameterised $C^1$ segment we have

$$\mathbf{n} = \frac{y^\prime (t) \hat{\mathbf{i}} - x^\prime (t) \hat{\mathbf{j}}}{ \sqrt{ x^\prime (t)^2 + y^\prime (t)^2 } } = \frac{ y^\prime (t) \hat{\mathbf{i}} - x^\prime (t) \hat{\mathbf{j}} }{ \Vert \mathbf{x}^\prime (t) \Vert}$$

We can calculate the line integral $\oint_{\partial D} \mathbf{F} \cdot \mathbf{n} ds$ along each $C^1$ segment of $\partial D$, the integral can be evaluated as 

$$\int_a^b (\mathbf{F}(\mathbf{x}(t)) \cdot \mathbf{n}(t)) \Vert \mathbf{x}^\prime (t) \Vert dt = \int_a^b (M(x(t), y(t)) \hat{\mathbf{i}} + N(x(t), y(t)) \hat{\mathbf{j}}) \cdot \frac{y^\prime (t) \hat{\mathbf{i}} - x^\prime (t) \hat{\mathbf{j}} }{ \Vert \mathbf{x}^\prime (t) \Vert} dt$$
$$\oint_{\partial D} \mathbf{F} \cdot d \mathbf{S} = \iint_D (\nabla \times \mathbf{F}) \cdot \hat{\mathbf{k}} dA$$
$$\int_a^b (\mathbf{F}(\mathbf{x}(t)) \cdot \mathbf{n}(t)) \Vert \mathbf{x}^\prime (t) \Vert dt = \int_a^b (M(x(t),y(t))y^\prime(t)-N(x(t),y(t))x^\prime (t)) dt$$

$$\int_a^b (\mathbf{F}(\mathbf{x}(t)) \cdot \mathbf{n}(t)) \Vert \mathbf{x}^\prime (t) \Vert dt = \int_{\mathbf{x}} - N \ dx + M \ dy$$

We can actually use Green's theorem here 

$$\oint_{\partial D} \mathbf{F} \cdot \mathbf{n} ds = \oint_{\partial D} -N dx + M dy = \iint_D \left( \frac{\partial M}{\partial x} - \frac{\partial (-N)}{\partial y} \right) dA$$

$$\int_a^b (\mathbf{F}(\mathbf{x}(t)) \cdot \mathbf{n}(t)) \Vert \mathbf{x}^\prime (t) \Vert dt = \iint_D \left( \frac{\partial M}{\partial x} + \frac{\partial N}{\partial y} \right) dA$$

$$\int_a^b (\mathbf{F}(\mathbf{x}(t)) \cdot \mathbf{n}(t)) \Vert \mathbf{x}^\prime (t) \Vert dt = \iint_D \nabla \cdot \mathbf{F} dA$$

or in more familiar terms...

$$\oint_{\partial D} \mathbf{F} \cdot \mathbf{n} ds = \iint_D \nabla \cdot \mathbf{F} dA$$

Which is the divergence of $\mathbf{F}$ (for a refresher on [divergence, click here](/blog/2025-05-10/grad-div-curl)).

What this tells us is that Green's theorem is also a special case of the Divergence theorem in 2D. I haven't yet written a blog on the Divergence theorem, but in short it tells you about the total amount of "stuff" leaving a shape and the total amount of "stuff" being made within a shape. When I do write a blog on Divergence theorem I'll relate it back to Green's theorem

### Pause: Notation and Computation

We have then three representations of Green's theorem, as follows
$$\oint M dx + N dy = \iint_D \left( \frac{\partial N}{\partial x} - \frac{\partial M}{\partial y}  \right) \ dx \ dy$$
$$\oint_{\partial D} \mathbf{F} \cdot d \mathbf{s} = \iint_D (\nabla \times \mathbf{F}) \cdot \hat{\mathbf{k}} dA$$
$$\oint_{\partial D} \mathbf{F} \cdot \mathbf{n} ds = \iint_D \nabla \cdot \mathbf{F} dA$$

Two of these represent fundamentally the same concepts, namely 

$$\oint M dx + N dy = \oint_{\partial D} \mathbf{F} \cdot d \mathbf{s}$$

So we can also say

$$\iint_D \left( \frac{\partial N}{\partial x} - \frac{\partial M}{\partial y}  \right) \ dx \ dy = \iint_D (\nabla \times \mathbf{F}) \cdot \hat{\mathbf{k}} dA$$

It's subtle but note that the third form is NOT the same as the first two

$$\oint_{\partial D} \mathbf{F} \cdot d \mathbf{s} \neq \oint_{\partial D} \mathbf{F} \cdot \mathbf{n} ds$$
They look pretty similar but they're genuinely different!

Critically, in the divergence form, we're using the *normal*. Just keep an eye out for this (and don't, like me, neglect this for hours and wonder why it isn't working.......)


#### Example 1

Verify the alternative Green's theorem on $\mathbf{F} = 2y \hat{\mathbf{i}} - 4x \hat{\mathbf{j}}$ on the ellipse $x^2 + 2y^2 \leq 4$

Starting with the double integral

$$\nabla \cdot \mathbf{F} = \frac{\partial 2y}{\partial x}  + \frac{-\partial 4x}{\partial y} = 0$$
At this point, we don't need to do anymore work as we know the answer will be 0.

Now for the line integral. We'll convert to polar since this is a circle. We need to parameterise an ellipse, which we can do by first rewriting the ellipse as 

$$\frac{x^2}{4} + \frac{y^2}{2} = 1$$
So the semi major axes are $2$ in $x$ and $\sqrt{2}$ in $y$. In other words

$$x = 2 \cos(t), \quad y = \sqrt{2} \sin(t), \quad t \in [0, 2 \pi]$$
$$x^\prime = -2 \sin(t), \quad y^\prime = \sqrt{2} \cos(t)$$
We can then use the identity

$$\mathbf{n} ds = (y^\prime, -x^\prime) dt$$ 
to give

$$\mathbf{n} ds = (\sqrt{2} \cos(t) \hat{\mathbf{i}} + 2 \sin(t) \hat{\mathbf{j}}) \\, dt$$

Meanwhile the parameterisation is 

$$\gamma(t) = 2 \cos(t) \hat{\mathbf{i}} + \sqrt{2} \sin(t) \hat{\mathbf{j}}$$

as usual. So,

$$F(\gamma(t)) = 2 \sqrt{2} \sin(t) \hat{\mathbf{i}} - 8 \cos(t) \hat{\mathbf{j}}$$



$$\int_0^{2 \pi} F(\gamma(t)) \cdot \mathbf{n} ds = \int_0^{2 \pi} (2 \sqrt{2} \sin(t) \hat{\mathbf{i}} - 8 \cos(t) \hat{\mathbf{j}}) \cdot (\sqrt{2} \cos(t) \hat{\mathbf{i}} + 2 \sin(t) \hat{\mathbf{j}}) \\, dt $$
$$\int_0^{2 \pi} F(\gamma(t)) \cdot \mathbf{n} ds = \int_0^{2 \pi} -12 \sin(t) \cos(t) dt = -12 \left[ \frac{\sin^2(t)}{2} \right]^{2 \pi}_0 = 0$$

So we have verified!
#### Example 2

Verify the alternative Green's theorem on $\mathbf{F} = (x^2 y + x) \hat{\mathbf{i}} + (y^3 - xy^2) \hat{\mathbf{j}}$ where $D$ is the region inside the circle $x^2 + y^2 = 9$ and outside the circle $x^2 + y^2 = 4$.

We'll begin with the double integral again. The divergence can be calculated as

$$\nabla \cdot F = 3y^2 + 1$$
We have circles so we will need to convert to polar

$$x = r \cos(\theta), \quad y = r \sin(\theta), \quad dA = r \\, dr \\, d \theta$$
$$\iint_D \nabla \cdot \mathbf{F} dA = \int_2^3 \int_0^{2 \pi} (3 r^2 \sin^2 (\theta) + 1) r \\, d\theta \\, d r$$

Solving the integrals one by one. We've already solved some of these integrals so we can reuse results

$$\int_0^{2 \pi} 3r^3 \sin^2 (\theta) + r \\, d \theta = \left[ \frac{3}{2} r^3 (\theta - \sin(\theta) \cos(\theta)) + r \theta\right]^{2 \pi}_0 = 3 \pi r^3 + 2 \pi r$$

$$\int_2^3 3 \pi r^3 + 2 \pi r \\, dr = \left[ \frac{3 \pi r^4}{4} + \frac{2 \pi r^2}{2} \right]^3_2 = \frac{215}{4} \pi$$

Now we turn our attention to the line integral. This is a fairly challenging problem. We can simplify it by realising that the annulus shape is the larger circle minus the smaller circle. So, rather than solving two line integral problems, we're going to solve $r= a$, and come up with a formula $\Phi(a)$. We then see that

$$\oint_{\partial D} \mathbf{F} \cdot \mathbf{n} ds = \Phi(3) - \Phi(2)$$

For this problem, the trigonometric terms can get out of hand. Therefore, to keep it clean, we define

$$C = \cos(t), \quad S = \sin(t), \quad K = \cos(2t), \quad Z = \sin(2t)$$

We can proceed as usual. We will obviously be converting to polar.

$$x = a C, \quad y = a S, \quad t \in [0, 2 \pi]$$
$$x^\prime = -a S, \quad y^\prime = a C$$
$$\mathbf{n} ds = (y^\prime, -x^\prime) dt = (aC \hat{\mathbf{i}} + a S \hat{\mathbf{j}}) dt$$

Now to parameterise the function
$$\gamma(t) = a C \hat{\mathbf{i}} + a S \hat{\mathbf{j}}$$
$$F(\gamma(t)) = (a^3 C^2 S + a C) \hat{\mathbf{i}} + (a^3 S^3 - a^3 C S^2)$$
Now to obtain the integrand 

$$F(\gamma(t)) \cdot \mathbf{n} = a^4 C^3 S + a^2 C^2 + a^4 s^4 - a^4 C S^3$$
We can massage this a little by using

$$S^2 + C^2 = 1$$
$$F(\gamma(t)) \cdot \mathbf{n} = a^4 S^4 + a^2 C^2 - a^4 CS (2 S^2 - 1)$$

And now to integrate 

$$\int F(\gamma(t)) \cdot \mathbf{n} ds = \int (a^4 S^4 + a^2 C^2 - a^4 CS (2 S^2 - 1)) dt$$

We immediately solve these one by one. We can use the power reduction formula here (see my integration blog tables to get it)

$$\int S^4 = -\frac{C S^3}{2} + \frac{1}{2} \int S^2 dt$$
And again

$$\int S^2 = - \frac{CS}{2} + \frac{1}{2} \int dt = - \frac{CS}{2} + \frac{t}{2}$$

So the first term (don't forget to carry the multiplications!)

$$\int S^4 = - \frac{C S^3}{4} - \frac{3 CS}{8} + \frac{3t}{8}$$

Second term (again, use integration tables)

$$\int C^2 = \frac{CS}{2} + \frac{t}{2}$$

Third term, by using $u = 2 S^2 - 1, du = 4 CS dt$

$$\int CS (2 S^2 - 1) dt = \frac{1}{4} \int u du = \frac{1}{4} \frac{u^2}{2} = \frac{(2 S - 1)^2}{8}$$

Collecting these up

$$\int_0^{2 \pi} F(\gamma(t)) \cdot \mathbf{n} ds = \left[ \frac{a^4(2s^2-1)^2}{8} - \frac{a^4 CS^3}{4} - \frac{3a^4CS}{8} + \frac{a^2 CS}{2} + \frac{3a^4t}{8} + \frac{a^2 t}{2}\right]^{2 \pi}_0$$

Recall that $\sin^n (0) = \sin^n (2 \pi) = 0$ so this actually reduces to 

$$\int_0^{2 \pi} F(\gamma(t)) \cdot \mathbf{n} ds = \frac{3}{4}a^4 \pi + a^2 \pi$$

We now have our formula, $\Phi(a) = \frac{3}{4}a^4 \pi + a^2 \pi$. So we can do

$$\oint_{\partial D} \mathbf{F} \cdot \mathbf{n} ds = \Phi(3) - \Phi(2)$$
$$\oint_{\partial D} \mathbf{F} \cdot \mathbf{n} ds = \frac{3}{4}3^4 \pi + 3^2 \pi - \frac{3}{4}2^4 \pi - 2^2 \pi = \frac{215}{4} \pi$$



## But What is Green's Theorem?

We've looked at a few different ways to write Green's theorem, and we've looked a little at what those representations mean, but we still haven't quite got to the heart of the matter: what actually *is* Green's theorem?

Let's get the second alternative representation of Green's theorem back again

$$\int_a^b (\mathbf{F}(\mathbf{x}(t)) \cdot \mathbf{n}(t)) \Vert \mathbf{x}^\prime (t) \Vert dt = \iint_D \nabla \cdot \mathbf{F} dA$$

If $C$ is a simple, oriented curve, the line integral $\int_C \mathbf{F} \cdot \mathbf{n} ds$, where $\mathbf{n}$ is the unit normal to $C$ as defined in the second alternative representation, is known as the *flux* of $\mathbf{F}$ across $C$. For example, if $\mathbf{F}$ represents the velocity of a vector field of a planar fluid, then the flux measures the rate of fluid transported across $C$ per unit time, assuming that $\mathbf{F}$ does not vary with time (there are actually ways of handling that too, but the maths balloons, so we won't be covering it).

Physically, you can think of a hose pipe. $C$ would represent the closed loop of the edge of the pipe, $D$ would represent the closed shape which forms the spout of the hosepipe ($C = \partial D$). $\mathbf{F}$ would represent the water coming out of the h

To see this, consider the amount of fluid transported across a small segment of $C$ during a brief time interval $\Delta t$. We have that the amount of fluid transported is approximately given by 

$$(\mathbf{F}(x,y) \Delta \cdot \mathbf{n}) \Delta s$$

where $\Delta s$ is the length of the segment of the curve $C$. This is only an approximate formula for the amount of fluid transported because the segment of the curve is not necessarily straight, and because the vector field $\mathbf{F}$ is not necessarily constant. If we instead divide by $\Delta t$ for the average rate of transport across the segment and also slice up the curve and take limits, we can get 

$$\frac{\Delta M}{\Delta t} \approx \int_C \mathbf{F} \cdot \mathbf{n} ds$$

And in the limit $\Delta t \rightarrow 0$

$$\frac{d M}{d t} \approx \int_C \mathbf{F} \cdot \mathbf{n} ds$$

which *is* the flux.

Okay, that's great, but we have a whole other form too. So what gives? There still feels to be something even more fundamental at work. As we've seen, the curl and divergence forms aren't themselves equal so how are they related.

In a word, Green's theorem is about bookkeeping. The more fundamental principle at play is this: what happens on the boundary of a shape is determined by what happens inside.

It's actually that simple, really. All Green's theorem notes is for the types of curves we've been studying, whatever we can observe on the boundary (whether the curl or divergence) is determined by what happens inside. Again if you think about a hosepipe, it's easy to visualise that the water at the very edge of the hosepipe is determined by the effect of all the water across the cross-sectional area of the stream.

Of course, the difference in this mental model to Green's is that Green's theorem works in 2D. It's more like the water passing out through the sides of the hose, but I couldn't think of a good analogy to that. However, after all of this, I think it should be clear that anything passing through the boundary of the shape has got to be the sum of what's happening within the shape!

## Proving Green's Theorem

Green's theorem in the general is actually really difficult to prove, but we shall dig into it here in some detail. We'll split the proof up into sections:

1. Regions which are type 3 (type 1 and type 2);
2. Regions which are not type 3, but can be subdivided into finitely many type 3 regions;
3. Regions which are not type 3 and can not be subdivided into finitely many type 3 regions.

Let's quickly be reminded of what type 1, type 2 and type 3 regions actually are. 

We say that $D$ is an *elementary region* in the plane if it can be described as a subset of $\mathbb{R}^2$ of one of the following three types.

**Type 1**

$$D = \{(x,y) | \gamma(x) \leq y \leq \delta(x), a \leq x \leq b \}$$
where $\gamma$ and $\delta$ are continuous on $[a,b]$.

**Type 2**

$$D = \{ (x,y) | \alpha(y) \leq x \leq \beta(y), c \leq y \leq d \}$$

where $\alpha$ and $\beta$ are continuous on $[c,d]$

**Type 3** 

Both type 1 and type 2 at the same time.

To visualise type 1 regions, notice how the $x$ values are bounded by constants $a \leq x \leq b$, while the $y$ values are bounded by functions $\gamma(x) \leq y \leq \delta(x)$. So I imagine a shape which has vertical left and right lines (constants) and wavy lines for the horizontal sections. As though you took the unit square and replaced $y = 0$ and $y = 1$ by sine functions (or any other function).

Type 2 regions are basically the same as above but swap $x$ and $y$. 

Type 3 regions are both at the same time. There's many possibilities, but the easiest to imagine is the unit square or even a circle. 

### Shapes Which are Type 3 Elementary Regions

We establish Green's theorem when $D$ is an elementary region in $\mathbb{R}^2$ of type 3. Thus, $D$ has the following properties

$$D = \{ (x,y) \in \mathbb{R}^2 | \gamma(x) \leq y \leq \delta(x), a \leq x \leq b \}$$
$$D = \{ (x,y) \in \mathbb{R}^2 | \alpha(y) \leq x \leq \beta(y), c \leq y \leq d \}$$

We can also assume that the functions $\alpha, \beta, \gamma, \delta$ are continuous and piecewise $C^1$

We can start by viewing $D$ as a type 1 elementary region. We can evaluate the corresponding part, namely $\oint_D M dx$. $\partial D$ consists of a lower curve $C_1$ and an upper curve $C_2$. We can parameterise the curves as 

$$C_1 : \begin{cases} x = t \\ y = \gamma(t) \end{cases} \quad a \leq t \leq b$$

$$C_2 : \begin{cases} x = t \\ y = \delta(t) \end{cases} \quad a \leq t \leq b$$

So $C_2$ is oriented opposite to the desired orientation, bearing this in mind, we can compute 

$$\oint_{\partial D} M(x,y) dx = \int_{C_1} M(x,y) dx - \int_{C_2} M(x,y) dx$$

note the minus sign!

$$\oint_{\partial D} M(x,y) dx = \int_a^b M(t, \gamma(t)) dt - \int_a^b M(t, \delta(t)) dt$$
$$\oint_{\partial D} M(x,y) dx = \int_a^b (M(t, \gamma(t)) - M(t, \delta(t))) dt$$

We can now compare the calculation of $\oint_{\partial D} M dx$ with $\iint_D - (\partial M / \partial y) dA$. We have

$$\iint_D - \frac{\partial M}{\partial y} dA = \int^b_a \int^{\delta(x)}_{\gamma(x)} - \frac{\partial M}{\partial y} \ dy \ dx$$
$$\iint_D - \frac{\partial M}{\partial y} dA = \int_a^b (-M(x, \delta(x)) + M(x, \gamma(x))) dx$$
$$\iint_D - \frac{\partial M}{\partial y} dA = \int_a^b (M(x, \gamma(x)) - M(x, \delta(x))) dx$$

Thus we see that

$$\oint_{\partial D} M dx \iint_D - \frac{\partial M}{\partial y} dA$$

By analogy 

$$\oint_{\partial D} N \ dy = \iint_D \frac{\partial N}{\partial x} dA$$

when viewing $D$ as a type 2 elementary region. Again, by analogy, we could show

$$\int_c^d (N(\beta(y), y) - N(\alpha(y), y)) dy$$

Since $D$ is simultaneously of type 1 and type 2 elementary regions

$$\oint_{\partial D} M(x,y) dx + N(x,y) dy = \oint_{\partial D} M \ dx + \oint_{\partial D} N \ dy$$

$$\oint_{\partial D} M(x,y) dx + N(x,y) dy = \iint_D - \frac{\partial M}{\partial y} \ dA + \iint_D \frac{\partial N}{\partial x} dA$$

$$\oint_{\partial D} M(x,y) dx + N(x,y) dy = \iint_D \left( \frac{\partial N}{\partial x} - \frac{\partial M}{\partial y} \right) dA$$

### Shapes Which Can be Subdivided into Finitely Many Type 3 Elementary Regions

Now suppose $D$ is not an elementary region of type 3, but can be subdivided into finitely many type 3 regions $D_1, D_2, \dots, D_n$ in such a way that these subregions overlap at most two at a time and only along common boundaries. A simple visualisation of this is a flat doughnut shape being cute into quarters. 

In step 1 we already showed Green's theorem held for each subregion, so 

$$\iint_D (N_x - M_y) dA = \iint_{D_1} (N_x - M_y) dA + \iint_{D_2} (N_x - M_y) dA + \cdots + \iint_{D_n} (N_x - M_y) dA$$

$$\iint_D (N_x - M_y) dA = \oint_{\partial D_1} M \ dx + N \ dy + \oint_{\partial D_2} M \ dx + N \ dy + \cdots + \oint_{\partial D_n} M \ dx + N \ dy$$

We would be jumping the gun slightly to conclude that this sums to our desired result, as we do have some portions of $\partial D_i$ which are not in $\partial D$ to take care of. Fortunately, we can actually realise that whenever we have this situation, it is always bordering exactly one other $\partial D_j$ directed in the opposite direction, thus, they always happen to cancel out in pairs!

$$\oint_{\partial D_1} M \ dx + N \ dy + \oint_{\partial D_2} M \ dx + N \ dy + \cdots + \oint_{\partial D_n} M \ dx + N \ dy = \oint_{\partial D} M dx + N dy$$

So we have established Green's theorem again

### Shapes Which Can Not be Subdivided into Finitely Many Type 3 Elementary Regions

Alas, I've mislead you dear reader. This blog shall contain no such proof of Green's theorem in the general. This is because a totally generic proof is really difficult to understand and requires a good dose of measure theory. Thus, we'll just sketch a general idea here.

An outline of the proof is that we divide the shape into infinitely many regions $D_1, D_2, \dots, D_n, \dots$ whose "limit" as $n \rightarrow \infty$ is $D$ and each $D_n$ can be subdivided into finitely many type 3 regions. We can claim that $\partial D_n \rightarrow \partial D$ as $n \rightarrow \infty$. Finally, we need to prove that as $n \rightarrow \infty$ that

$$\iint_D (N_x - M_y) dA \rightarrow \iint_D (N_x - M_y) dA$$
$$\oint_{\partial D_n} M dx + N dy \rightarrow \oint_{\partial D} M dx + N dy$$

Maybe one day I can get around to actually discussing the full proof in my blog, but I'm just not there yet in terms of some of the more difficult aspects of the theory. 

## What's Next?

By now you should have mastered Green's theorem! Well done! If you did the problems by yourself you may have found some of the integrals and geometry a bit challenging, but once you have these skills you will never unlearn them.

The next logical place to go from here is Stoke's Theorem and Divergence Theorem, both of which I have blogs coming for!

## References

Colley, S., J. (2012) *Vector Calculus (4th ed.)*. Pearson Education

Boas, M., L. (2005). *Mathematical Methods in the Physical Sciences (3rd ed.)*. Wiley