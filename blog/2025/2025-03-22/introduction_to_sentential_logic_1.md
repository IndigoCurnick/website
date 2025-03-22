This article will be part of a series, so make sure you check back for the next part!

The study of logic is an ancient part of the liberal arts tradition. Unfortunately, this tradition has waned in the last century and now very few people study them. As part of my interest in polymath, I wish to contribute to a revival of the arts.

What are the liberal arts? The liberal arts are a study of two different groups of subjects: the trivium and the quadrivium. The trivium are the three arts relating to the mind: logic (the art of thinking), grammar (the art of creating and combining symbols) and rhetoric (the art of communication). The quadrivium are the four arts relating to matter: arithmetic (the theory of number), music (application of arithmetic), geometry (theory of space), astronomy (application of geometry).

While the quadrivium might now be a tad on the outdated side, as many students might not have an interest in music and astronomy, the trivium are universal constants. Everything a person can ever do depends upon thinking, combining symbols and communicating.

In this blog, we're going to focus in on logic, and on a particular part of it. Specifically, we'll be looking at sentential logic. Sentential logic is the method of analysing the form, and not the content, of an argument to determine whether it is valid or not. This idea originally started with Aristotle (and probably earlier Greek thinkers), but reached its current form with the mathematicians of the late 19th and earth 20th centuries. Bertrand Russel, for example, was particularly influential in its development.

Who should study logic? Ideally, everyone. However, anyone working or studying mathematics, physics, computer science or philosophy should master this subject.

## Categorical Propositions

A categorical proposition affirms or denies some relation without any qualification. It expresses a direct relationship between the subject of the sentence and the predicate. Some examples

1. All dogs are mammals
2. Some birds are not flightless

In 1) the subject is "dogs" and the predicate is "mammals". 

We classify these into four groups - called the AEIO forms.

- A - total affirmations e.g. all lions are animals
- E - total negation e.g. no lions are horses
- I - part affirmation e.g. some lions are tame
- O - part negation e.g. some lions are not tame

As far as I can tell, the AEIO terms are purely a teaching aid that came from Medival logic textbooks influenced by Boethius. They don't have any formal meaning, but have just stuck around out of tradition. They're often used as shorthands so it's good to learn them.

## Syllogisms

Let's start with the basic form of a syllogism we might encounter

- P1: A bat is a mammal
- P2: No bird is a mammal
- C: A bat is not a bird

We can see that C follows from P1 and P2.

Syllogisms have the following 10 rules, which we'll elaborate on

Rule 1: A syllogism must contain three and only three terms. We usually denote these with P1, P2, and C. For "premise one", "premise two" and "conclusion".

Rule 2: A syllogism must contain three and only three propositions. Each of the terms must be a single proposition only. For example, if P1 was "a bat is a mammal and no bird is a mammal" then we can easily go astray. 

Rule 3: The middle term must be distributed in at least one of the premises. The middle term is the term which appears in both premises. To be distributed means to be part of a universal. So, in the above example "mammal" is the middle term. In P1 "mammal" is not distributed because it tells us nothing about mammals, only some of the mammals which are bats, in this case. In P2 "mammal" is distributed, because we learn something about all mammals - namely, that not a single one of them is a bird. An invalid example is below

- P1: all dogs are animals (middle term "animal", not distributed)
- P2: all cats are animals (middle term "animal", not distributed)
- C: All dogs are cats (false!!!)

Rule 4: No term may be distributed in the conclusion which was undistributed in its own premise. If the premise is of the "some" form, then we can't make a universal claim in the conclusion. Consider

- P1: some dogs are brown
- P2: all dogs are animals
- C: all animals are brown (false!!!)

Here we've falsely concluded that all animals are brown because some dogs are brown. Notice how in the conclusion "animals" is distributed but in P2 it is not. At most we can conclude that *some* animals are brown.

Rule 5: From two negative premises no conclusion can be drawn. This is because we can't establish any connection between the terms. Consider

- P1: no cats are dogs
- P2: no dogs are reptiles
- C: all cats are reptiles (false!!!)

While at first glance C might *seem* valid at first glance, we really can't know anything about the status of the relationship between cats and reptiles here. You need to be careful as false arguments can seem to be true at first glance with this one. Consider

- P1: No mammals are cold blooded
- P2: No reptiles are mammals
- C: no reptiles are cold blooded (false!!!)

Rule 6: If one premise is negative, the conclusion must be negative. This one is pretty simple. Consider

- P1: No mammals are cold blooded
- P2: A dog is a mammal
- C: No dogs are cold blooded

We can never draw a positive conclusion from something *not* being true.

Rule 7: From two partial or singular (or contingent) premises, no conclusion can be drawn. Partial means uses "some" instead of "all". Singular means about one individual of a class e.g. "Bob" rather than say "all humans". Contingent means things which may or may not be true at any given time e.g. "it is raining" (sometimes true, sometimes false). Consider

- P1: Some cats are fluffy (partial)
- P2: Some fluffy things are dogs (partial)
- C: Some cats are dogs (false!!!)

Another example

- P1: Socrates is a philosopher
- P2: Some philosophers are wise
- C: Socrates is wise (false!!! We need other evidence to prove this, since some philosophers *aren't* wise)

Rule 8: If one premise is partial, the conclusion must be partial. So, if one of the premises uses "some" then the conclusion must use "some" too. 

- P1: Some dogs are brown
- P2: All dogs are animals
- C: Some animals are brown (true!)

Rule 9: If one premise is contingent, the conclusion must be contingent. In other words, if any one of the premises might be true or false depending on context, then the conclusion must also be maybe true, maybe false depending on context. Consider

- P1: All bachelors are unmarried (necessary, always true no matter the context)
- P2: John is a bachelor (contingent, John can get married and this changes)
- C: John is unmarried (contingent, John can get married later)

Rule 10: If one or both premises are empirical, the conclusion must be empirical. Consider

- P1: All humans are mortal (empirical, we have observed this)
- P2: Socrates is human (analytic, necessarily follows from the definition of *Socrates*)
- C: Socrates is mortal (empirical)
## Deductive Reasoning

We've seen the syllogism, but there we were still shackled to the content of the argument. What we'd like to start doing now is unshackling ourselves from the content and just analyse the form.

Something important to understand at this stage is we are only looking to see if an argument is *valid*, not if an argument is *true*. Truth comes from the content of the argument. For example, the following argument is valid, but not true

- P1: All dogs are reptiles
- P2: All cats are dogs
- C: All cats are reptiles

You might wonder what the point in this kind of analysis is if it doesn't guarantee the argument is true. There's really two reasons. First, no argument can be reliably true if it is invalid (sometimes "correct" conclusions are drawn from invalid arguments but this is mere coincidence!). So, understanding whether an argument is valid helps filter out lots of bad arguments really easily. Moreover, the content of arguments is often confusing and misleading: rewriting the argument into a simpler structure can help us systemically analyse smaller parts which is easier to determine truth. Second, this kind of analysis is extraordinarily useful in mathematics, which we will see in a little bit as we go further. 

So, how do we abstract these arguments? We introduce *variables* - letters which can stand for certain things. Let's look at two arguments

- P1: All rocks are hard
- P2: All stones are rocks
- C: All stones are hard

- P1: All mammals are warm blooded
- P2: All dogs are mammals
- C: All dogs are warm blooded

Let's take the following idea, let's replace every instance of "dog" with \\(d\\), every instance of "mammal" with \\(m\\), every instance of "warm blooded" with \\(w\\), every instance of rocks with \\(r\\), every instance of "hard" with \\(h\\),  every instance of "stone" with \\(s\\)

- P1: All \\(r\\) are \\(h\\)
- P2: All \\(s\\) are \\(r\\)
- C: All \\(s\\) are \\(h\\)

- P1: All \\(m\\) are \\(w\\)
- P2: All \\(d\\) are \\(m\\)
- C: All \\(d\\) are \\(w\\)

Hmmm we're starting to see something now. Let's call the middle term \\(M\\) (remember the middle term is the one which appears in both premises). Let's call the major term \\(A\\) (the major term is the predicate of the conclusion - "hard" in the first argument and "warm blooded" in the second). Let's also call the minor term \\(B\\) (the minor term is the subject of the conclusion - "stones" in the first example and "dogs" in the second)

- P1: All \\(M\\) are \\(A\\)
- P2: All \\(B\\) are \\(M\\)
- C: All \\(B\\) are \\(A\\)

- P1: All \\(M\\) are \\(A\\)
- P2: All \\(B\\) are \\(M\\)
- C: All \\(B\\) are \\(A\\)

Aha! We've just discovered our first abstract argument form. There are many, many different valid argument forms. I simply won't be able to cover them all in this blog, maybe I can cover them in future.


## Exercises

 Are the following syllogisms valid or not? Why?

1)

- Either John or Barry will win the fishing contest
- Barry can't compete because he is sick
- John will win the fishing contest

2)

- Either John or Barry will win the fishing contest
- John won before
- John will win the fishing contest

3)

- Every number divisible by 2 is an even number
- 10 is divisible by 2
- 10 is an even number

4)

- A bat is a mammal
- No bird is a mammal
- A bat is not a bird

5)

- If it is raining, then the ground will be wet
- It is raining
- The ground will be wet

6)

- If it is raining, then the ground will be wet
- The ground is wet
- It is raining

7)

- If it is raining, then the ground will be wet
- The ground is not wet
- It is not raining

8)

- If it is raining, then the ground will be wet
- It is not raining
- The ground will not be wet

You can find solutions to these exercises [here](/blog/2025-03-22/introduction-sentential-logic-1-solutions), but make sure you try them all yourself first!

## References

Joseph, Sister M. (1937) *The Trivium: The Liberal Arts of Logic, Grammar, and Rhetoric*. Paul Dry Books

Velleman, D. J. (2019) _How to Prove It: A Structured Approach (3rd ed)_. Cambridge University Press

Stoll, R., R. (1968) _Set Theory and Logic_ Dover Publications

