\null\clearpage

Testing your game
=================

When you want to assure the quality of your product, you need to test it. Testing will be probably be the bane of your existence as a developer, but it will also help you finding bugs and ensure that they don't come back.

Remember that you should be the one catching bugs, or your players will do that for you (and it won't be pretty).

Let's take a deep dive into the world of testing!

When to test
------------

### Testing "as an afterthought"

Most of the time, testing is left "as an afterthought", something that comes last, or at least something that is done way too late in the development process.

Testing is made difficult because the code is all interconnected, making it really hard (if not impossible) to separate and test each component.

Leaving testing as an afterthought is not a good idea, let's see the alternatives.

### Test-Driven Development

Some swear by the "Test-Driven Development" (from now on "TDD") approach. TDD consists in developing the test before writing a single line of code, then after the test is ready, create the code that solves that test.

That way the test will "drive us" to the solution of the problem. At least that's what it is supposed to do.

As a critique to the TDD approach, I personally think that people will end up trying to "solve the test", instead of "solving the problem". This means that sub-optimal solutions may be adopted, and "edge cases" will be missed.

### The "Design to test" approach

To get the best of both worlds, we need to work a bit more on our software design, by designing by having its testing in mind: functions should be self-contained, the weakest amount of coupling should be present (if any) and it should be possible to [mock](#mocking) elements easily.

Paying attention and going the extra mile to create an architecture that is easy to test will reward you in the long run.

### You won't be able to test EVERYTHING

Before trying to test anything, remember that testing is **hard**. You won't be able to test everything, and the situation will be worse when you will be in a time crunch to deliver your game.

Remember that hacking is not always a bad thing, sometimes cutting corners will get your game shipped and having a solid (and tested) basis will help you with that too.

Mocking {#mocking}
------------------

Before talking about the nitty-gritty of testing, we need to talk about "Mocking".

Mocking is a procedure that is usually performed during tests, that substitutes (in-place) an object or function with a so-called "mock". A mock is something designed to have no real logic, and just return a pre-defined result, or just have a pre-defined, very simple, behaviour.

Mocking will help you "detaching" objects that depend on each other, substituting such dependencies with "puppets" (the mock objects) that behave consistently and are not affected by bugs that may be present in the object that you are mocking.

Types of testing
----------------

Let's take a look at how we can test our game, because (as with many things in life), testing can be more than meets the eye. In this section we will talk about both manual and automated testing, the difference between them and what method suits what situation.

### Automated Testing

Automated testing is usually performed when new code is pushed to the repository, or on-demand. Automated testing makes use of a "test suite": a bunch of tests that are run on the game's elements to test their correctness.

Here we can see a small example of a simple function, along with a test, in pseudo-code:

~~~
function sum_two_numbers(a, b):
    // This function takes two numbers and sums them
    return a + b

function test_sum():
    // This function tests the sum function
    int result = sum_two_numbers(2, 2)
    assert result == 4
~~~

As we can see, the test makes use of the "assert" statement, which in many languages raises an error when the "assert" is false. This means that if, for some reason, the `sum_two_numbers` function was edited to return "2+2=5", an exception would be raised when the test is run.

### Manual Testing

Sometimes also called "play testing", this is usually divided in time periods, where more and more people participate:

- **In-house testing** with a small team of dedicated testers;
- **Closed Beta** where a small number of selected players is able to test the game, report bugs and issues. Usually at this stage, the game is already mostly finished and playable;
- **Open Beta** similar to the closed beta, but players can freely subscribe to play the game.

We will talk specifically about each one of these test types in detail in the following sections.

Unit Testing
------------

\placeholder
<!-- TODO -->

Integration Testing
-------------------

\placeholder
<!-- TODO -->

Regression Testing
------------------

\placeholder
<!-- TODO -->

Playtesting
-----------

\placeholder
<!-- TODO -->

### In-House Testing

\placeholder
<!-- TODO -->

### Closed Beta Testing

\placeholder
<!-- TODO -->

### Open Beta Testing

\placeholder
<!-- TODO -->
