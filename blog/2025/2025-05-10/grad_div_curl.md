In this article, our goal shall be to study three mathematical operations in vector calculus

1. The gradient (grad): turns a scalar field into a vector field
2. The divergence (div): turns a vector field into a scalar field
3. The curl (curl): turns a vector field into a different vector field

We shall be studying these in a very formal mathematical way, breaking beyond just mere computation to understand *why* they exist, and *what* they really are. If you're only looking for a quick reference for grad, div, curl, this probably isn't for you (but I'll pop the main results in the tl;dr just below if you want to grab them and get out).

## tl;dr

Grad is the gradient of a vector. It measures where the slop of a vector is the steepest

\\[\nabla \phi = \left( \frac{\partial \phi}{\partial x}, \frac{\partial \phi}{\partial y}, \frac{\partial \phi}{\partial z} \right)\\]

Div is a measure of direction of flow of a vector - if things are spreading out it is a positive div. Div is a scalar

\\[\nabla \cdot \phi = \frac{\partial \phi_x}{\partial x} + \frac{\partial \phi_y}{\partial y} + \frac{\partial \phi_z}{\partial z}\\]

Curl measures the "twistiness" of a vector. Imagine placing a small stick into the vector and seeing which way it rotates

\\[\nabla \times \phi = \left( 
\frac{\partial \phi_z}{\partial y} - \frac{\partial \phi_y}{\partial z},
\frac{\partial \phi_x}{\partial z} - \frac{\partial \phi_z}{\partial x},
\frac{\partial \phi_y}{\partial x} - \frac{\partial \phi_x}{\partial y}
\right)\\]

## Scalar Fields and Vector Fields

Let's first understand what a "vector field" and a "scalar field" is.

A scalar field is defined by

\\[f : X \subseteq \mathbb{F}^n \rightarrow \mathbb{F}\\]

So let's really rip this apart. \\(f\\) is a function here - a field is a function which gives every point in space a value. \\(f :\\) is the notation which says "\\(f\\) is a function defined by". 

\\(\mathbb{F}\\) is unfortunately itself called a field. THIS IS A DIFFERENT FIELD. It is super unfortunate that both of these concepts use the term "field". Yes, this is confusing. However, this field \\(\mathbb{F}\\) is actually a *set* not a function. This set is essentially short for any set which has the operations addition and multiplication defined which satisfy a bunch of axioms: associativity, commutativity, distributivity, identities, and every nonzero element having a multiplicative inverse. We will not be able to cover the full extent of what this means in this article however, as a very brief summary you can essentially replace \\(\mathbb{F}\\) in your mind with "\\(\mathbb{R}\\) or \\(\mathbb{C}\\) or \\(\mathbb{Q}\\)". \\(\mathbb{F}\\) is a shorthand, because a lot of things in this article will stay the same whether we use the real numbers, complex numbers or the rational numbers (there are other sets of numbers which are also fields but these are the 99.9% of cases you'll ever come across).

\\(\mathbb{F}^n\\) is saying "an \\(n\\) dimensional vector". So in two dimensions that would be \\(\mathbb{F}^2\\) which could be a vector like \\(v = (x,y)\\) and in three dimensions that could look like \\(\mathbb{F}^3\\) which could be a vector like \\(v = (x,y,z)\\).

So \\(f : X \subseteq \mathbb{F}^n \rightarrow \mathbb{F}\\) is saying "\\(f\\) is a function defined on the subset \\(X\\) of the vector \\(\mathbb{F}^n\\) which maps to the values \\(\mathbb{F}\\)". In other words, a field is a function which assigns a scalar value to every point in a vector space. 

This is much easier to understand with some physical examples. Imagine a temperature field. We will consider the temperature of a room. Now the vector space \\(\mathbb{R}^3\\) is essentially the whole universe, but we will only consider the subsection \\(X\\) i.e. the room. Now this temperature field at every point in the room will assign a temperature value. 

Another example might be gravitational potential energy. If we imagine some mass in space, the vector space is \\(\mathbb{R}^3\\). We'll consider the potential in the entire universe from this mass, so \\(X\\) is \\(\mathbb{R}^3\\) in this case. And again, the function \\(f\\) now is a field which assigns a gravitational potential to every point in space.

A vector field can be defined by

\\[\mathbf{F} : X \subseteq \mathbb{R}^n \rightarrow \mathbb{R}^n\\]

Basically everything which has been said so far can now be repeated here, except for one detail which is that we know map every point in the vector space to a vector, rather than a scalar value.

A physical example might be the gravitational field itself. Imagine some mass in space again. The vector space is \\(\mathbb{R}^3\\) and at every point in space this defines a force of attraction to that mass, which is a vector. 

Another physical example might be the velocity of a fluid. If we imagine some tank of water which is in motion, the vector space is \\(\mathbb{R}^3\\), but we only consider the subset \\(X\\) inside the tank (what is the velocity of a fluid where there is no fluid?). At every point the water has some velocity, which is a vector.

## Gradient

Consider a scalar field again. Let's say it's a temperature field in a room. Suppose that at one end of the room there's a heater, and at the other there's an open window. Clearly there is going to be a difference in temperature between these two sides of the room. 

Let's say the temperature field is \\(T(x,y,z)\\). Let's just first consider moving along the \\(x\\) axis of the room. What is the change in temperature? Very simply it would be \\(\frac{\partial T}{\partial x}\\).

By extension in the \\(y\\) direction the change would be \\(\frac{\partial T}{\partial y}\\), and of course in the \\(z\\) direction it would be \\(\frac{\partial T}{\partial z}\\).

We can actually very easily generalise this across all three axis at the same time by writing

\\[\nabla T = \frac{\partial T}{\partial x} \mathbf{\hat{i}} + \frac{\partial T}{\partial y} \mathbf{\hat{j}} + \frac{\partial T}{\partial z} \mathbf{\hat{k}}\\]

Where \\(\nabla\\) is the "nabla operator" or sometimes called the "del operator". Itself has the definition

\\[\nabla = \mathbf{\hat{i}} \frac{\partial}{\partial x} + \mathbf{\hat{j}} \frac{\partial}{\partial y} + \mathbf{\hat{k}} \frac{\partial}{\partial z}\\]

The empty partials are spaces waiting for a suitable field to differentiate. 

### Example 1

Suppose that a scalar temperature field is defined by \\(T = x^2 - y^2 + xyz + 12\\). The gradient is

\\[\nabla T = \frac{\partial T}{\partial x} \mathbf{\hat{i}} + \frac{\partial T}{\partial y} \mathbf{\hat{j}} + \frac{\partial T}{\partial z} \mathbf{\hat{k}} = (2x + yz) \mathbf{\hat{i}} + (-2y + xz) \mathbf{\hat{j}}  + (xy) \mathbf{\hat{k}}\\]

If we evaluate this at \\((1,2,3)\\) we find

\\[\nabla T = (4 + 6) \mathbf{\hat{i}} + (-4 + 3) \mathbf{\hat{j}} + 2 \mathbf{\hat{k}} = 10 \mathbf{\hat{i}} - \mathbf{\hat{j}} + 2 \mathbf{\hat{k}}\\]

### Example 2

Let's take a second example of the field \\(f(x,y) = x^2 - y^2\\), at the point \\((1,1)\\). We want to figure out in which direction and at what rate is the greatest increase in the field at this point.

\\[\nabla f = 2x \mathbf{\hat{i}} - 2y \mathbf{\hat{j}}\\]

So at \\((1,1)\\)

\\[\nabla f = \mathbf{u} = 2 \mathbf{\hat{i}} - 2 \mathbf{\hat{j}}\\]

We can now get the rate as

\\[\lvert \nabla f(1,1) \rvert = \lvert \mathbf{u} \rvert = \sqrt{2^2 + (-2)^2} = 2 \sqrt{2}\\]

We can also turn this into a unit vector, \\(\mathbf{\hat{u}}\\)

\\[\mathbf{\hat{u}} = \frac{\mathbf{u}}{\lvert \mathbf{u} \rvert} = \frac{2}{2 \sqrt{2}} \mathbf{\hat{i}} - \frac{2}{2 \sqrt{2}} \mathbf{\hat{j}} = \frac{1}{\sqrt{2}} \mathbf{\hat{i}} - \frac{1}{\sqrt{2}} \mathbf{\hat{j}}\\]

### Grad in Spherical and Cylindrical Coordinates

We can also define grad in spherical and cylindrical coordinates. In cyclindrical:

\\[\nabla f = \mathbf{\hat{e}\_r} \frac{\partial f}{\partial r} + \mathbf{\hat{e}\_\theta} \frac{1}{r} \frac{\partial f}{\partial \theta} + \mathbf{\hat{e}\_z} \frac{\partial f}{\partial z}\\]

(note the \\(\frac{1}{r}\\))

In spherical:

\\[\nabla f = \mathbf{\hat{e}\_r} \frac{\partial f}{\partial r} + \mathbf{\hat{e}\_\theta} \frac{1}{r} \frac{\partial f}{\partial \theta} + \mathbf{\hat{e}\_\phi} \frac{1}{r \sin(\phi)}\frac{\partial f}{\partial \phi}\\]

### Example 3

Let's consider a charge. The potential caused by this charge is given by

\\[f(r, \theta, \phi) = \frac{1}{r}\\]

Therefore, the gradient of this field is

\\[\nabla f = - \frac{1}{r^2} \mathbf{\hat{e}_r}\\]

In other words, the electric field points towards the charge (the minus sign).

### Example 4

Consider an infinitely long hot vertical wire, and consider that the temperature around it is defined by

\\[f(r, \theta, z) = \ln(r)\\]

Therefore, the gradient is

\\[\nabla f = \frac{1}{r} \mathbf{\hat{e}_r}\\]

## Div

As we saw, grad turns a scalar field into a vector field. Div does the opposite: given a vector field we shall get back a scalar field. Div is defined by

\\[\nabla \cdot \mathbf{F(x,y,z)} = \frac{\partial F_x}{\partial x} + \frac{\partial F_y}{\partial y} + \frac{\partial F_z}{\partial z}\\]

Note that grad is \\(\nabla f\\) and div is \\(\nabla \cdot \mathbf{F}\\) - note the additional \\(\cdot\\).

The physical intuition of div is all about flow. It's the amount of "stuff" flowing out minus the "stuff" flowing in. Imagine a balloon:

- Positive divergence = balloon inflates. Field is diverging out of the point
- Negative divergence = balloon deflates. Field is converging into the point
- Zero divergence = balloon stays the same. Field might be swirling but is net constant

In summary div is telling us how much the vector field is behaving as a source (positive divergence) or sink (negative divergence)

### Example One

Consider the field

\\[\mathbf{F} = x \mathbf{\hat{i}} + y \mathbf{\hat{j}}\\]

Then 

\\[\nabla \cdot \mathbf{F} = 1 + 1 = 2\\]

So in this field the divergence is 2 everywhere. This means at every point the flow is greater away from each point than into it; in other words, \\(\mathbf{F}\\) is diverging at every point.

### Example Two

Consider 

\\[\mathbf{F} = x^2 y \mathbf{\hat{i}} + y^2 x \mathbf{\hat{j}} + xyz \mathbf{\hat{k}}\\]

Then the divergence is 

\\[\nabla \cdot \mathbf{F} = 2xy + 2xy + xy = 5xy\\]

So at, say, \\((3,2,0)\\) the divergence is

\\[\nabla \cdot \mathbf{F} = 5 \cdot 3 \cdot 2 = 30\\]

### Div in Spherical and Cylindrical Coordinates

In spherical coordinates:

\\[\nabla \cdot \mathbf{F} = \frac{1}{r^2} \frac{\partial (r^2 F_r)}{\partial r} + \frac{1}{\sin(\theta)} \frac{\partial (\sin(\theta F_\theta))}{\partial \theta} + \frac{1}{r \sin(\theta)} \frac{\partial F_\phi}{\partial \phi}\\]

In cylindrical coordinates:

\\[\nabla \cdot \mathbf{F} = \frac{1}{r}\frac{\partial (r F_r)}{\partial r} + \frac{1}{r} \frac{\partial F_\theta}{\partial \theta} + \frac{\partial F_z}{\partial z}\\]

### Example Three

Let's consider the electric field from a point charge (in spherical coordinates)

\\[\mathbf{F}(r) = \frac{1}{r^2}\mathbf{\hat{e}_r}\\]

If we compute the divergence

\\[\nabla \cdot \mathbf{F} = \frac{1}{r^2} \frac{\partial}{\partial r} \left( r^2 \cdot \frac{1}{r^2} \right)= \frac{1}{r^2} \frac{\partial 1}{\partial r} = 0\\]

Wait... shouldn't there be a source here? After all the point charge is generating the field!

That's correct, but the field only acts like a source at one point, the origin. The field is neither creating or consuming "stuff" outside of that origin point. It is possible to compute the divergence at that point, but not using the techniques presented in this article (I will later do an article on the divergence theorem which will allow us to compute it).

You might think the field is being consumed as it spreads out because it comes weaker, but that effect is due only to inverse square law: that does not consume the field, only dilute its strength over space.

### Example Four

Let's do a similar example to example three but now we have some source along a vertical infinite wire (cylindrical coordinates)

\\[\mathbf{F}(r) = \frac{1}{r} \mathbf{\hat{e}_r}\\]

The div is calculated by

\\[\nabla \cdot \mathbf{F} = \frac{1}{r} \frac{\partial}{\partial r} \left(r \cdot \frac{1}{r} \right) = \frac{1}{r} \frac{\partial}{\partial r} 1 = 0\\]

Again, we the same phenomena (and we will need to develop the divergence theorem later to find the divergence at the origin).

## Curl

The curl of a vector field can be defined as follows

\\[\nabla \times \mathbf{F} = \left( 
\frac{\partial F_z}{\partial y} - \frac{\partial F_y}{\partial z},
\frac{\partial F_x}{\partial z} - \frac{\partial F_z}{\partial x},
\frac{\partial F_y}{\partial x} - \frac{\partial F_x}{\partial y}
\right)\\]

As the name implies, curl measures the "twistiness" of a vector field. However it does so in a very subtle way. Imagine a whirlpool. If you drop an object into that whirlpool it will make a big circle around the whole whirlpool. This is not the "twistiness" that curl measures. Instead, imagine a paddle fixed in place and free to rotate on some axis. The curl is how much that paddle rotates about the axis. Crucially, the direction of curl tells you the direction of that axis.

### Example One

Consider 

\\[\mathbf{F} = (3x^2 z + y^2) \mathbf{\hat{i}} + 2xy \mathbf{\hat{j}} + (x^3 - 2z) \mathbf{\hat{k}}\\]

The curl would then be

\\[\nabla \times \mathbf{F} =
\left( \frac{\partial}{\partial y} (x^3 - 2z) - \frac{\partial}{\partial z} (2xy) \right) \mathbf{\hat{i}} + 
\left( \frac{\partial}{\partial z} (3x^2 z + y^2) - \frac{\partial}{\partial x} (x^3 - 2z)  \right) \mathbf{\hat{j}} + 
\left( \frac{\partial}{\partial x} (2xy) - \frac{\partial}{\partial y} (3x^2z + y^2)  \right) \mathbf{\hat{k}}\\]

\\[\nabla \times \mathbf{F} = (0-0) \mathbf{\hat{i}} + (3x^2 - 3x^2) \mathbf{\hat{j}} + (2y - 2y) \mathbf{\hat{k}} = 0\\]

This field has no curl anywhere - we call such a field *irrotational*.

### Example Two

Consider

\\[ \mathbf{F} = x^2 y \mathbf{\hat{i}} - 2xy \mathbf{\hat{j}} + (x + y - z) \mathbf{\hat{k}}\\]

Then the curl is 

\\[\nabla \times \mathbf{F} =
\left(\frac{\partial}{\partial y} (x + y -z) - \frac{\partial}{\partial z} (-2xz) \right) \mathbf{\hat{i}} +
\left(\frac{\partial}{\partial z} (x^2 y) - \frac{\partial}{\partial x} (x+y-z) \right) \mathbf{\hat{j}} +
\left(\frac{\partial}{\partial x} (-2xz) - \frac{\partial}{\partial y} (x^2 y) \right) \mathbf{\hat{k}}\\]

\\[\nabla \times \mathbf{F} = (1+2x) \mathbf{\hat{i}} - \mathbf{\hat{j}} - (x^2 + 2z) \mathbf{\hat{k}}\\]

### Curl in Spherical and Cylindrical Coordinate

In spherical coordinates:

\\[\nabla \times \mathbf{F} =
\frac{1}{r \sin(\theta)} \left( \frac{\partial}{\partial \theta} (F\_\phi \sin(\theta)) - \frac{\partial F\_\theta}{\partial \phi}) \right) \mathbf{\hat{e}\_r} +
\frac{1}{r} \left( \frac{1}{\sin(\theta)} \frac{\partial F\_r}{\partial \phi} - \frac{\partial (r F\_\phi)}{\partial r} \right) \mathbf{\hat{e}\_\theta} +
\frac{1}{r} \left( \frac{\partial (r F\_\theta)}{\partial r} - \frac{\partial F\_r}{\partial \theta} \right) \mathbf{\hat{e}\_\phi}\\]

In cylindrical coordinates:

\\[\nabla \times \mathbf{F} = 
\left( \frac{1}{r} \frac{\partial F\_z}{\partial \theta} \right) \mathbf{\hat{e}\_r} + 
\left( \frac{\partial F\_r}{\partial z} - \frac{\partial F\_z}{\partial r}  \right) \mathbf{\hat{e}\_\theta} +
\frac{1}{r} \left( \frac{\partial (r F\_\theta)}{\partial r} - \frac{\partial F\_r}{\partial \theta} \right) \mathbf{\hat{e}\_z}\\]

### Example Three

Consider (in cylindrical coordinates)

\\[\mathbf{F} = r \mathbf{\hat{e}_\theta}\\]

So the curl is

\\[\nabla \times \mathbf{F} = \frac{1}{r} \left( \frac{\partial r^2}{\partial r} - 0 \right) \mathbf{\hat{e}_z} = \frac{1}{r} \cdot 2r \mathbf{\hat{e}_z} = 2 \mathbf{\hat{e}_z}\\]

In other words, constant upwards curl at all places in the field.

### Example Four

Consider (in spherical coordinates)

\\[\mathbf{F} = \sin(\theta) \mathbf{\hat{e}_\phi}\\]

So 

\\[\nabla \times \mathbf{F} = \frac{1}{r \sin(\theta)} \left( \frac{\partial \sin^2(\theta)}{\partial \theta} - 0 \right) \mathbf{\hat{e}_r} = \frac{2 \cos(\theta)}{r} \mathbf{\hat{e}_r}\\]

## References

Riley, K., F. (1974). *Mathematical Methods for the Physical Sciences: An Informal Treatment for Students of Physics and Engineering*. Cambridge University Press

Boas, M., L. (2005). *Mathematical Methods in the Physical Sciences (3rd ed.)*. Wiley

Velleman, D. J. (2019) _How to Prove It: A Structured Approach (3rd ed)_. Cambridge University Press

Colley, S., J. (2012) *Vector Calculus (4th ed.)*. Pearson Education

Axler, S. (2015) *Linear Algebra Done Right (3rd ed.)*. Springer