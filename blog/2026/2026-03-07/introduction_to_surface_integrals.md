## Introduction

In my [previous instalment on vector calculus](/blog/2026-01-03/introduction-to-greens-theorem) we looked at Green's theorem. We're almost ready to extend that into three dimensions, but we first need to learn how to do surface integrals. 

A lot of this material will feel very familiar to you, since the concepts we've been learning in double, triple and line integrals will be coming back up again. This time, we'll take the study in three stages - first we'll look at how to compute surface areas, then we'll look at scalar surface integrals and finally look at vector surface integrals.

Notice how just like line integrals, we can have scalar and vector surface integrals. 


## Surface Area

We can define a parametrised surface $S$ to be the image of a function $\mathbf{\Phi} : D \subset \mathbb{R}^2 \rightarrow \mathbb{R}^3$, written as $\mathbf{\Phi}(u,v) = (x(u,v), y(u,v), z(u,v))$. Thinking about this physically, imagine a sphere. The surface of that sphere is defined in 3D space by three coordinates. However, if we had two vectors that ran along the surface of the sphere, every point on the surface could be defined by two coordinates. For instance, if $\mathbf{T}_u$ ran (curved) from pole to pole and $\mathbf{T}_v$ ran (curved) around the equator, any point on the surface can be defined by those two vectors.

The surface area of a parameterised surface is defined by 

$$A(S) = \iint_D \lVert \mathbf{T}_u \times \mathbf{T}_v \rVert du \\ dv$$

We can define 

$$\mathbf{T}_u = \frac{\partial x}{\partial u} (u, v) \hat{\mathbf{i}} + \frac{\partial y}{\partial u} (u,v) \hat{\mathbf{j}} + \frac{\partial z}{\partial u} (u,v) \hat{\mathbf{k}}$$

$$\mathbf{T}_v = \frac{\partial x}{\partial v} (u, v) \hat{\mathbf{i}} + \frac{\partial y}{\partial v} (u,v) \hat{\mathbf{j}} + \frac{\partial z}{\partial v} (u,v) \hat{\mathbf{k}}$$

**Let's just pause here a moment**. Every textbook does this with $\mathbf{T}_u$ and $\mathbf{T}_v$, then provide the above definition. Maybe it is obvious to you, but it wasn't obvious to me, and I was confused for days about this: 

$$T_u = \frac{\partial T}{\partial u}$$

I think they use this notation to do two things:

1. Clean it up a little - some of these problems can get a lot of terms, and this is nicer
2. To make you not think about a derivative, but rather about literally taking small steps in $u$ and $v$ along the surface of the shape (thus bounding out a parallelogram)

Nevertheless, it would be great if textbooks could have made this clearer! At any rate, I will continue to use the standard notation, but I hope this leaves nobody confused. **Unpause**

We can think about the surface in terms of a Riemann Sum. 
Imagine some surface. If we approximate the surface by a series of rectangles, denoted $R\_{ij}$. The vertices of such a rectangle would be given by $(u\_i, v\_j),(u\_{i+1},v\_j),(u\_i,v\_{j+1}),(u\_{i+1},v\_{j+1})$. The values of the vectors spanning this surface at $u_i$ and $v_i$ can be given by $\mathbf{T}\_{u\_i}$ and $\mathbf{T}\_{v\_j}$ and the actual rectangle sides are given by $\Delta u \mathbf{T}\_{u\_i}$ and $\Delta v \mathbf{T}\_{v\_j}$. The surface area of one of those little rectangles is given by 
$$A(R\_{ij}) = \lVert \Delta u \mathbf{T}\_{u_i} \times \Delta v \mathbf{T}\_{v_j} \rVert = \lVert \mathbf{T}\_{u_i} \times \mathbf{T}\_{v\_j} \rVert \Delta u \\ \Delta v$$

Thus the area of the patchwork cover is

$$A\_n = \sum\_{i=0}^{n-1} \sum\_{j=0}^{n-1} A(R\_{ij}) = \sum^{n-1}\_{i=0} \sum^{n-1}\_{j=0} \lVert \mathbf{T}\_{u_i} \times \mathbf{T}\_{v\_j} \rVert \Delta u \\ \Delta v$$

And as $n \rightarrow \infty$ the sums $A_n$ converge to the integral

$$\iint\_D  \lVert \mathbf{T}\_{u_i} \times \mathbf{T}\_{v_j} \rVert d u \\ d v$$

**Why $D$**? In this blog there are always two surfaces going on - $S$ and $D$. This took me a while to get my head around, but I'll try and explain it as simply as I can. $S$ (in my notation) always refers to the actual surface of interest. $D$ is a square pseudo-surface. Think about a sphere again, its surface is $S$. Remember when I said

>For instance, if $\mathbf{T}_u$ ran (curved) from pole to pole and $\mathbf{T}_v$ ran (curved) around the equator, any point on the surface can be defined by those two vectors.

Sure, you can think about them running along the curved surface, but since they're 2D vectors running at right angles, they also describe a rectangle too. That pseudo-surface is $D$!

In other words, the *point* of paramaterisation is it helps us move from some arbitrary, difficult surface to a much simpler surface to do our integrals.

(NOTE: I imagine mathematicians would take issue with my term "pseudo-surface" here. To them, $D$ and $S$ are both just as real as each other. This is a term to help with understanding at this stage only!)

### Example - Surface Area of Sphere

We can represent a sphere of radius $r$ by 

$$T(\theta, \phi) = (r \sin(\phi) \cos(\theta), r \sin(\phi) \sin(\theta), r \cos(\theta))$$
where $\theta = [0, 2 \pi], \phi \in [0, \pi]$

Since the number of terms will grow rapidly, we can use some shorthands to clear things up: $\sin(\phi) = S_\phi, \cos(\phi) = C_\phi, \sin(\theta) = S_\theta, \cos(\theta) = C_\theta$

So an alternative writing is

$$T(\theta, \phi) = r S_\phi C_\theta \hat{\mathbf{i}} + r S_\phi S_\theta \hat{\mathbf{j}} + r C_\phi \hat{\mathbf{k}}$$

We can compute the partial derivatives

$$T_\theta = -r S_\phi S_\theta \hat{\mathbf{i}} + r S_\phi C_\theta \hat{\mathbf{j}}$$

$$T_\phi = r C_\phi C_\theta \hat{\mathbf{i}} + r C_\phi S_\theta \hat{\mathbf{j}} - r S_\theta \hat{\mathbf{k}}$$

Hopefully you recall how to do a cross product - if you don't this blog isn't for you yet! However, here's the cross product as a reminder

If 

$$\mathbf{a} = a_1 \hat{\mathbf{i}} + a_2 \hat{\mathbf{j}} + a_3 \hat{\mathbf{k}}, \quad \mathbf{b} = b_1 \hat{\mathbf{i}} + b_2 \hat{\mathbf{j}} + b_3 \hat{\mathbf{k}}$$

Then

$$\mathbf{a} \times \mathbf{b} = 
(a_2 b_3 - a_3 b_2) \hat{\mathbf{i}} +
(a_3 b_1 - a_1 b_3) \hat{\mathbf{j}} +
(a_1 b_2 - a_2 b_1) \hat{\mathbf{k}}$$

So

$$T_\theta \times T_\phi = 
(-r^2 S^2_\phi C_\theta) \hat{\mathbf{i}} +
(-r^2 S^2_\phi S_\theta) \hat{\mathbf{j}} + 
(-r^2 C_\phi S_\phi) \hat{\mathbf{k}}$$
$$\lVert T_\theta \times T_\phi \rvert = 
\sqrt{r^4 S^4_\phi C^2_\theta + r^4 S^4_\phi S^2_\theta + r^4 C^2_\phi S^2_\phi} = r^2 \sin(\phi)$$
Dropping out of the compressed notation now, since we won't have many terms from here on out - this is a nice case with lots and lots of cancels!

We need to solve

$$A = \int^\phi_0 \int^{2 \pi}_0 r^2 \sin(\phi) d \theta \ d \phi = \int 2r^2 \phi \sin(\phi) d \phi$$
Where the first integral is trivial

$$A = 2 \pi r^2 [-\cos(\phi)]^\pi_0 = 4 \pi r^2$$

Which is indeed the well known result for the surface area of a sphere!

### Example - Surface Area of Cone

A cone needs a little more geometric set up than a sphere. A cone has a circle base, which has a radius, $R$. A cone also has a height denoted by $h$. The height is the shortest distance from the circle base to the apex of the cone. Then there is the "slant height", $l$, which is the length of the slanted side. If you imagine taking a section of a cone, the radius and height would form a right angle, and the slant height would be a hypotenuse of a triangle.

We can then introduce a constant, $k$, to be a ratio between height and radius $\frac{h}{R}$. This tells us how "pointy" a cone is - low $k$ means a flat cone.

For a cone, we have

$$T(r, \theta) = (r \cos(\theta), r \sin(\theta), kr)$$
We introduce $r$ - a dummy variable we do the computation over, which is a variation in the radius from 0 to $R$.

$$T_r = \cos(\theta) \hat{\mathbf{i}} + \sin(\theta) \hat{\mathbf{j}} + k \hat{\mathbf{k}}$$

$$T_\theta = -r \sin(\theta) \hat{\mathbf{i}} + r \cos(\theta)$$

$$T\_r \times T\_\theta = -kr \cos(\theta) \hat{\mathbf{i}} -kr \sin(\theta) \hat{\mathbf{j}} + (r \cos^2(\theta) + r \sin^2(\theta)) \hat{\mathbf{k}}$$

Cleaning this up with the old $\cos^2(x) + \sin^2(x) = 1$

$$T_r \times T_\theta = -kr \cos(\theta) \hat{\mathbf{i}}
-kr \sin(\theta) \hat{\mathbf{j}} + r \hat{\mathbf{k}}$$
$$\lVert T_r \times T_\theta \rVert = \sqrt{k^2 r^2 \cos(\theta) + k^2 r^2 \sin^2(\theta) + r^2} = r \sqrt{1 + k^2}$$

We now have everything to compute the surface area

$$A = \int_0^R \int_0^{2 \pi} r \sqrt{1 + k^2} \ d \theta \ dr = \int^R_0 2 \pi r \sqrt{1 + k^2} \ dr$$

$$A = \pi R^2 \sqrt{1 + k^2}$$
So a pretty easy integration, all things considered. We can place this into a more familiar form by substituting out $k$

$$\pi R^2 \sqrt{1 + k^2} = \pi R l$$

Most formulas for cones use the slant height.

IMPORTANT NOTE: This gives the area of the curved cone area only (sometimes called the "lateral surface area"), for the full surface area you need to add on the area of the circle base too, so in other words, the total surface area of a cone is 

$$\pi R l + \pi R^2$$


## Scalar Surface Integral

Now that we know how to compute the surface area, we can go ahead and compute the scalar surface integral. Physically, a scalar surface integral will tell us about how much "stuff" is on the surface. For instance, if the function $f$ represented the mass density of a thin sheet, then the scalar surface integral would tell us the total mass of the sheet. An even better example might be if $f$ told us the thickness of some paint on a wall. Then the scalar surface integral would tell us the total volume of painted used to cover the wall.

As always, we can begin with thinking about some Riemann sums. Imagine some area $S$ and $f(x,y,z)$ is a continuous function whose domain includes $S$. Suppose $S$ is divided up into many rectangles $R_{ij}$, and $c_{jk}$ is the test point within the given rectangle. We know the area is apprxoimated by

$$A(R\_{ij}) = \lVert \mathbf{T}\_{u\_i} \times \mathbf{T}\_{v\_j} \rVert \Delta u \\ \Delta v$$

From the Riemann sum of the previous section. We can approximate the scalar surface area by simply multiplying by the value of the function at our test point

$$S\_n = \sum_{i=0}^{n-1} \sum_{j=0}^{n-1} f(c\_{ij}) \lVert \mathbf{T}\_{u_i} \times \mathbf{T}\_{v\_j} \rVert \Delta u \\ \Delta v$$

Imagine the thickness of paint example. We've chopped the wall up into a bunch of smaller rectangles, and then taken some point inside each rectangle and assumed the thickness of paint in the rectangle is constant at that value. Obviously, the thickness is actually varying within each rectangle, so this is an approximation. But that doesn't matter - if we make the rectangles really really small (i.e. take the limit) then the approximation approaches the correct value.

$$\lim_{n \rightarrow \infty} S\_n = \iint_S f dS$$

Of course, in the integration, we're no longer really taking a test point, but rather integrating the whole parameterised surface, so 

$$\iint\_S f dS = \iint\_D f(\mathbf{\Phi(u,v)}) \ \lvert \mathbf{T}\_{u} \times \mathbf{T}\_{v} \rvert \\ d u \\ d v$$

### Example 1

Evaluate the scalar surface integral of the function $f(x,y,z) = x+y$ over the surface given by

$$T(u,v) = (2u \cos(v), 2u \sin(v), u)$$
where $u \in [0,4]$ and $v \in [0, \pi]$

Computing the partials and cross products we have 

$$T_u = 2 \cos(v) \hat{\mathbf{i}} + 2 \sin(v) \hat{\mathbf{j}} + \hat{\mathbf{k}}$$

$$T_v = -2u \sin(v) \hat{\mathbf{i}} + 2 u \cos(v) \hat{\mathbf{j}}$$

$$T_u \times T_v = -2u \cos(v) \hat{\mathbf{i}} - 2u \sin(v) \hat{\mathbf{j}} + 4u \hat{\mathbf{k}}$$

$$\lvert T_u \times T_v \rvert = \sqrt{4u^2 \cos^2{v} + 4u^2 \sin^2(v) + 16u^2} = 2u \sqrt{5}$$

Now parameterising the function itself

$$f(T(u,v)) = 2u \cos(v) + 2u \sin(v) = 2u (\sin(v) + \cos(v))$$

Now we just need to do the integral

$$A = \int_0^\pi \int_0^4 2u (\sin(v) + \cos(v)) \cdot 2u \sqrt{5} \ du \ dv = \int_0^\pi \int_0^4 2u^2 \sqrt{5} (\sin(v) + \cos(v) \\ du \\ dv$$
$$A = \int_0^\pi \left[ \frac{4u^3}{3} \sqrt{5} (\sin(v) + \cos(v)) \right]^4_0 \ dv = \int_0^\pi \frac{256}{3} \sqrt{5} (\sin(v) + \cos(v)) \\ dv$$
$$A = \frac{256}{3} \sqrt{5} [-\cos(v) + \sin(v)]_0^\pi$$
$$A = \frac{512}{3} \sqrt{5}$$

### Example 2

Evaluate the scalar surface integral of the function $f(x,y,z) = x^2 + y^2 + z^2$ on the surface paramaterised by

$$T(s,t) = (s, s+t, t)$$

where $s \in [0,1], t \in [0,2]$.

Thankfully, in this case, the derivatives are much more simple!

$$T_s = \hat{\mathbf{i}} + \hat{\mathbf{j}}$$
$$T_t = \hat{\mathbf{j}} + \hat{\mathbf{k}}$$
$$T_s \times T_t = \hat{\mathbf{i}} - \hat{\mathbf{j}} + \hat{\mathbf{k}}$$
$$\lvert T_s \times T_t \rvert = \sqrt{1 + 1 + 1} = \sqrt{3}$$
Now paramaterising our function

$$f(T(s,t)) = s^2 + (s+t)^2 + t^2 = 2s^2 + 2t^2 + 2st$$

All that's left to do is the integral

$$A = 2 \sqrt{3} \int_0^2 \int_0^1 (s^2 + t^2 + st) \\ ds \\ dt$$
I'll break this one down

$$\int_0^1 s^2 + t^2 + st ds = \left[ \frac{s^3}{3} + st^2 + \frac{s^2 t}{2} \right]_0^1 = \frac{1}{3} + t^2 + \frac{t}{2}$$

$$\int_0^2 \frac{1}{3} + t^2 + \frac{t}{2} dt = \left[ \frac{t}{3} + \frac{t^3}{3} + \frac{t^2}{4} \right]_0^2 = \frac{13}{3}$$

So

$$A = \frac{26}{3} \sqrt{3}$$

## Vector Surface Integral

Now it's time to do vector surface integrals. Vector surface integrals represent the amount of "stuff" flowing through a surface. Suppose I have a butterfly net and swing it through the air - the amount of air passing through the net at any point in time is represented by a vector surface integral. Alternatively we could put a heater in a room and the heat leaving the room is again represented by the vector surface integral.

This "flow of stuff" has another name you may have come across: flux. A property of flux is it has a direction. Imagine a sphere with a heat source inside. The heat is flowing from "inside" to "outside". However, mathematically, there's no reason to define the positive side of the sphere with the conventional outside. Sometimes for convenience we might define the conventional inside of the sphere as outside, which would flip the direction of flux (even though the physical "stuff" is unchanged - this is just a directional definition change, like flipping coordinates).

While the above paragraph might be confusing, the point I'm making is we now need to be careful about which direction of the surface we define as the positive and negative direction. It's possible to get different but correct answers to these problems if your direction is reversed. In complex problems with lots of surfaces, if you aren't consistent, you will get them wrong. It doesn't matter which way you orient things, so long as it is consistent.

Anyway, the formula for a vector surface integral is give by

$$\iint_S \mathbf{F} \cdot d \mathbf{S} = \iint_D \mathbf{F}(\mathbf{\Phi}(u,v)) \cdot \mathbf{N}(u,v) du \ dv$$
where $\mathbf{N}(u,v) = \mathbf{T}_u \times \mathbf{T}_v$


### Example 1

The temperature in $\mathbb{R}^3$ is defined as $f(x,y,z) = 3x^2 + 3z^2$. Compute the heat flux across the surface given by $x^2 + z^2 = 2$ where $y \in [0,2]$ if $k=1$.

For this one, we notice that the surface is a circle, extruded in $y$, or in other words, a cylinder. S, we will use cylindrical coordinates, so find that the paramaterisation of the surface is given by

$$T(\theta, y) = (\sqrt{2} \cos(\theta), y, \sqrt{2} \sin(\theta))$$

where now $y \in [0,2]$ and $\theta \in [0, 2 \pi]$.

From here, we can proceed as usual

$$T_\theta = - \sqrt{2} \sin(\theta) \hat{\mathbf{i}} + \sqrt{2} \cos(\theta) \hat{\mathbf{k}}$$
$$T_y = \hat{\mathbf{j}}$$

$$N(\theta, y) = T_\theta \times T_y = -\sqrt{2} \cos(\theta) \hat{\mathbf{i}} - \sqrt{2} \sin(\theta) \hat{\mathbf{k}}$$

Heat flux is defined as $\mathbf{F} = k \Delta f$, and since $k = 1$ then for us $\mathbf{F} = \Delta f$. Since $f = 3x^2 + 3z^2$ then

$$\mathbf{F} = 6x \hat{\mathbf{i}} + 6z \hat{\mathbf{k}}$$
We can now proceed as usual

$$F(T(\theta, y)) = 6 \sqrt{2} \cos(\theta) \hat{\mathbf{i}} + 6 \sqrt{2} \sin(\theta) \hat{\mathbf{k}}$$

$$F(T(\theta, y)) \cdot N(\theta, y) = -12 \cos^(\theta) - 12 \sin^2(\theta) = -12$$
$$\int_0^{2} \int_0^{2 \pi} -12 \ d \theta \ dy = \int_0^2 -24 \pi dy = -48 \pi$$

NOTE: You may also get $+48 \pi$ if you orient the surface differently to me

### Example 2

Calculate the flux where $\mathbf{F} = (x + 3y^5) \hat{\mathbf{i}} + (y + 10xz) \hat{\mathbf{j}} + (z-xy) \hat{\mathbf{k}}$ over the half sphere given by $x^2 + y^2 + z^2 \leq 1, z \geq 0$.

We've so far been using the right hand side of the provided equations, while that might be easier sometimes, it can also make it more difficult. If you try solving 

$$\iint_D \mathbf{F}(\mathbf{X}(s,t)) \cdot \mathbf{N}(s,t) ds \\ dt$$

For this one it can become quite the mess. Instead, try

$$\iint_\mathbf{X} \mathbf{F} \cdot d \mathbf{S}$$

This is because here $\mathbf{n} = (x,y,z)$. So

$$d \mathbf{S} = \mathbf{n} dS, \quad dS = \sin(\phi) \ d \phi \ d \theta$$
$$\mathbf{F} \cdot d \mathbf{S} = (\mathbf{F} \cdot \mathbf{n}) dS = (\mathbf{F} \cdot (x,y,z)) \sin(\phi) \ d \phi \ d \theta$$
$$\mathbf{F} \cdot d \mathbf{S} = x(x+3y^5) + y(y+10xz) + (z-xy)$$
$$\mathbf{F} \cdot d \mathbf{S} = x^2 + 3xy^5 + y^2 + 10xyz + z^2 -xyz$$

Now a trick here is that on the unit sphere $x^2 + y^2 + z^2 = 1$ so

$$\mathbf{F} \cdot d \mathbf{S} = 1 + 3xy^5 + 9xyz$$

We can eliminate two terms here: $3xy^5$ and $9xyz$.

Since 

$$x = \sin(\phi) \cos(\theta)$$
$$y = \sin(\phi) \sin(\theta)$$
$$z = \cos(\phi)$$

With $\theta \in [0,2\pi]$ then

$$3xy^5 = 3\sin^6(\phi) \cos(\theta) \sin^5 (\theta)$$

Since this is odd in the $\sin^5(\theta)$ term the whole thing is 0 when integrated over $0 \rightarrow 2 \pi$

$$9xyz = 9 \sin^2 (\phi) \cos(\phi) \cos(\theta) \sin(\theta)$$
$$\cos(\theta) \sin(\theta) = 1/2 \sin(2 \theta)$$
Again, odd so this will go to 0 in our integral. Therefore

$$\iint_\mathbf{X} \mathbf{F} \cdot d \mathbf{S} = \int_0^{2 \pi} \int_0^{\pi / 2} \sin{\phi} \ d \phi \ d \theta$$
$$\int_0^{\pi / 2} \sin{\phi} \ d \phi \ d \theta = 1$$
$$\iint_\mathbf{X} \mathbf{F} \cdot d \mathbf{S} = \int_0^{2 \pi} d \theta = 2 \pi$$

NOTE: If you orient "out" differently to me you will get $-2\pi$

## Conclusion

You should now be able to perform surface integrals! The next steps in vector calculus from here is Divergence theorem and Stokes theorem.

As a summary:

$$A(S) = \iint\_D \lVert \mathbf{T}\_u \times \mathbf{T}\_v \rVert du \\ dv$$

$$\iint\_S f dS = \iint\_D f(\mathbf{\Phi(u,v)}) \ \lvert \mathbf{T}\_{u} \times \mathbf{T}\_{v} \rvert \\ d u \\ d v$$

$$\iint_S \mathbf{F} \cdot d \mathbf{S} = \iint\_D \mathbf{F}(\mathbf{\Phi}(u,v)) \cdot \mathbf{N}(u,v) du \\ dv$$

## References

Boas, M., L. (2005). *Mathematical Methods in the Physical Sciences (3rd ed.)*. Wiley

Colley, S., J. (2012) *Vector Calculus (4th ed.)*. Pearson Education

Marsden, J., E., Tromba, A. (2012) *Vector Calculus (6th ed.)*. W. H. Freeman and Company.