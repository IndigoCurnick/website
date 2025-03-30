In this blog we'll begin to understand imaginary and complex numbers: why they 
exist and their properties. We'll finish up with a few real world examples from 
physics to see that imaginary numbers are just a really bad name: they do 
actually exist!

## Basic Motivation

Consider the following quadratic equation

\\[x^2 - 4 = 0\\]

We can easily solve this equation to see where it crosses the \\(y=0\\) line:

\\[x^2 = 4\\]

\\[x = \sqrt{4}\\]

\\[x = 2, x = -2\\]

No surprises here at all! But now consider

\\[ x^2 + 4 = 0 \\]

\\[x^2 = -4 \\]

\\[x = \sqrt{-4}\\]

What's the square root of a negative number? Well, for a long time we'd say there
isn't one. But what if we allow for such a number? We introduce \\(i\\), the
imaginary number, which is equal to \\(\sqrt{-1}\\). Thus the solutions to the
above becomes

\\[x = 2i, x = -2i\\]

As it turns out, these are real and valid solutions to polynomials! This fact 
will become more obvious as this article goes on.

## Complex Numbers and Their Properties

Imaginary numbers are not often used by themselves, we usually wrap it up into 
a complex number. A complex number is simply a number with a real part, and an 
imaginary part, of the form

\\[z = x + yi \\]

Where \\(x\\) and \\(y\\) are real numbers. \\(z\\) is a complex number here.

We define two functions on complex numbers to help us work with them

\\[\Re{z} = x\\]

\\[\Im{z} = y\\]

In other words, \\(\Re\\) extracts the real part and \\(\Im\\) extracts the 
imaginary part.

Let's look at the operations that complex numbers can take

Addition:

\\[z + z^\prime = (x + yi) + (x^\prime + y^\prime i) = (x+x^\prime) + (y+y^\prime)i \\]

Subtraction:

\\[ z - z^\prime = (x + iy) - (x^\prime + iy^\prime) = (x-x^\prime) + (y - y^\prime)i \\]

Multiplication:

\\[zz^\prime = (x + yi) (x^\prime + y^\prime i) = (xx^\prime - yy^\prime) + (xy^\prime + yx^\prime)i \\]

Division:

\\[\frac{z^\prime}{z} = \frac{x^\prime + y^\prime i}{x + yi} = \frac{xx^\prime + yy^\prime}{x^2 + y^2} + \frac{xy^\prime - yx^\prime}{x^2 + y^2} i\\]

An alternative way to write a complex number is

\\[z = r \cos(\theta) + ir \sin(\theta)\\]

Where \\(r = \sqrt{x^2 + y^2}\\) and \\(\theta = \arctan(y/x)\\). This introduces
a very important idea in complex numbers. We can actually imagine plotting a complex
number on a grid with the real numbers along the \\(x\\) axis and the imaginary
numbers along the \\(y\\) axis. This allows us to represent rotations easily,
as we shall see. (This kind of plotting is sometimes called an Argand diagram.)

When representing complex numbers in this polar way we have analogues to \\(\Re\\)
and \\(\Im\\)

\\[\text{mod}(z) = \lvert z \rvert = r\\]

\\[\arg(z) = \theta \\]



## Using Complex Numbers for Rotations

### Some Preliminary Work

#### Rotation on the Argand Diagram

Something useful about the Argand diagram representation of complex numbers is 
adding 1 to \\(z\\) is equivalent to moving one step to the right, in the \\(x\\)
axis. Adding \\(i\\) moves one step up, in the \\(y\\) axis. Multiplication by 
a scalar affects both the real and imaginary parts; if you view the complex number 
as a kind of vector this would mean a multiplication by 2 would double the length.

Multiplication by \\(i\\) is particularly interesting, so let's work through it 
a little slower. If we consider 

\\[z = 1 + i \\]

Then \\(\Re(z) = 1\\) and \\(\Im(z) = 1\\). If we multiply by \\(i\\) then

\\[iz = i + i^2 = -1 + i\\]

So now \\(\Re(z) = -1\\) and \\(\Im(z) = 1\\), which you might notice is
equivalent to a \\(\pi / 2\\) rotation counterclockwise i.e. in the positive
direction.

Two interesting results which will be helpful are that 
\\(\Re(zz^\prime) = \Re(z)\Re(z^\prime) - \Im(z)\Im(z^\prime) \\) and
\\(\Im(ab) = \Re(z) \Im(z^\prime) + \Im(z)\Re(z^\prime)\\). Let's prove this

We know that 

\\[zz^\prime = (xx^\prime - yy^\prime) + (xy^\prime + yx^\prime)i \\]

So 

\\[ \Re(zz^\prime) = xx^\prime - yy^\prime \\]

and trivially

\\[\Re(z)\Re(z^\prime) - \Im(z)\Im(z^\prime) = xx^\prime - yy^\prime \\]

Similarly 

\\[\Im(zz^\prime) = xy^\prime + yx^\prime\\]

and trivially

\\[ \Re(z) \Im(z^\prime) + \Im(z)\Re(z^\prime) = xy^\prime + yx^\prime \\]

#### The Complex Conjugate

The next interesting definition we will use is the idea of a complex conjugate,
which is defined as the reflection of a complex number in the real axis (the
\\(x\\) axis in the Argand diagram). It is very simply defined, for 
\\(z = x + iy\\) as 

\\[z^* = x - iy\\]

The complex conjugate also itself has a rather interesting result in that 
if \\(z,a,b\\) are all complex and \\(z = ab\\) then \\(z^* = a^* b^*\\). 
Let's say \\(z = x + iy\\), \\(a = p + iq \\) and \\(b = r + si\\).

So

\\[ z = ab = (pr - qs) + (ps + qr)i \\]

Therefore

\\[ z^* = (pr - qs) - (ps + qr)i \\]

Now we can easily say that

\\[a^* = p - qi\\]

\\[b^* = r - si\\]

So

\\[a^* b^* = (pr - qs) + (-ps - qr) i = (pr - qs) - (ps  qr) i\\]

Therefore

\\[z^* = a^* b^* \\]

The next interesting result is that \\(\Re(z) = \frac{1}{2}(z + z^*)\\)
and \\( \Im(z) = \frac{1}{2i}(z - z^*) \\).

To prove these just notice that 

\\[z + z^* = x + iy + x - iy = 2x\\]

So \\(\Re(z) = \frac{1}{2}(z + z^*)\\)

Similarly 

\\(z - z^* = x + iy - x + iy = iy\\)

Therefore \\( \Im(z) = \frac{1}{2i}(z - z^*) \\)

#### Distance and Bearing on Argand Diagram

Since the Argand diagram can be seen as a way of representing the complex number
as a vector, the length of the complex number on this diagram is given easily 
by 

\\(r = \sqrt{x^2 + y^2}\\)

Which as we have seen is also called \\(\text{mod}(z)\\). Something interesting
we can do with the conjugate here is 

\\[ zz^* = (x + iy)(x-iy) \\]

\\[zz^* = x^2 + iyx -ixy -i^2y^2 \\]

\\[zz^* = x^2 + y^2 = \text{mod}(z)^2\\]

So \\(zz^*\\) is often called the modulus square, for obvious reasons. 

Another useful consequence is if we look at the alternative way of writing a 
complex number of \\(z = r \cos(\theta) + ir \sin(\theta)\\) where 
\\(r = \sqrt{x^2 + y^2}\\) and \\(\theta = \arctan(y/x)\\), then we can see 
pretty simply that \\(\Re(z) = \text{mod}(z) \cos(\theta)\\) and
\\(\Im{z} = \text{mod}(z) \sin(\theta)\\).

#### Complex Numbers as Series 

Proof of the following facts are beyond the scope of this blog, but I will do 
one on infinite series in future to prove them. However

\\[ e^x = 1 + x + \frac{x^2}{2!} + \frac{x^3}{3!} + \cdots \\]

And also

\\[ \cos(\theta) = 1 - \frac{\theta^2}{2!} + \frac{\theta^4}{4!} - \frac{\theta^6}{6!} + \cdots \\]

\\[ \sin(\theta) = \theta - \frac{\theta^3}{3!} + \frac{\theta^5}{5!} - \frac{\theta^7}{7!} + \cdots \\]

If we substitute \\(i \theta\\) into \\(e^x\\) we get

\\[
e^{i \theta} =
1 + i \theta +
\frac{(i \theta)^2}{2!} +   
\frac{(i \theta)^3}{3!} +   
\frac{(i \theta)^4}{4!} +   
\frac{(i \theta)^5}{5!} + \cdots
\\]

Which if we handle the exponents properly (recall that \\(i^2 = -1\\))

\\[
e^{i \theta} =
1 + i \theta +
\frac{\theta^2}{2!} -
\frac{i \theta^3}{3!} +   
\frac{\theta^4}{4!} +   
\frac{i \theta^5}{5!} + \cdots
\\]

Which if we rewrite it in a more serendipitous form of

\\[
e^{i \theta} = 
\left(
1 -
\frac{\theta^2}{2!} +
\frac{\theta^4}{4!} + \cdots
\right) +
i \left( 
\theta -
\frac{\theta^3}{3!} +
\frac{\theta^5}{5!} + \cdots 
\right) =
\cos(\theta) + i \sin(\theta)
\\]

Which is truly an extraordinary result! 

### Representing Oscillations with Complex Numbers

Recall that an oscillation can be represented by 

\\[y = A \sin(\omega t + \phi) \\]

Where \\(A\\) is the amplitude of the wave, and \\(\omega \\) is the angular 
frequency of the wave, and \\(\phi\\) is the phase shift. 
The angular frequency is related to the frequency \\(f\\)
and the period \\(T\\) by

\\[\omega = 2 \pi f\\]
\\[\omega = \frac{2\pi}{T}\\]

So, using the results we derived we can actually rewrite this as

\\[y = \Re(A e^{i (\omega t + \phi)})\\]

Which in turn can be written as

\\[y = \Re(A e^{i \phi} e^{i \omega t})\\]

but we can also define the "complex amplitude" as \\(B = A e^{i \phi}\\) then

\\[y = \Re(B e^{i\omega t})\\]

Okay, while this is all good you might be wondering *why bother* going 
through all this complex rigamarole when we already had a perfectly 
functional equation for oscillations?

Well for one, adding together waves is now much easier. Suppose that

\\[y = A \cos(\omega t) + B \cos(\omega t + \phi)\\]

Trigonometric identities could simplify this, but what about just saying

\\[
y = 
\Re(A e^{i \omega t}) + \Re(B e^{i(\omega t + \phi)}) = 
\Re((A + Be^{i\phi}) e^{i\omega t})
\\]

Also derivatives are now much easier, so calculating the speed of a wave is 
basically trivial

\\[\frac{d}{dt} e^{i\omega t} = i \omega e^{i \omega t}\\]

## Using Complex Numbers for Alternating Current

In high school physics courses we only ever really look at d.c current, and if 
a.c. is discussed at all, it's only ever really in concept and not a strenuous 
mathematical analysis. Now that we are armed with the knowledge of complex
numbers, we can explore it a little.

An a.c voltage and current is given by

\\[V = V_0 \cos(\omega t)\\]

\\[I = I_0 \cos(\omega t + \phi) \\]

Two things to note: the \\(\phi\\) means that the current might be out of phase
to the voltage. Secondly, when dealing with circuits the human race has unfortunately 
confused itself to no end. Sometimes \\(j\\) is used rather than \\(i\\) for the 
imaginary number, as \\(I\\) is often used for current. However, lots of
people also use \\(j\\) for current.... In this article I will make use of the 
obvious solution to this "problem" by simply making current \\(I\\) and the 
imaginary number \\(i\\), which, are the standard symbols *anyway*!

Okay so we remember how to write that as a complex number:

\\[V = \Re(V_0 e^{i \omega t})\\]

\\[I = \Re(I_0 e^{i(\omega t + \phi)}) = \Re(I_0 e^{i \phi} e^{i \omega t})\\]

We can define a complex amplitude as 

\\[J_0 = I_0 e^{i \phi}\\]

Which makes it even simpler, as \\(J_0\\) contains the amplitude and phase 
information, so 

\\[I = \Re(J_0 e^{i \omega t}) \\]

We can then define a property called *impedance* which is defined as

\\[Z = \frac{V_0}{J_0}\\]

Where the modulus of impedance gives the ratio of the amplitudes of the voltage
and current waves, while the argument of impedance gives the phase difference 
between the two waves. Because of this, it is also sometimes helpful to write 
impedance in the more standard complex number form

\\[Z = R + iX\\]

Where \\(R\\) is, as we said, the amplitudes of the voltage and current waves. 
I serendipitously write this as \\(R\\) because, as you will soon see, this 
*is* the resistance. As you might recall from d.c. circuits \\(V = IR\\) is 
Ohm's law. \\(X\\) is called the reactance, and as we said, that's the phase 
difference between the two waves. Both \\(R\\) and \\(X\\) are real numbers.

If we consider the resistor, Ohm's law again tells us that \\(V = IR\\), and 
so if \\(I = I_0 \cos(\omega t)\\) then \\(V = I_0 R \cos(\omega t)\\). In such 
a scheme, the phase difference is 0. Therefore, \\(J_0 = I_0\\), and so 
\\(Z = V / I = R\\). So indeed the real part of the impedance is literally 
the resistance from Ohm's law.

We won't cover anymore of a.c. circuits, everything else works by the same 
kind of analogy with d.c. circuits (since this isn't a circuits blog but a 
complex number blog!)

## Conclusion

This blog should hopefully have gotten you pretty comfortable with complex numbers.
Furthermore, my main hope is to show you that the "imaginary" number is not something
fake, or made-up or indeed imaginary in any way! It has genuine, real connections 
to very real and physical phenomena: it is just as real in that sense as any 
natural number. It's really just a quirk of history that it was given this
unfortunate name.

## References

Cullerne, J., Machacek, A. (2008). *The Language of Physics: A Foundation for University Study*. Oxford University Press

Riley, K., F. (1974). *Mathematical Methods for the Physical Sciences: An Informal Treatment for Students of Physics and Engineering*. Cambridge University Press

Boas, M., L. (2005). *Mathematical Methods in the Physical Sciences (3rd ed.)*. Wiley