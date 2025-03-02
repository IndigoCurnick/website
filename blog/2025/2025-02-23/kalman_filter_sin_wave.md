
\\[ \newcommand{\Lagr}{\mathscr{L}} \\]


The sin wave tracking problem presents us with a unique opportunity.
There are many possible solutions to this problem, some linear and some 
non-linear. This allows us to see the effect of different kinds of Kalman 
filters in action

First, let's just quickly write a simple method which generates data which we 
can use in all the examples for the sin wave.


<pre>
<code>
pub fn get_data() -> Data {
    let mut t_hist = vec![];
    let mut y_hist = vec![];
    let mut y_m = vec![];
    let mut t = 0.0;

    let normal = Normal::new(0.0, R).unwrap();
    let mut rng = rand::thread_rng();

    for _ in 0..1000 {
        let y = A * (OMEGA * t).sin();

        t_hist.push(t);
        y_hist.push(y);
        y_m.push(y + normal.sample(&mut rng));

        t += TS;
    }

    return Data {
        t: t_hist,
        y: y_hist,
        y_m: y_m,
    };
}

pub struct Data {
    pub t: Vec&lt;f64>,
    pub y: Vec&lt;f64>,
    pub y_m: Vec&lt;f64>,
}
</code>
</pre>

Just like in our other problems, this allows us to test against noisy 
measurements. Recall that

\\[ x = \sin(\omega t) \\]

## Linear First Order

We begin with the simplest case for the sin wave tracking problem - linear and 
first order tracking. We understand that the measurements will be given by

\\[ x^* = \sin(\omega t) + \nu \\]

Where \\(\nu\\) is some noise. Looking at the derivatives we have

\\[ x = \sin(\omega t) \\]
\\[ \dot{x} = \sin(\omega t) \\]

We've already handled linear first order problems before, so I won't spend too 
much time on the details. Suffice to say, we've already seen all of the relevant 
matricies before.

\\[
\mathbf{\Phi}_k =
\begin{bmatrix}
1 & dt \\\\
0 & 1
\end{bmatrix}
\\]

\\[
\mathbf{H} =
\begin{bmatrix}
1 & 0
\end{bmatrix}
\\]

\\[
\mathbf{Q}_k =
\Phi_s
\begin{bmatrix}
\frac{dt^3}{3} & \frac{dt^2}{2} \\\\
\frac{dt^2}{2} & dt
\end{bmatrix}
\\]

With that, we can define

\\[\tilde{x}_k = x^*\_k - \hat{x}\_{k-1} - dt \hat{\dot{x}}\_{k-1} \\]

So that we can use the following solutions

\\[ \hat{x}\_k = \hat{x}\_{k-1} + dt \hat{\dot{x}}\_{k-1} + K\_1 \tilde{x}\_k \\]

\\[ \hat{\dot{x}} = \hat{\dot{x}}\_{k-1} + K\_2 \tilde{x}\_k \\]

And we get the following results which does show the Kalman filter is able to 
somewhat track the wave. It's able to remove quite a lot of the jaggedness and 
smooth the data, but the residuals show that it isn't a major improvement in the 
actual accuracy. Specifically, it has a significant lag.

<div id="full-plot-linear-first-order" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="residual-linear-first-order" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<script id="LinearFirstOrderKalmanFilterScripts" src="/blog-assets/2025-02-22-kalman-filter-sin-wave/linear_first_order.js"></script>

## Linear Second Order

Let's now try a linear second order filter. Again, we've see the
derivation of a second order filter before, so I'll just remind
you of the relevant results.

\\[
\mathbf{\Phi}\_k =
\begin{bmatrix}
1 & dt & 0.5 dt^2 \\\\
0 & 1 & dt \\\\
0 & 0 & 1
\end{bmatrix}
\\]

\\[
\mathbf{H} =
\begin{bmatrix}
1 & 0 & 0
\end{bmatrix}
\\]

\\[
\mathbf{Q}_k =
\Phi_s
\begin{bmatrix}
\frac{dt^5}{20} & \frac{dt^4}{8} & \frac{dt^3}{6} \\\\
\frac{dt^4}{8} & \frac{dt^3}{3} & \frac{dt^2}{2} \\\\
\frac{dt^3}{6} & \frac{dt^2}{2} & dt
\end{bmatrix}
\\]

We can then define

\\[ \tilde{x} = x^*\_k - \hat{x}\_{k-1} - dt \hat{\dot{x}}\_{k-1} - 0.5
dt^2 \hat{\ddot{x}}\_{k-1} \\]

So now we can solve the equations to give

\\[ \hat{x} = \hat{x}\_{k-1} + dt \hat{\dot{x}}\_{k-1} + 0.5 dt^2 \hat{\ddot{x}}\_{k-1} + K_1 \tilde{x} \\]

\\[ \hat{\dot{x}} = \hat{\dot{x}}\_{k-1} + dt \hat{\ddot{x}}\_{k-1} + K\_2 \tilde{x} \\]

\\[ \hat{\ddot{x}} = \hat{\ddot{x}}\_{k-1} + K_3 \tilde{x} \\]

Which gives the following results, which are actually slightly worse. You might 
be able to tune some of the spectral density parameters on \\(\mathbf{Q}_k\\) 
to get slightly better results, but the filter is overshooting by a lot.

<div id="full-plot-linear-second-order" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="residual-linear-second-order" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<script id="LinearSecondOrderKalmanFilterScripts" src="/blog-assets/2025-02-22-kalman-filter-sin-wave/linear_second_order.js"></script>

## Linear Filter with A Priori Information

In this variation, we're going to help the filter out by giving it some 
additional information. We'll have a slightly different state matrix. Let's 
start by remembering the fundamentals of oscillations.

\\[ x = A \sin(\omega t) \\]

\\[ \dot{x} = A \omega \cos(\omega t) \\]

\\[ \ddot{x} = -A \omega^2 \sin(\omega t) \\]

Which means we can rewrite the second derivative using the first equation like 
so

\\[ \ddot{x} = -\omega^2 x \\]

Using this, we can form a new state-space equation

\\[
\begin{bmatrix}
\dot{x} \\\\
\ddot{x}
\end{bmatrix} =
\begin{bmatrix}
0 & 1 \\\\
-\omega^2 & 0
\end{bmatrix}
\begin{bmatrix}
x \\\\
\dot{x}
\end{bmatrix}
\\]

By inspection we can see that

\\[
\mathbf{F} =
\begin{bmatrix}
0 & 1 \\\\
-\omega^2 & 0
\end{bmatrix}
\\]

Unfortunately, in this case the derivations are not so simple of the \\(\Phi\\) 
matrix. We know that

\\[ \mathbf{\Phi}(t) = \Lagr ((\mathbf{s} \mathbf{I} - \mathbf{F}^{-1})) \\]

The Laplace transform is tricky, but we can start with the matrix manipulation

\\[ \mathbf{s} \mathbf{I} =
\begin{bmatrix}
s & 0 \\\\
0 & s
\end{bmatrix}
\\]

\\[
\mathbf{s} \mathbf{I} - \mathbf{F} =
\begin{bmatrix}
s & -1 \\\\
\omega^2 & s
\end{bmatrix}
\\]

We can now inverse this to give


\\[
(\mathbf{s} \mathbf{I} - \mathbf{F})^{-1} =
\frac{1}{s^2 + \omega^2}
\begin{bmatrix}
s & 1 \\\\
-\omega^2 & s
\end{bmatrix}
\\]

My primary reference on this section
_Fundamnetals of Kalman Filtering: A Practical Approach 3rd Edition_ by Paul 
Zarchan, Howard Musoff and Frank K. Lu lists the solution to the inverse as
\\( \frac{1}{s^ + \omega^2} \begin{bmatrix} s & 1 \\ -\omega^2 & 1 \end{bmatrix} \\),
however, by the standard convention of matrix inverse, you get my solution with 
the \\(s\\) in the bottom right hand corner. WolframAlpha agrees with my 
derivation, so I will list it as the correct version here. I think that the 
final Laplace transform matrix for \\(\Phi(t)\\) is correct as given, but I will 
continue to investigate. Many people online agree on \\(\Phi(t)\\) for this 
problem, but maybe they are just using Zarchan as reference without digging 
deeper. I will try and keep this up to date if I find any more information
on this.

The Laplace itself is tricky. For most real world problems, the Laplace will be 
too challenging to actually perform by itself. Typically, we would use reference 
tables to perform the Laplace transform. A useful book to have a copy of for 
this purpose if you plan on making original Kalman filters is 
_CRC Standard Mathematical Tables and Formulas_ 33rd Edition by Dan Zwillinger

\\[ \Phi(t) =
\begin{bmatrix}
\cos(\omega t) & \sin(\omega t) / \omega \\\\
-\omega \sin(\omega t) & \cos(\omega t)
\end{bmatrix}
\\]

We are now in a position to solve the Riccati equations. We define a residual as

\\[\tilde{x} = x^*\_k - \cos(\omega dt) \hat{x}\_{k-1} - \frac{\sin(\omega dt)}{\omega} \hat{\dot{x}}\_{k-1} \\]

And the solutions as

\\[ \hat{x}\_k = \cos(\omega dt) \hat{x}\_{k-1} + \frac{\sin(\omega dt)}{\omega} \hat{\dot{x}}\_{k-1} + K\_1 \tilde{x} \\]

\\[ \hat{\dot{x}}\_k = -\omega \sin(\omega dt) \hat{x}\_{k-1} + \cos(\omega dt) \hat{\dot{x}}\_{k-1} + K_2 \tilde{x} \\]

We get the following results for this solution which are really quite good. The 
data is smooth and the residuals are consistently small!

<div id="full-plot-linear-a-priori" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="residual-linear-a-priori" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<script id="LinearAPrioriKalmanFilterScripts" src="/blog-assets/2025-02-22-kalman-filter-sin-wave/linear_a_priori.js"></script>

## Extended Kalman Filter Solution

Since this variation is non-linear, we can expect to have a non-linear state. 
Returning to the equations for oscillation again

\\[x = A \sin(\omega t)\\]

We bring back the amplitude \\(A\\) for this section. We define a new variable 
\\(\theta = \omega t\\). If the frequency is constant (which we will assume so 
for this example) then \\( \dot{\theta} = \omega\\) and \\( \dot{\omega} = 0 \\). 
We will also assume constant amplitude, so \\( \dot{A} = 0\\). Therefore, the 
state-space equations are given by

\\[
\begin{bmatrix}
\dot{\theta} \\\\
\dot{\omega} \\\\
\dot{A}
\end{bmatrix} =
\begin{bmatrix}
0 & 1 & 0 \\\\
0 & 0 & 0 \\\\
0 & 0 & 0
\end{bmatrix}
\begin{bmatrix}
\theta \\\\
\omega \\\\
A
\end{bmatrix} +
\begin{bmatrix}
0 \\\\
u_{s1} \\\\
u_{s2}
\end{bmatrix}
\\]

This state-space equation is interesting in the noise component. We introduce 
two different spectral densities. This is because the noise in \\(\omega\\) 
might not be the same as the noise in \\(A\\). In general 
\\(u\_{s1} \neq u\_{s2}\\). This imapcts how we derive \\(\mathbf{Q}\_k\\)
slighlty.

\\[
\mathbf{Q} =
\begin{bmatrix}
0 & 0 & 0 \\\\
0 & \Phi_{s1} & 0 \\\\
0 & 0 & \Phi_{s2}
\end{bmatrix}
\\]

We can continue the derivation of \\(\mathbf{Q}_k\\) after finding the
fundamental matrix.

\\[
\mathbf{F} =
\begin{bmatrix}
0 & 1 & 0 \\\\
0 & 0 & 0 \\\\
0 & 0 & 0
\end{bmatrix}
\\]

Thankfully, this is a quite easy to derive the fundamental matrix in this 
instance because

\\[
\mathbf{F}^2 =
\begin{bmatrix}
0 & 0 & 0 \\\\
0 & 0 & 0 \\\\
0 & 0 & 0
\end{bmatrix}
\\]

Therefore, we can use the Taylor expansion to find the fundamental matrix

\\[
\mathbf{\Phi}(t) = \mathbf{I} + \mathbf{F} t =
\begin{bmatrix}
1 & t & 0 \\\\
0 & 1 & 0 \\\\
0 & 0 & 1
\end{bmatrix}
\\]

\\[
\mathbf{Q}\_k = \int^{T\_s}\_0 \mathbf{\Phi} (\tau) \mathbf{Q}
\mathbf{\Phi}^T (\tau) d \tau
\\]

\\[
\mathbf{Q}\_k = \int^{T\_s}\_0
\begin{bmatrix}
1 & \tau & 0 \\\\
0 & 1 & 0 \\\\
0 & 0 & 1
\end{bmatrix}
\begin{bmatrix}
0 & 0 & 0 \\\\
0 & \Phi\_{s1} & 0 \\\\
0 & 0 & \Phi\_{s2}
\end{bmatrix}
\begin{bmatrix}
1 & 0 & 0 \\\\
\tau & 1 & 0 \\\\
0 & 0 & 1
\end{bmatrix}
d \tau
\\]

\\[
\mathbf{Q}\_k = \int^{T\_s}\_0
\begin{bmatrix}
\tau^2 \Phi\_{s1} & \tau \Phi\_{s1} & 0 \\\\
\tau \Phi\_{s1} & \Phi\_{s1} & 0 \\\\
0 & 0 & \Phi\_{s2}
\end{bmatrix}
d \tau
\\]

\\[
\mathbf{Q}\_k =
\begin{bmatrix}
\frac{\Phi\_{s1} dt^3}{3} & \frac{\Phi\_{s1} dt^2}{2} & 0 \\\\
\frac{\Phi\_{s1} dt^2}{2} & \Phi\_{s1} dt & 0 \\\\
0 & 0 & \Phi\_{s2} dt
\end{bmatrix}
\\]

Finally, coming to the actual non-linear part we will need to do partial 
derivatives. Since

\\[
\Delta x^* =
\begin{bmatrix}
\frac{\partial x}{\partial \theta} \frac{\partial x}{\partial
\omega} \frac{\partial x}{\partial A}
\end{bmatrix}
\begin{bmatrix}
\Delta \theta \\\\
\Delta \omega \\\\
\Delta A
\end{bmatrix} + \nu
\\]

Since \\(x = S \sin(\omega t) = A \sin(\theta) \\) then

\\[ \frac{\partial x}{\partial \theta} = A \cos(\theta) \\]

\\[ \frac{\partial x}{\partial \omega} = 0 \\]

\\[ \frac{\partial x}{\partial A} = \sin(\theta) \\]

Which gives us \\(\mathbf{H}\\) as

\\[
\mathbf{H} =
\begin{bmatrix}
A \cos(\theta) & 0 & \sin{\theta}
\end{bmatrix}
\\]

We are finally in a position to begin solving the Riccati equations! Frist, we 
define the predicted values

\\[
\begin{bmatrix}
\bar{\theta}\_k \\\\
\bar{\omega}\_k \\\\
\bar{A}\_k
\end{bmatrix} =
\begin{bmatrix}
1 & dt & 0 \\\\
0 & 1 & 0 \\\\
0 & 0 & 1
\end{bmatrix}
\begin{bmatrix}
\hat{\theta}\_{k-1} \\\\
\hat{\omega}\_{k-1} \\\\
\hat{A}\_{k-1}
\end{bmatrix}
\\]

\\[ \bar{\theta}\_k = \hat{\theta}\_{k-1} + \hat{\omega}\_{k-1} dt \\]

\\[ \bar{\omega}\_k = \hat{\omega}\_{k-1} \\]

\\[ \hat{A}\_k = \hat{A}\_{k-1} \\]

Note, that it is these bar values which you will use for \\(\mathbf{H}\\)

We define a residual as

\\[ \tilde{x} = x^*\_k - \bar{A}\_k \sin(\bar{\theta}\_k) \\]

And the final solutions are

\\[ \hat{\theta}\_k = \bar{\theta}\_k + K\_1 \tilde{x} \\]

\\[ \hat{\omega}\_k = \bar{\omega}\_k + K\_2 \tilde{x} \\]

\\[ \hat{A}\_k = \bar{A}\_k + K_3 \tilde{x} \\]

Here's the results from this simulation which show that the extended Kalman 
filter is not necessarily better. The linear with a priori information did much 
better. In this case, mathematically, there are two different ways to calculate 
\\(x\\) from the state - either from \\(\phi\\) or \\(\omega\\). I added both but you 
can clearly see that \\(\phi\\) is better (try clicking on the \\(\omega\\) line in 
the legend to hide it!).

<div id="full-plot-non-linear" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="residual-non-linear" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<script id="NonLinearKalmanFilterScripts" src="/blog-assets/2025-02-22-kalman-filter-sin-wave/non_linear.js"></script>

## Extended Kalman Filter With A Priori Information

In this implementation of the filter we will inform it of the correct value of 
\\(A\\). Now, our state-space equation is

\\[
\begin{bmatrix}
\dot{\theta} \\\\
\dot{\omega}
\end{bmatrix} =
\begin{bmatrix}
0 & 1 \\\\
0 & 0
\end{bmatrix}
\begin{bmatrix}
\theta \\\\
\omega
\end{bmatrix} +
\begin{bmatrix}
0 \\\\
u_s
\end{bmatrix}
\\]

We've already derived \\(\mathbf{\Phi}_k\\) for this situation before, and it is 
given by

\\[
\mathbf{\Phi}_k =
\begin{bmatrix}
1 & dt \\\\
0 & 1
\end{bmatrix}
\\]

\\(\mathbf{Q}\\) is clearly given by

\\[
\mathbf{Q} =
\begin{bmatrix}
0 & 0 \\\\
0 & u_s
\end{bmatrix}
\\]

So we can derive \\(\mathbf{Q}_k\\) by

\\[
\mathbf{Q}_k = \int^{T\_s}\_0
\begin{bmatrix}
1 & \tau \\\\
0 & 1
\end{bmatrix}
\begin{bmatrix}
0 & 0 \\\\
0 & \Phi_s
\end{bmatrix}
\begin{bmatrix}
1 & 0 \\\\
\tau & 1
\end{bmatrix}
d \tau
\\]

\\[
\mathbf{Q}\_k = \int^{T\_s}\_0
\Phi_s
\begin{bmatrix}
\tau^2 & \tau \\\\
\tau & 1
\end{bmatrix}
\\]

\\[
\mathbf{Q}_k =
\Phi_s
\begin{bmatrix}
\frac{dt^3}{3} & \frac{dt^2}{2} \\\\
\frac{dt^2}{2} & dt
\end{bmatrix}
\\]

Again, our measurements are non-linear so

\\[
\Delta x^* =
\begin{bmatrix}
\frac{\partial x}{\partial \theta} & \frac{\partial x}{\partial
\omega}
\end{bmatrix}
\begin{bmatrix}
\Delta \theta \\\\
\Delta \omega
\end{bmatrix} + \nu
\\]

Performing the partial derivatives gives us

\\[ \frac{\partial x}{\partial \theta} = A \cos(\theta) \\]

\\[ \frac{\partial x}{\partial \omega} = 0 \\]

And so we have \\(\mathbf{H}\\)

\\[ \mathbf{H} = \begin{bmatrix} A \cos(\theta) & 0 \end{bmatrix} \\]

And so we have all the elements needed to solve the Riccati equations. We begin 
by defining the predictions

\\[
\begin{bmatrix}
\bar{\theta}\_k \\\\
\bar{\omega}\_k
\end{bmatrix} =
\begin{bmatrix}
1 & dt \\\\
0 & 1
\end{bmatrix}
\begin{bmatrix}
\hat{\theta}\_{k-1} \\\\
\hat{\omega}\_{k-1}
\end{bmatrix}
\\]

\\[ \bar{\theta}\_k = \hat{\theta}\_{k-1} + \hat{\omega}\_{k-1} dt \\]

\\[ \bar{\omega}\_k = \hat{\omega}\_{k-1} \\]

We define a residual as

\\[ \tilde{x} = x^*\_k - A \sin(\bar{\theta}\_k) \\]

Finally, we can give the solutions to the Riccati equations as

\\[ \hat{\theta}\_k = \bar{\theta}\_k + K\_1 \tilde{x} \\]

\\[ \hat{\omega}\_k = \bar{\omega}\_k + K\_2 \tilde{x} \\]

Here's the results from this filter which clearly shows that even with a priori 
information, the EKF struggles to be as smooth as the linear filter. Again, I 
included \\(x\\) calculated from both \\(\phi\\) and \\(\omega\\) - \\(\phi\\) 
is obviously better. (You can hide the \\(\omega\\) line to see the good results 
better)

<div id="full-plot-non-linear-a-priori" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="residual-non-linear-a-priori" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<script id="NonLinearAPrioriKalmanFilterScripts" src="/blog-assets/2025-02-22-kalman-filter-sin-wave/non_linear_a_priori.js"></script>

## Alternative EKF

In this Kalman filter we will use a very different state and state-space 
equation. This time, the state-space equation will itself be non-linear as the 
state will appear within it. Therefore, we won't be able to read \\(\mathbf{F}\\) 
directly from the state-space equation like we have been doing so far! Let's 
once more start with our basic equations

\\[ x = A \sin(\omega t) \\]

\\[ \dot{x} = A \omega \cos(\omega t) \\]

\\[ \ddot{x} = -A \omega^2 \sin(\omega t) \\]

\\[ \ddot{x} = -\omega^2 x \\]

And in this example, let's place the model noise in the derivative of the frequency.


\\[ \dot{\omega} = u\_s \\]

Thus the state-space equation is

\\[
\begin{bmatrix}
\dot{x} \\\\
\ddot{x} \\\\
\dot{\omega}
\end{bmatrix} =
\begin{bmatrix}
0 & 1 & 0 \\\\
-\omega^2 & 0 & 0 \\\\
0 & 0 & 0
\end{bmatrix}
\begin{bmatrix}
x \\\\
\dot{x} \\\\
\omega
\end{bmatrix} +
\begin{bmatrix}
0 \\\\
0 \\\\
u\_s
\end{bmatrix}
\\]

As I mentioned, notice how \\(\omega\\) appears within the state-space matrix? 
\\(\omega\\) is also a part of our state, so this state-space matrix is 
non-linear. As such, \\(\mathbf{F}\\) is not simply equal to the state-space 
matrix.

\\[
\mathbf{F} = \frac{\partial f(\mathbf{x})}{\partial \mathbf{x}} =
\begin{bmatrix}
\frac{\partial \dot{x}}{\partial x} & 
\frac{\partial \dot{x}}{\partial \dot{x}} & 
\frac{\partial \dot{x}}{\partial \omega} \\\\
\frac{\partial \ddot{x}}{\partial x} & 
\frac{\partial \ddot{x}}{\partial \dot{x}} & 
\frac{\partial \ddot{x}}{\partial \omega} \\\\
\frac{\partial \dot{\omega}}{\partial x} & 
\frac{\partial \dot{\omega}}{\partial \dot{x}} & 
\frac{\partial \dot{\omega}}{\partial \omega}
\end{bmatrix}
\\]

Which can be easily evaluated to

\\[
\mathbf{F} =
\begin{bmatrix}
0 & 1 & 0 \\\\
-\hat{\omega}^2 & 0 & -2 \hat{\omega} \hat{x} \\\\
0 & 0 & 0
\end{bmatrix}
\\]

Sadly, in this case an exact solution to \\(\mathbf{\Phi}_k\\) is impossible, 
so we will have to accept an approximation with the Taylor expansion.

\\[
\mathbf{\Phi}\_k \approx
\mathbf{I} + \mathbf{F} t =
\begin{bmatrix}
1 & dt & 0 \\\\
-\hat{\omega}^2 dt & 1 & -2 \hat{\omega} \hat{x} dt \\\\
0 & 0 & 1
\end{bmatrix}
\\]

To derive \\(\mathbf{Q}\\) we can again use that

\\[
\mathbf{Q}\_k = \int^{T_s}\_0
\mathbf{\Phi} (\tau) \mathbf{Q} \mathbf{\Phi}^T (\tau) d \tau
\\]

\\[
\mathbf{Q}_k = \int^{T_s}_0
\begin{bmatrix}
1 & dt & 0 \\\\
-\hat{\omega}^2 dt & 1 & -2 \hat{\omega} \hat{x} dt \\\\
0 & 0 & 1
\end{bmatrix}
\begin{bmatrix}
0 & 0 & 0 \\\\
0 & 0 & 0 \\\\
0 & 0 & \Phi_s
\end{bmatrix}
\begin{bmatrix}
1 & -\hat{\omega} \tau & 0 \\\\
\tau & 1 & 0 \\\\
0 & -2 \hat{\omega} \hat{x} \tau & 1
\end{bmatrix}
d \tau
\\]

\\[
\mathbf{Q}\_k = \int^{T\_s}\_0
\Phi\_s
\begin{bmatrix}
0 & 0 & 0 \\\\
0 & 4 \hat{\omega}^2 \hat{x}^2 \tau^2 & -2 \hat{\omega} \hat{x} \tau \\\\
0 & -2 \hat{\omega} \hat{x} \tau & 1
\end{bmatrix}
\\]

\\[
\mathbf{Q}_k =
\Phi_s
\begin{bmatrix}
0 & 0 & 0 \\\\
0 & \frac{4}{3} \hat{\omega}^2 \hat{x}^2 T^3_s & -\hat{\omega} \hat{x} T_s^2 \\\\
0 & -\hat{\omega} \hat{x} T^2_s & T_s
\end{bmatrix}
\\]

Surprisingly, our measurements are actually linear! So

\\[\mathbf{H} = \begin{bmatrix} 1 & 0 & 0 \end{bmatrix} \\]

An unfortunate complexity is that in order to derive the prediction states we 
have no closed form solution! We could use the \\(\mathbf{\Phi}\_k\\) like we 
normally do, but it's only an approximation. For use in the Kalman filter 
equations for \\(m\\) and \\(k\\) it is good enough, but to actually propagate the 
state it will introduce just far too much error into the system. Therefore, we 
will need to numerically integrate to do the predictions.

<pre>
<code>
fn project(x: f64, x_dot: f64, omega: f64, step: f64) -> (f64, f64) {
    let mut x_bar = x;
    let mut x_dot_bar = x_dot;
    let mut t = 0.0;
    while t <= TS {
        let x_dot_dot = -omega.powf(2.0) * x_bar;
        x_dot_bar = x_dot_bar + step * x_dot_dot;
        x_bar = x_bar + step * x_dot_bar;
        t = t + step;
    }

    return (x_bar, x_dot_bar);
}
</code>
</pre>

Above is my Rust code to do that. It takes in the state, and also a step size. 
I used a step size of 1e-5. It then numerically integrates up to `TS`, which I 
set to be 0.01. If you change that, expect to have to change the step size. 
We now have \\(\bar{x}_k\\) and \\(\bar{\dot{x}}_k\\). This is everything we 
need to solve the Riccati equations.


\\[ \tilde{x} = x^* - \bar{x}\_k \\]

\\[ \hat{x}\_k = \bar{x}\_k + K\_1 \tilde{x} \\]

\\[ \hat{\dot{x}} = \bar{\dot{x}}\_k + K_2 \tilde{x} \\]

\\[ \hat{\omega} = \hat{\omega}\_{k-1} + K_3 \tilde{x} \\]

The results are give as follows, which clearly show that this is a really 
interesting filter. It gets okay results, while not being super smooth, 
but it has quite a significant warm up time with respect to some of the other 
filters.

<div id="full-plot-alternative-non-linear" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="residual-alternative-non-linear" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<script id="AlternativeKalmanFilterScripts" src="/blog-assets/2025-02-22-kalman-filter-sin-wave/alternative.js"></script>

## References 

Zarchan, P., Musoff, H. (2009) *Fundamentals of Kalman Filtering: A Practical Approach (3rd Ed.)*. 
American Institude of Aeronautics and Astronautics