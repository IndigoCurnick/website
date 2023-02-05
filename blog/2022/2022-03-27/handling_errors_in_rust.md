<!-- html -->
<!-- wp:paragraph -->

<h1>Handling Errors in Rust</h1>

        <p>Many people who come to Rust for the first time struggle with error handling. There is no concept of the try-catch
            block as there is in many other languages like Java or Python. How do you handle errors? This is done via four
            concepts – <code>Result</code>, <code>Option</code>, <code>Unwrap</code> and <code>Match</code>.</p>
        <!-- /wp:paragraph -->
        
        <!-- wp:heading {"level":1} -->
        <h2>Result</h2>
        <!-- /wp:heading -->
        
        <!-- wp:paragraph -->
        <p>Result is the core concept of error handling. Since Rust is a functional language, we obviously expect to use many
            functions. If each of those functions returns a result we can radically reduce the chances of our program crashing.
            How? Let's use an example</p>
        <!-- /wp:paragraph -->
        
        <!-- wp:code -->
        <pre class="wp-block-code"><code>
        pub fn divide(x: f64, y: f64) -&gt; Result&lt;f64, String&gt; {
            if y == 0.0_f64 {
                return Err("Can not divide by zero".to_string());
            } else {
                return Ok(x / y);
            }
        }</code></pre>
        <!-- /wp:code -->
        
        <!-- wp:paragraph -->
        <p>Phew. There's A LOT going on here – let's try and break it down step by step.</p>
        <!-- /wp:paragraph -->
        
        <!-- wp:paragraph -->
        <p>We have a public function called divide that takes two 
            <code>float64</code> arguments. This function will divide 
            <code>x</code> by <code>y</code>. Simple enough, but there is a 
            danger zone when <code>y</code> is 0 – since we can not divide by zero, 
            our program will crash. How can we avoid this? In something like 
            Python or Java, we would probably use a Try-Catch block. 
            Here, we can use a <code>Result</code>.
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:paragraph -->
        <p>We see this function returns a 
            <code>Result&lt;f64, String&gt;</code>. 
            <code>Result&lt;_,_&gt;</code>
            is simply a type like any other.
            Specifically it is an <code>Enum</code> type that contains two 
            values – an <code>Ok</code> type and an <code>Error</code> type. 
            In my example the <code>Ok</code> type is <code>float64</code>
            and the <code>Error</code> type is <code>String</code>. 
            You could put any types in there.
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:paragraph -->
        <p>This function has two branches, and we obviously must return from 
            both of them. Notice how we "wrap" what we return in either 
            <code>Ok(_)</code> or <code>Err(_)</code>. This is how we tell 
            Rust which is an Ok value and which is an Error value. 
            Whatever is in <code>Ok(_)</code> must match the first type nestled 
            inside <code>Result&lt;_,_&gt;</code> and whatever in 
            <code>Err(_)</code> the second.
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:paragraph -->
        <p>As a note here too, the reason why I must use the 
            <code>.to_string()</code> function is because Rust has two different 
            kinds of strings. It has <code>String</code> which is more like how 
            strings work in most other languages – this is basically a char 
            array under the hood. The other type is <code>&amp;str</code> – 
            this is a reference to a string in memory – a "string literal". 
            Since a string I type in between quotes is always a string literal, 
            I do not have the string itself but only a reference to it. I won't 
            get specifically into the reasons why now, but in order to pass this
            string literal reference out of this function we would need to 
            introduce one of the most unique (and challenging) concepts in Rust 
            – the lifetime.
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:heading {"level":1} -->
        <h2>Option</h2>
        <!-- /wp:heading -->
        
        <!-- wp:paragraph -->
        <p><code>Option</code> is similar to <code>Result</code>, but it either 
            contains a value or contains <code>None</code>. Since we've already 
            covered a similar concept, let's just dive right in with an example
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:code -->
        <pre class="wp-block-code"><code>
        pub fn contains(text: &amp;str, target_char: char) -&gt; Option&lt;String&gt; {
            if text.contains(target_char) {
                return Some(text.to_string());
            } else {
                return None;
            }
        }</code></pre>
        <!-- /wp:code -->
        
        <!-- wp:paragraph -->
        <p>This function checks to see if the <code>target_char</code> is inside
            text, if it is, it returns the text converted to a 
            <code>String</code>. Again, we see this "wrapping" principle at play
            – the type we want to return is now wrapped inside 
            <code>Some(_)</code>. In the case where there is nothing, 
            we just return <code>None</code> – with no wrapping.
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:heading {"level":1} -->
        <h2>Unwrap</h2>
        <!-- /wp:heading -->
        
        <!-- wp:paragraph -->
        <p>Ok, we have discussed <code>Result</code> and <code>Option</code> 
            which "wrap up" what's inside them, so we can safely pass around 
            errors and null values. How do we actually get to the thing inside 
            of it? We use the function unwrap. With a <code>Result</code> or 
            <code>Option</code> type, we can unwrap to get the value inside. So,
            for example
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:code -->
        <pre class="wp-block-code"><code>
        let foo: Option&lt;f64&gt; = Some(1_f64);
        let bar: f64 = foo.unwrap(); // bar is equal to 1_f64 here</code></pre>
        <!-- /wp:code -->
        
        <!-- wp:paragraph -->
        <p>or for another example*</p>
        <!-- /wp:paragraph -->
        
        <!-- wp:code -->
        <pre class="wp-block-code"><code>
        let foo: Result&lt;f64, String&gt; = Ok(1_f64);
        let bar: f64 = foo.unwrap(); // bar is equal to 1_f64 here</code></pre>
        <!-- /wp:code -->
        
        <!-- wp:paragraph -->
        <p>However, we want to stay away from using <code>.unwrap()</code> 
            unless we are <em>really</em> sure that the result is not an error
            or the <code>Option</code> is not <code>None</code>. Why? 
            Because if we unwrap a result to find an <code>Error</code>, our 
            program will "panic" – that is, crash. How then can we actually deal
            with <code>Result</code> and <code>Option</code>? 
            We introduce the next concept - <code>Match</code>
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:heading {"level":1} -->
        <h2>Match</h2>
        <!-- /wp:heading -->
        
        <!-- wp:paragraph -->
        <p><code>Match</code> is a kind of statement in Rust, similar to an 
            <code>If</code> statement but with one important difference – 
            a <code>Match</code> statement must cover all possible values of the
            type. Let's use two examples first, then talk a little more
            theoretically.
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:paragraph -->
        <p>Let's consider the functions we wrote earlier.</p>
        <!-- /wp:paragraph -->
        
        <!-- wp:code -->
        <pre class="wp-block-code"><code>
        let x = 10_f64;
        let y = 0_f64;
        let div = divide(x, y);
        let num = match div {
            Ok(x) =&gt; x,
            Err(y) =&gt; panic!("{}", y)
        };
        let text = "Hello";
        let target = 'e';
        let text_res = contains(text, target);
        match text_res {
            Some(x) =&gt; println!("{}", x),
            None =&gt; println!("Target char not found"),
        };</code></pre>
        <!-- /wp:code -->
        
        <!-- wp:paragraph -->
        <p>In the first example we use the divide function we wrote. We can see 
            that since y is 0, the function will return an <code>Error</code>
            – so when we <code>Match</code> the result the second arm will fire. 
            In this example we assign from the <code>Match</code> statement.
            There are two variants for <code>Result&lt;_,_&gt;</code> - 
            <code>Ok(value)</code> and <code>Err(y)</code>. 
            Notice how I have x and y wrapped inside <code>Ok(_)</code>Ok(_)
            and <code>Err(_)</code> - these are NOT the same x and y defined 
            further up in the function as they have a different scope, these
            are just temporary variables.
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:paragraph -->
        <p>In the second example, we do not assign from the <code>Match</code> 
            statement. Instead, we simply execute code. 
            <code>Option&lt;_&gt;</code> has two variants, 
            <code>Some(value)</code> and <code>None</code>, which make the two 
            branches of our <code>Match</code> statement. In this case, "Hello"
            does contain 'e' so the <code>Some(_)</code> arm will execute and 
            print "Hello" to the console.
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:paragraph -->
        <p><code>Match</code> statements have the special property that ALL 
            possible values for the type being matched are covered. For
            <code>Result</code> and <code>Option</code>, this is easy since 
            they only have two values. We can <code>Match</code> on any type 
            though. Here's an example of <code>Match</code> on an integer.
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:code -->
        <pre class="wp-block-code"><code>
        let x = 6_i64;
        match x {
            1_i64 =&gt; println!("Value is one"),
            2_i64 | 3_i64 | 4_i64 =&gt; println!("Value is either two, three or four"),
            5_i64..=10_i64 =&gt; println!("Value is between five and ten"),
            _ =&gt; println!("Value is something else")
        };</code></pre>
        <!-- /wp:code -->
        
        <!-- wp:paragraph -->
        <p>In this example we <code>Match</code> an integer. 
            It is very unlikely that I will type out every single possible 
            integer value or range, so instead we use <code>_</code> to mean 
            "any other values for this type that is not explicitly already 
            mentioned". I usually find that when I <code>Match</code> on an 
            integer I think it should only be able to have a set number of 
            possible values and often <code>panic</code> on the <code>_</code>. 
            An example might be matching on the length of a 
            <code>Vector</code>Vector. If we only ever want the vector length to
            be 10, 11 or 12 we could <code>panic</code> and print an error on 
            all other values ("Incorrect number of elements in vector").
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:paragraph -->
        <p>So why do it this way? What's wrong with a good old Try-Catch? 
            The answer is simple – Try-Catch is dysgenic. 
            Got a troublesome program? Just whack the whole thing inside a 
            Try-Catch. I would be surprisingly wealthy if I had a pound
            for every time I saw this. By using <code>Match</code> it forces us 
            to think really carefully about all of the possible places
            where our program could crash – and nothing, no Try-Catch or error 
            handling, can overcome how important it is to think.
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:heading {"level":1} -->
        <h2>A Few Notes</h2>
        <!-- /wp:heading -->
        
        <!-- wp:paragraph -->
        <p>Since <code>Result</code> and <code>Option</code> are just types, we 
            can pass them around and into functions. Now, I have never passed 
            around a <code>Result</code>, but <code>Option</code> is more or 
            less how Rust does key word arguments. Let's look at an example.
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:code -->
        <pre class="wp-block-code"><code>
        pub fn process(x: f64, y: f64, z: Option&lt;f64&gt;) -&gt; f64 {
            let mut res = x * y;
            match z {
                Some(p) =&gt; res += p,
                None =&gt; {}
            };
            return res;
        }</code></pre>
        <!-- /wp:code -->
        
        <!-- wp:paragraph -->
        <p>In this example, the function will multiply together x and y, and if 
            z is something it will add it to the result.
            Notice how we simply use <code>{}</code> to indicate that nothing 
            needs to be done if z is <code>None</code>
            – remember <code>Match</code> statement MUST
            cover all possible values of the type.
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:paragraph -->
        <p>While we looked at the <code>Match</code> statement which is one of 
            the most powerful concepts in Rust, we are also provided with
            some functions to check which we could use in an if statement. 
            These are <code>is_some()</code>, <code>is_none()</code> 
            (which apply to <code>Option</code>) and <code>is_ok()</code>, 
            <code>is_err()</code> (which apply to <code>Result</code> type). 
            I leave it as an exercise for the reader to figure out
            what these do (if you have understood the article it should 
            hopefully be obvious!!!!)
        </p>
        <!-- /wp:paragraph -->
        
        <!-- wp:paragraph -->
        <p>I'll show you one example without commentary</p>
        <!-- /wp:paragraph -->
        
        <!-- wp:code -->
        <pre class="wp-block-code"><code>
        pub fn process(x: f64, y: f64, z: Option&lt;f64&gt;) -&gt; f64 {
            let mut res = x * y;
            if z.is_some() {
                res += z.unwrap(); // Is it safe to unwrap here? Why?
            }
            return res;
        }</code></pre>
        <!-- /wp:code -->
        
        <!-- wp:paragraph -->
        <p>*While I have used manual type annotations for clarity, in this 
            particular example it is necessary, as Rust can not
            infer the type of foo. Remember that the type in this case is 
            BOTH parts of <code>Result&lt;_,_&gt;</code>. While it can infer
            <code>Result&lt;f64,_&gt;</code>, being unable to infer the error 
            state makes this unable to be compiled.
        </p>
        <!-- /wp:paragraph -->
<!-- html -->