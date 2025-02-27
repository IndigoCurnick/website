In this blog we will track a falling object using a Kalman filter. Our goal will
be to create a simulation of a falling object, and filter that with the Kalman
filter. We'll plot the residuals and see how well we can reconstruct the original
signal from the noisey signal. We'll code this in Rust, which is a great choice
for Kalman filtering! If you don't use Rust, that's okay, you should find it quite
easy to convert into your language of choice.

For this problem, we will be tracking an object dropped from a height. Whenever 
I make Kalman filters I always like to think about two things - what will be in 
the state, and what measurements do I have access to?

In this problem we will be modelling the problem as constant acceleration. 
We will only have access to distance measurements. We can think of the 
measurement device as being a radar which gives us the distance the object is 
from the ground. The standard deviation of these measurements is \\(300m\\) 
(so, quite large). Let's also say that the object drops from an initial height 
of \\(130,000m\\) with an initial velocity of \\(-2,000\frac{m}{s}\\). Acceleration 
is a constant \\(-9.81\frac{m}{s^2}\\).

While acceleration is constant, and we know it, just for fun in this problem 
we're going to make the Kalman filter attempt to estimate the acceleration.

First, we will produce our simulation. For this, we start with the initial 
conditions and propagate the system forward. This will be the ideal case. 
For every ideal step, we will also make a measurement by applying some Gaussian 
noise to the real position. The goal of the exercise is ultimately to extract 
the original positions from the noisy positions.

For this problem I will make the time step \\(0.1s\\). At each time step we can 
work out the new velocity as

\\[v = u + g \cdot dt\\]

where \\(u\\) was the velocity at the last time step, \\(g\\) is the acceleration 
and \\(dt\\) is the time step.

We can then get the change in distance as

\\[\Delta s = 0.5 (u + v) dt\\]

This is all the physics we need to produce the simulation! It's
quite simple.

<pre><code>
const SIGNOISE: f64 = 300.0;
const PHIS: f64 = 0.001;
const TS: f64 = 0.1;
const INIT_S: f64 = 130_000.0;
const INIT_U: f64 = -2000.0;
const G: f64 = -9.81;
const MAXT: f64 = 60.0;

struct Data {
    pub s: Vec&lt;f64>, // True value distance
    pub x: Vec&lt;f64>, // Measurement distance
    pub t: Vec&lt;f64>, // Time
    pub v: Vec&lt;f64>, // velocity
}

fn get_data() -> Data {
    let mut s = INIT_S;
    let mut t = 0.0;
    // let mut u = ;
    let mut u = INIT_U;
    let g = G;
    let dt = TS;

    let mut s_history = vec![];
    let mut x_history = vec![];
    let mut t_history = vec![];
    let mut v_history = vec![];

    let normal = Normal::new(0.0, SIGNOISE).unwrap();
    let mut rng = rand::thread_rng();

    while t < MAXT {
        // Measurement

        s_history.push(s);
        t_history.push(t);
        v_history.push(u);
        x_history.push(s + normal.sample(&mut rng));

        // Propagate
        let v = u + g * dt;
        let d = 0.5 * (u + v) * dt;

        s += d;
        u = v;
        t += dt;
    }

    return Data {
        s: s_history,
        x: x_history,
        t: t_history,
        v: v_history,
    };
}
</code></pre>

I make use of the <code>rand</code> and <code>rand_distr</code>
crates to produce the random numbers.

Now that we have the data, let's try making the Kalman filter
proper. First, the state is going to be

\\[\mathbf{x} = [x, \dot{x}, \ddot{x}]^T \\]

So that's position above ground, velocity, and acceleration.
And our measurement will be

\\[\mathbf{z} = [x^*] \\]

In other words, we will only be measuring the position. Thus, the filter will be 
expected to infer both the velocity and the acceleration.

There are many ways to think about the transition between states. There are
ways of systematically deriving \\(\Phi_k\\) which is essential for trickier 
problems. However, in this case it is better to build up our intuition. 
What matrix would transform our state to the next iteration? We've almost 
already written it down from the SUVAT equations!

\\[ \Phi_k =
\begin{bmatrix}
1 & dt & \frac{1}{2} (dt)^2 \\\\
0 & 1 & dt \\\\
0 & 0 & 1
\end{bmatrix}
\\]

When in doubt, expand.

\\[
\begin{bmatrix}
x_k \\\\
\dot{x}\_k \\\\
\ddot{x}\_k
\end{bmatrix} =
\begin{bmatrix}
x_{k-1} + dt \dot{x}\_{k-1} + \frac{1}{2} (dt)^2 \ddot{x}\_{k-1} \\\\
\dot{x}\_{k-1} + dt \ddot{x}\_{k-1} \\\\
\ddot{x}\_{k-1}
\end{bmatrix} =
\begin{bmatrix}
1 & dt & \frac{1}{2} (dt)^2 \\\\
0 & 1 & dt \\\\
0 & 0 & 1
\end{bmatrix}
\begin{bmatrix}
x\_{k-1} \\\\
\dot{x}\_{k-1} \\\\
\ddot{x}\_{k-1}
\end{bmatrix}
\\]

Now, thinking about \\(H\\) - I try to think of this as how we can extract a 
measurement from our state. In this case our measurement is \\(x\\) and our state 
contains an \\(x\\) in the first row, so \\(H\\) is

\\[H = [1, 0, 0]\\]

Now to consider \\(\mathbf{Q}_k\\). The continuous noise matrix
looks like

\\[\mathbf{Q} =
\begin{bmatrix}
0 & 0 & 0 \\\\
0 & 0 & 0 \\\\
0 & 0 & \Phi_s
\end{bmatrix} =
\Phi_s
\begin{bmatrix}
0 & 0 & 0 \\\\
0 & 0 & 0 \\\\
0 & 0 & 1
\end{bmatrix}
\\]

Where \\(\Phi_s\\) is the white noise parameter. Part of the art of the Kalman 
filter is picking this value. Later on, we will study deriving this in more 
serious detail, but for now, build intuition. In our model, the noise is all in
the acceleration, which is then propagated up to the higher order terms. The 
discrete noise model is always given by

\\[\mathbf{Q}_k = \int^{dt}_0 \Phi_k \mathbf{Q} \Phi_k^T dt^\prime \\]

In this case, \\(\mathbf{Q}_k\\) is given by

\\[\mathbf{Q}_k =
\Phi_s
\begin{bmatrix}
\frac{(dt)^5}{20} & \frac{(dt)^4}{8} & \frac{(dt)^3}{3} \\\\
\frac{(dt)^4}{8} & \frac{(dt)^3}{3} & \frac{(dt)^2}{2} \\\\
\frac{(dt)^3}{6} & \frac{(dt)^2}{2} & dt
\end{bmatrix}
\\]

Now, we have everything we need to start filtering! We now simply solve the 
Riccati equations.

\\[ \bar{\ddot{x}} = \hat{\ddot{x}}\_{k-1} \\]
\\[ \bar{\dot{x}} = \hat{\dot{x}}\_{k-1} + dt \hat{\ddot{x}}\_{k-1} \\]
\\[ \bar{x} = \hat{x}\_{k-1} + dt \hat{\dot{x}}\_{k-1} + 0.5 (dt)^2 \hat{\ddot{x}}\_{k-1} \\]

Remember that

\\[ M_k = \Phi_k P_{k-1} \Phi^T_k + \mathbf{Q}_k \\]
\\[ K_k = M_k H^T [H M_k H^T + R_k]^{-1} \\]

We also define

\\[ \tilde{x} = x^* - \bar{x} \\]

And so

\\[ \hat{x}_k = \bar{x}_k + K_1 \tilde{x} \\]
\\[ \hat{\dot{x}}_k = \bar{\dot{x}}_k + K_2 \tilde{x} \\]
\\[ \hat{\ddot{x}}_k = \bar{\ddot{x}}_k + K_3 \tilde{x} \\]

So, now to put this into code

<pre><code>
let data = get_data();

let mut state = matrix(vec![0.0, 0.0, 0.0], 3, 1, Row);

let mut cov = zeros(3, 3);
cov[(0, 0)] = 99999999.0;
cov[(1, 1)] = 99999999.0;
cov[(2, 2)] = 99999999.0;

let h = matrix(vec![1.0, 0.0, 0.0], 1, 3, Row);
let r = matrix(vec![SIGNOISE], 1, 1, Row);

let mut x_history = vec![];
let mut v_history = vec![];
let mut a_history = vec![];

let mut x_residual = vec![];
let mut v_residual = vec![];
let mut a_residual = vec![];

let mut x_measurement_residual = vec![];

for i in 0..data.t.len() {
    let x_star = data.x[i];

    let xkminus1 = state.data[0];
    let xdotkminus1 = state.data[1];
    let xdotdotkminus1 = state.data[2];

    let xdotdot_bar = xdotdotkminus1;
    let xdot_bar = xdotkminus1 + xdotdotkminus1 * TS;
    let x_bar = xkminus1 + xdotkminus1 * TS + 0.5 * xdotdotkminus1 * TS.powf(2.0);

    let phi = phi(TS);
    let q = q(TS);
    let m = make_m(&phi, &cov, &q);
    let k = make_k(&m, &h, &r);

    let x_tilda = x_star - x_bar;

    let k1 = k[(0, 0)];
    let k2 = k[(1, 0)];
    let k3 = k[(2, 0)];

    let x_hat = x_bar + k1 * x_tilda;
    let xdot_hat = xdot_bar + k2 * x_tilda;
    let xdotdot_hat = xdotdot_bar + k3 * x_tilda;

    state = matrix(vec![x_hat, xdot_hat, xdotdot_hat], 3, 1, Row);
    cov = new_cov(&k, &h, &m);
}
</code></pre>

For brevity, I took out the code that collects the values for plotting.
Don't forget to check the 
<a href="https://github.com/IndigoCurnick/kalman-filtering-rs">companion repository</a>
for everything.

<div id="position-plot" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="velocity-plot" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="acceleration-plot" class="plotly-graph-div" style="height:100%; width:100%;"></div>

And these results are pretty great! The position, especially is a phenomenal 
improvement over the noist measurements. The velocity and acceleration 
parameters could be better, but for states which are inferred and are never 
updated directly, there is a limit to how good you can have them be.

The velocity and acceleration parameters take a very long time to converge and 
stabalise. This is because of initial conditions. We give the filter no helpful 
starting conditions. Better starting conditions and a corresponding lower 
starting covariance matrix will help a faster convergence. Whether you can 
practically seed the Kalman filter appropriately will depend a lot on the 
specific application. Sometimes you will have some kind of decent starting 
estimate, sometimes not.

Another plot that can really help us understand what is happening is the 
residual plot. This is where we take the difference between the estimated value 
and the true value - obviously we want to minimise this as much as possible. 
For the position, since I also have a measurement, I include the residual 
between the measurement and the true value.

<div id="position-residual" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="velocity-residual" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="acceleration-residual" class="plotly-graph-div" style="height:100%; width:100%;"></div>

I think the position residual is particularly impressive. After about \\(10s\\) of 
warm up, we reduce the noise from something like \\(300m\\) to something like 
\\(100m\\).

The keen eyed among you will have noticed that there's nothing about this filter 
which exactly requires us to have a falling object. Any object under constant 
acceleration will be tracked just fine by this filter. Try changing values in 
the simulation to see how it performs!

<script id="KalmanFilterScripts" src="/blog-assets/2025-01-04-kalman-filter-falling/plots.js"></script>

## References 

Zarchan, P., Musoff, H. (2009) *Fundamentals of Kalman Filtering: A Practical Approach (3rd Ed.)*. 
American Institude of Aeronautics and Astronautics

Tziallas, G., Adam, M., Assimakis, N., Polyzos, A. (2021) *Position, Velocity and Acceleration Tracking Using Kalman Filter*.
B P International