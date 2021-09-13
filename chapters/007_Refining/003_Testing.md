{{pagebreak}}

Testing your game {#testing}
=================

:::::: {.epigraph author="Anonymous"}
The bitterness of poor quality remains long after the sweetness of meeting the schedule has been forgotten.
::::::

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

Care should be taken when making automated tests: they should be as simple as possible, to avoid the presence of bugs in the tests themselves, also, like all code in the world, it's subject to its own maintenance: you should thoroughly comment your tests, and if the tested component changes, the connected test should change too, or it may fail.

### Manual Testing

Sometimes also called "play testing", this is usually divided in time periods, where more and more people participate:

- **In-house testing** with a small team of dedicated testers;
- **Closed Beta** where a small number of selected players is able to test the game, report bugs and issues. Usually at this stage, the game is already mostly finished and playable;
- **Open Beta** similar to the closed beta, but players can freely subscribe to play the game.

We will talk specifically about each one of these test types in detail in the following sections.

Unit Testing
------------

Unit Testing takes care of testing the smallest component possible inside your system: that usually means a single function. Such "units" must be separated from all their dependencies (via [mocking](#mocking)) to avoid bugs in their dependencies to interfere with our testing efforts.

Many programming languages have their own unit testing frameworks, among the most used:

- **Python:** Unittest (in the standard library);
- **Javascript:** unit.js;
- **C++:** Boost Testing Library;
- **Java:** JUnit;
- **Lua:** luaunit.

Remember: during unit testing, you need to make sure that the unit that you're testing has its dependencies mocked, or the results of your test will depend on the performance and correctness of said dependencies.

Integration Testing
-------------------

Integration testing is a step further from "unit testing", here you take different "units", put them together and check how they behave as a group.

There are many approaches to integration testing, the most used are:

- **Big bang integration:** The majority of the units are put together to form a somewhat complete system (or at least a major part of it), after that the integration testing starts. This can lead up to an "explosion of complexity" if the results are not accurately recorded.
- **Bottom-up:** Test the "lowest level" components first, this should help testing the "higher level" ones. After that you test the components that are "one level above" the previous ones. Keep going until the component at the top of the hierarchy is tested.
- **Top-down:** Opposite of the previous approach, you test the integrated modules first, then you branch into the lower-level modules, until reaching the bottom of the hierarchy.
- **Sandwich Testing:** a combination of Bottom-Up and Top-Down.

Regression Testing
------------------

Regression testing is a bit of an outlier in our "specific to general" testing structure, but that's because their objective is different.

Regression testing (sometimes also called *non regression testing*, you'll see why) is used to avoid our software from regressing into previous bugs.

This means that every time you find a serious bug in your software, you should fix it and make a test that will check for you if said bug is resurfacing.

With time, bugs and regression tests will accumulate, which usually means that automation is involved (like continuous intergration and delivery) to execute them at each push or at regular intervals.

Playtesting
-----------

Automated testing won't be able to help with how a game "feels" to the player, for that you need to a thorough play testing strategy. Here we will talk a bit of different strategies that can be mixed and matched to get the best out of it.

### In-House Testing

The first sessions of play testing should be done in-house, with a dedicated play testing team that has great reporting capabilities, and the product should already include tools that allow for quick reporting of bugs and issues that arise, as well as a good logging system set to its `DEBUG` level for maximum detail.

Close collaboration with the testing team is vital for a good game to be released, instead of seeing them as "the ones that give you more work", try looking at them as "the ones that will ensure your game gets a lot of praise".

### Closed Beta Testing

Happening after the in-house testing, usually done with multiplayer games, the "Closed Beta" phase is done with a selected group of players that try the game and report each and every issue with it, as well as bugs.

The product should have an easy way to directly report bugs and issues form inside the game itself, with the possibility of attaching a detailed log with the "ticket".

### Open Beta Testing

Differently from the "Closed Beta" phase, and usually coming after that, the "Open Beta" phase doesn't have a hard limit on the number of players that can take part to the beta testing.

The product should have the same characteristics that are used in the "closed beta" phase, plus possibly a higher degree stability.

Open Beta is usually done to test the robustness of the network system at higher loads, and having a possibly large (maybe larger than expected) player base can be a real test for the infrastructure.

### A/B Testing

A/B testing is a particular kind of testing that doesn't involve the "solidity of the game", as much as the enjoyability of some of its features. In A/B testing users are randomly presented with one of two versions of a certain feature; with a slight difference (usually a single variable that affects the experience is changed) between each of the two "versions".

A/B testing is normally used in an user experience research setting, but it can be used in game development too, to see which version (usually called *variant*) is better suited for the game, or even prepare a "segmented strategy" where one of the two variants could find place in a certain situation (for example a "simplified control scheme" vs a "full control scheme").
