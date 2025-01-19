In a [previous blog entry](/blog/2025-01-04/kalman-filtering-falling-object)
we looked at tracking a falling object. That was focused on the practical 
implementation. This time we will cover tracking an object moving at constant 
velocity but focus more on the actual derivation of the filter. 
Let's start with the differential state-space equations.

\\[
\begin{bmatrix}
\dot{x} \\\\
\ddot{x}
\end{bmatrix} =
\begin{bmatrix}
0 & 1 \\\\
0 & 0
\end{bmatrix}
\begin{bmatrix}
x \\\\
\dot{x}
\end{bmatrix} +
\begin{bmatrix}
0 \\\\
u_s
\end{bmatrix}
\\]

Where \\(u_s\\) is the white noise parameter. Why is \\(u_s\\) where it is? 
In effect we want to be modelling noise that the Kalman filter might not be 
taking into account. Since the position is dependent on the velocity, if you put 
the noise on the velocity parameter we can propagate this up to the position 
parameter.

Now we have the system dynamics matrix \\(\mathbf{F}\\) given by

\\[
\mathbf{F} =
\begin{bmatrix}
0 & 1 \\\\
0 & 0
\end{bmatrix}
\\]


And we can use this to find the fundamental matrix \\(\Phi\\) by
use of the Taylor series

\\[
\Phi(t) = e^{\mathbf{F}t} =
\mathbf{I} + \mathbf{F}t + \frac{(\mathbf{F}t)^2}{2!} + \dots + 
\frac{(\mathbf{F}t)^n}{n!} + \dots
\\]

Thankfully this is actually very easy to solve since

\\[\mathbf{F}^2 =
\begin{bmatrix}
0 & 0 \\\\
0 & 0
\end{bmatrix}
\\]

and so all higher order terms in the expansion are actually zero
matricies too! Therefore, the fundamental matrix is given by


\\[
\Phi(t) = e^{\mathbf{F}t} =
\begin{bmatrix}
1 & 0 \\\\
0 & 1
\end{bmatrix} +
\begin{bmatrix}
0 & 1 \\\\
0 & 0
\end{bmatrix} t =
\begin{bmatrix}
1 & t \\\\
0 & 1
\end{bmatrix}
\\]


Note that while we derived this, in practice normally the fundamental matrix 
can be derived by intuition alone. The fundamental matrix answers the question 
of how our state moves forward by one time step.

Now in this instance \\(\mathbf{H}\\) is also simple, and we can find it from 
intuition. The only measurement we will get will be position, but our state 
contains a position and a velocity. How can we extract just a position from 
that? It's clearly


\\[ \mathbf{H} = [1, 0] \\]

\\(\mathbf{R}\\) is also simple in this case

\\[\mathbf{R} = [\sigma^2] \\]


Where \\(\sigma^2\\) is the standard deviation of our noise parameter.
In our simulation this will be some known constant, but in real life you will be 
having to guess this. Perhaps the manufacturer of the measurement device will 
provide some constant, or perhaps the device will report a reading and an 
uncertainty. You might just have to guess it based on experiments.

Now we want to find \\(\mathbf{Q}_k\\). We already said \\(\mathbf{w}\\) was

\\[\mathbf{w} = [0, u_s]^T \\]


\\(\mathbf{Q}\\) is given by

\\[\mathbf{Q} = E(\mathbf{ww}^T)\\]

So the continuous process noise is
            
\\[
\mathbf{Q}(t) =
\begin{bmatrix}
0 & 0 \\\\
0 & \Phi_s
\end{bmatrix}
\\]

And to find the discrete process noise from the continuous
process noise we use
            
\\[\mathbf{Q}_k = \int^{dt}_0 \Phi(\tau) \mathbf{Q}(\tau) \Phi^T(\tau) d \tau\\]

\\[
\mathbf{Q}_k = \int^{dt}_0
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
d \tau =
\int^{dt}_0
\Phi_s
\begin{bmatrix}
0 & \tau \\\\
0 & 1
\end{bmatrix}
\begin{bmatrix}
1 & 0 \\\\
\tau & 1
\end{bmatrix}=
\int^{dt}_0
\Phi_s
\begin{bmatrix}
\tau^2 & \tau \\\\
\tau & 1
\end{bmatrix}
\\]

\\[\mathbf{Q}_k = \Phi_s
\begin{bmatrix}
(dt)^3 / 3 & (dt)^2 / 2 \\\\
(dt)^2 / 2 & dt
\end{bmatrix}\\]

            
Great, but what is \\(\Phi_s\\)? Well, that's a parameter which represents the 
process noise, and the only real way to derive it is by trial and error. 
You simply need to try multiple values and see what allows the filter to 
stabalise. There is no mathematical way to really derive it as such. Finding 
\\(\Phi_s\\) is much more an art than a science. You should be weary of 
overfitting though - it's entierly possible that you might choose a value of 
\\(\Phi_s\\) which looks great in one simulation but totally falls apart in the 
real world.
            
<center>
    <strong>DO NOT EVER TRY AND "TUNE" A KALMAN FILTER ON A
        SINGLE DATA SET</strong>
</center>
     
Now, all the pieces are in place. We can solve the Riccati equations and 
produce the final Kalman filter
            
\\[ M_k = \Phi_k P_{k-1} \Phi^T_k + \mathbf{Q}_k \\]

\\[ K_k = M_k \mathbf{H}^T [\mathbf{H} M_k \mathbf{H}^T + \mathbf{R}_k]^{-1} \\]

\\[\bar{\dot{x}} = \hat{\dot{x}}_{k-1}\\]

\\[\bar{x} = \hat{x}_{k-1} + \bar{\dot{x}} dt\\]

\\[\tilde{x} = x^* - \bar{x}\\]

\\[\hat{\dot{x}} = \bar{\dot{x}} + K_2 x^*\\]

\\[\hat{x} = \bar{x} + K_1 x^*\\]

\\[ P_k = (\mathbf{I} - K_k \mathbf{H}) M_k \\]

            
I won't write out the code in full in this article, since I already did that in 
the last article, and this is a simpler implementation. Don't forget to look at 
the [companion repository](https://github.com/IndigoCurnick/kalman-filtering-rs)
for the full code!

I was able to get the following results for my implementation, which are really 
good!

<div id="position-plot" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="velocity-plot" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="position-residual" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="velocity-residual" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<script id="KalmanFilterScripts" src="/blog-assets/2025-01-11-kalman-filter-constant-velocity/plots.js"></script>

## References 

Zarchan, P., Musoff, H. (2009) *Fundamentals of Kalman Filtering: A Practical Approach (3rd Ed.)*. 
American Institude of Aeronautics and Astronautics

Tziallas, G., Adam, M., Assimakis, N., Polyzos, A. (2021) *Position, Velocity and Acceleration Tracking Using Kalman Filter*.
B P International