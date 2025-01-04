I recently read the book "Refactoring: Improving the Design of Existing Code" by Martin Fowler. I thought the book was ok, but a little outdated. Generally, the book was a product of its time – while there were some gems of advice present, a lot of the book was dedicated to refactoring procedural code into object orientated code. While that was certainly in vogue in 1999, in 2022 we generally want to go the other way. My hope in this article is to bring some condensed nuggets of wisdom with some more up-to-date advice in certain areas. So, what is refactoring? Refactoring is when we improve the design of existing code without changing any of its features or performance in order to make it easier to add new features, improve performance, maintain and fix bugs in the future.

Fowler opens the book with a story that's familiar to me. He tells the tale of being a consultant in the software development business and strongly suggested to the management team of a large project to take some time and do a large scale refactor of their code-base. The managers assured him they would "get around to it at some point". Over the next few months he watched progress grind to a halt and the project collapse because "at some point" never came. They were always, this time, we're sure, so so close to delivery, let's just wait another week on this refactor. Soon, adding any new features or fixing bugs that had been bothering them for weeks or months became impossible. Already, the first key lesson is learned. With any major programming project you _will not_ get the design right the first time. Attempting to will lead to you analysis-paralysis. Most people understand this – the idea of planing out every single function or object in some giant code base from the top down is obviously never going to work. Therefore, it is better to simply start. However, people do not like the implication of this. The implication being they will need to, not "at some point", but early and often, improve the design of this code. Refactor early and refactor often to ensure that your project does not become some giant, buggy mess that nobody can work with.

There, we've discussed a very vague sense of when to refactor. Can we be any more specific? Yes. Computers do not care about code aesthetics. They can handle thousands upon thousands of functions inside one file, or all the work being scattered across many functions in different files, variables and functions named obscure and unclear things. None of these impact the computer. However as humans we will simply struggle with all of these things. When we have a code base that's like this – messy, unclear, difficult to work with – adding new features or fixing bugs becomes impossible. So, *before you add a new feature refactor*. Ask yourself what the code base should look like to make adding that new feature easy and then change the design to look like that. *If a bug has taken more than 10 minutes to find, you need to refactor*. Bugs that are challenging to locate in the code imply the code-base is challenging to understand, therefore, it is a good sign to step back from bug hunting and instead refactor.

You might think this sounds like a lot of refactoring. It is. You might worry that this will consume so much time from adding new features or fixing bugs. This, you shouldn't worry about. As long as from the start of the code-base, you were frequently refactoring in little five or ten minute bursts, you will only need five or ten minutes to implement these refactors. Huge refactors generally indicate that the refactoring process was ignored and needs to be done more frequently in future. Generally, I find that most programmers spend most of their time fixing bugs. It's because of this fact that refactoring actually increases the amount of time you get to adding new features. By constantly keeping the code-base clean, tidy and well designed you tend to avoid or spot a lot of bugs early. Finding and fixing them also becomes quicker and easier than it otherwise would be. Martin Fowler's whole PhD was on the study of refactoring and his findings were pretty conclusive – if you want to maximise the amount of time you have to develop, refactor a lot!

However, refactoring is not without dangers. What if you inadvertently break the code in some way? This is a genuine worry, and can be alleviated by another excellent technique that goes hand-in-hand with refactoring: tests. Testing is sadly overlooked in many programming courses, but I think it is beyond essential. Tests need to be automated and self checking. Automated means all the tests run with a single command, self checking means they report on whether or not they passed or failed. Now, every language has some sort of testing environment that you can use. `Python` has `pytest` or `unittest`. `Rust` has the extremely powerful `cargo test` (which I can not recommend enough as simply *the best* testing suit possible). You don't have to use one of these if you don't want to, it's often easy enough to make your own test environment by simply having a flag which switches the program from normal mode to test mode.

Testing comes in three flavours. Unit tests, integration tests and system tests. Unit tests are the simplest form of test and simply test a single function or method. The test inputs some values, receives the output and checks against some pre-programmed correct result. Suppose I had a function that took a number and returned the next highest prime. I'll show a little example in pseudo-code

    
    next_prime(num) {
    ... (implementation not relevant)
        return result;
    }
    
    test_next_prime() {
        assert(next_prime(0) is 2);
        assert(next_prime(1) is 2);
        assert(next_prime(2) is 2);
        assert(next_prime(4) is 5);
        assert(next_prime(-100) is 2);
        assert(next_prime(1.5) is 2);
    }

Many languages use `assert` to check the condition is true. You can see what my plan is here - I'm checking a variety of different kinds of inputs. 0 and 1 are obviously always interesting numbers. 2 is interesting - what if the function gets a prime? You can see in this implementation I decided it should simply return the same prime. Of course, the function name could also be read that it doesn't care if the number going in is a prime or not and just returns the next prime. Obviously, what the function does is up to you but the point of the test is to make you think about these edge cases. I then go on to check some negative values and floating point values.

The next set of tests are integration tests. These usually check little "blocks" of code as I like to think of them - a few functions that belong together. This could be one particular feature. Let's say you have a program that opens files with a specific ending of that file a good integration test is checking just the part dealing with opening files.

Finally there are system tests. These tests check the entire system from start to end as a user would use it.

I like to think of these as a pyramid. You'll probably want a huge number of unit tests, a dozen integration tests and only a couple of system tests. Remember that tests are themselves just code and are subject to bugs! Try and keep them simple.

Now that we have this automated, self checking testing suite, we can deal with the problem of breaking our code base during a refactor. All we have to do at this point is run the tests before and after we start the refactor! We will immediately be told if we have broken anything. If you run the tests often, every few minutes, you'll also know the limited number of places these issues could have been introduced were.

A good piece of advice that Fowler has is thinking about "hats". What hat are you wearing at any given moment you are programming? Are you wearing your feature hat, refactor hat, bugfixing hat and so on. Don't mix what you are doing - it leads to ruin. Adding a feature while refactoring is a sure way to get lost. Refactor, verify everything works via automated tests, then add features. You need to be very strict in keeping this separation.

So beyond when we want to add new features, when should we refactor? How do we know that it's time? This section is going to contain some vague advice since every situation is going to be unique, but I'll try and give some tips. As time goes on, you will develop a feel for when a code-base needs to be refactored. People often talk about "bad smells". Some of the things that tip them off to bad smells are

*   Functions that are too long
*   Temporary variables
*   Large, complex data structures
*   "Shotgun surgery" - every time you need to make a change you need to make lots of little changes in many places

I'll note that Fowler's list is much longer than mine, but he is writing his book for object orientated software which just tends to be more problematic than functional. [Read more here about that](/blog/2022-02-06/i-hate-oo). If you start to notice any of these things, it's a really good sign you need to do a general refactor.

Summary and Tips
----------------

*   When you need to add a new feature, first refactor to make adding that feature easier
*   Before beginning to refactor create a set of automated, self checking tests
*   Refactor in small steps so you can easily correct mistakes
*   Anyone can write code that computers can understand; great programmers write code for other humans
*   When refactoring use rhythm - test, small change, test, small change, test, small change...
*   Always know what "hat" you are wearing while you work
*   When you feel the need to add a superfluous comment try refactoring
*   A suit of tests is a powerful bug detector and decapitates the time it takes to find bugs
*   When you get a bug report, first write a test that simulates that issue