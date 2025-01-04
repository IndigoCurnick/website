As part of my job, I work with a lot of earth data. To make my life easier,
I made a [small Rust library](https://github.com/IndigoCurnick/geodesic_coordinates_rs).
I haven't put it on crates.io yet, because right now it feels like a hodgepodge 
of different things useful to me, rather than a cohesive package. Plus, it 
has little documentation or tests. Therefore, if you want to include it in your 
Rust project, add the following to your `Cargo.toml`

```
[dependencies]
geodesic_coordinates_rs = { git = "https://github.com/IndigoCurnick/geodesic_coordinates_rs", branch = "master" }
```

In this article I'm just going to show you the distance and bearing calculators.

When moving on the earth, we don't really move on straight lines. We always move 
on a curved path, owing to the curvature of the earth. This means that the 
distance between two places as the crow flies is not simple to calculate. 
Another consequence is that our bearing (direction we face) changes as we move.

The crate offers three different ways to compute these geodesics - with the 
Haversine, Vincenty or Karney formula. Haversine is the oldest, 
and least accurate. It assumes that the earth is a sphere, which introduces some 
inaccuracy (the earth is an oblate spheroid - slightly squished at the poles and 
slighly bulging at the equator). 
Haversine is an updated formula from the 1970s, which is very poular and 
widely used. It's very accurate and easy to implement, but can be somewhat 
computationally expensive. Although, that depends on your definition of 
"computationally expensive". Even a low-range CPU could easily perform hundreds 
of thousands of Vincenty calculations a second. If performance is your 
concern, then Karney is for you. The Karney method is currently the most 
accurate and most efficient algorithm I am aware of. However, it's a pain to 
implement (luckily, I already did that for you). In general, I'd always suggest 
using Karney.


For each of these there are two "modes"
of computation. We can either have a start location and end location and ask the
question "what is the distance between these locations, and which bearing would 
I have to start moving in to get from the first to the second". Notice how 
the answers matter which way around the locations are. For example, from London 
to Berlin the bearing is 77.24°. However, from Berlin to London the bearing is 
267.93°! The second mode is asking the question "where will I end up and what 
direction will I be facing if I start with this direction and travel this 
distance?"

To make the crate easy to use, I made the functions resemble each other 
as much as possible. 

```
pub fn distance_and_bearing(lat1: Radians, lon1: Radians, lat2: Radians, lon2: Radians) -> DistBearing
pub fn location_and_bearing(lat1: Radians, lon1: Radians, bearing: Radians, distance: Metres) -> LocBearing
```

All three versions (Haversine, Vincenty and Karney) have these two methods only.
Each are in a different module, so you import them differently

```
use geodesic_coordinates_rs::geodesics::karney::distance_and_bearing;
```

For example. Notice how the functions take `Radians` - this is just a 
type alias for `f64`. Make sure you convert your human readable latitudes and 
longitudes into radians first! For instance, London is 
51.49684°, -0.16329° or 0.89879, -0.00285. It might seem weird at first, 
but this is much easier since most of the time when working with earth data the 
latitudes and longitudes will be in radians. To help you with this, I also 
export a constant `DEG_TO_RAD` which you can multiply by your latitudes and
longitudes to convert them to radians.

What about `DistBearing` and `LocBearing`? These are simple structs that just
help giving you results.

```
pub struct LocBearing {
    pub lat: Radians,
    pub lon: Radians,
    pub bearing: Radians,
}
```

`LocBearing` is used to anser that question "if I start at this position and 
travel this distance in this direction, where will I end up and what direction 
will I be facing?". Thus, `lat` and `lon` will be the final position - again, in 
radians. You can divide by `DEG_TO_RAD` to convert them back into familiar 
coordinates. `bearing` is also in radians - this is the final direction.

```
pub struct DistBearing {
    pub distance: Metres,
    pub bearing: Radians,
}
```

`DistBearing` is used to answer the question "what is the distance and direction
between these two places?". `distance` is in metres. `bearing`, again, is in 
radians and it represents the starting direction. As we already discussed, by 
travelling in a "straight line" the direction you face will constantly be 
changing. 

I also export all these as JavaScript functions you can use on the web, too. I 
do this via WASM. Rust has fantastic WASM support, which makes exporting to the 
web or a Node.js server really simple. Again, I haven't put this on NPM yet 
because I don't think the crate is ready, but you're welcome to build it from 
source using 

```
wasm-pack build --target web
```

You will need to install wasm-pack first.


There's only a small change between the Rust version and the JavaScript version,
and that's with WASM there are no modules. So, each of the functions becomes 
prepended with the type. For instance, there is `haversine_distance_and_bearing`
or `karney_location_and_bearing` and so on. To use these, you import them 
from the JavaScript file, which is a thin wrapper for the WASM that wasm-pack 
will automatically generate for you, something like this 

```
import init, { haversine_distance_and_bearing } from "/path/to/geodesic_coordinates_rs.js";

await init();
```

It's important to include the `await init();` line, since loading WASM has to be 
done async. If you try and use the functions before then, it'll crash. 

Other than that small difference, everything else is exactly the same. You 
still need to put radians in, and you get the same structs back out. Because this 
is JavaScript, if you embed it in a webpage then it won't need to make a request 
to your server to compute the result. 

You can see an example of the crate in action [here](/toolbox/geodesics). That 
shows you something you could make on the web, and of course, everything is open 
source. I hope you enjoy using this tool!