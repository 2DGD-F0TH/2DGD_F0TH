Metrics and dashboards
----------------------

During development you need to keep an eye on the quality of your project, that's when you need a **project dashboard**: but before that, you need to decide what your **quality metrics** are, that means the measurements that define if your project is "up to par" with what you expect or not.

### SLOC

This is probably the simplest metric out there: The "Source Line of Code" (SLOC). It is used to measure the size of a program by counting its lines of code. Once Bill Gates said the following:

> Measuring programming progress by lines of code is like measuring aircraft building progress by weight.

An aircraft must be lightweight and robust, and being heavier than necessary will stop it from flying. The same reasoning should be applied here: a longer source code doesn't mean a better product.

It is important to strike a balance between "readability" and "brevity": your code should be short, but being source code, it is still meant for humans to read, so readability matters more than brevity.

Usually the SLOC metric is used to give a "order of magnitude" impression of the program: considering 2 programs that do exactly the same thing, one is 10.000 lines of code, the other one is 100.000, you may start to suspect that the bigger program is more (probably uselessly) complex and less maintainable.

### Cyclomatic Complexity

More precisely called "McCabe's Cyclomatic Complexity", this metric defines the number of linearly independent paths through a program's source code: the higher the metric, the higher is the number of paths a piece of code can take in its elaboration.

This means that a higher number of paths takes into account a higher number of conditions and decisions and when such number becomes too high, the code becomes hard to maintain.

The maximum complexity suggested is 10, although sometimes it's good to relax such metric to a maximum of 15. When the cyclomatic complexity becomes higher than the maximum value, it is suggested to split the module into smaller, more maintainable modules.

Your IDE, if advanced enough, should already be able to warn you of a high cyclomatic complexity.

:::: pitfall ::::
Be mindful that cyclomatic complexity may have issues of "over-estimation" or "under-estimation", depending on a case-by-case basis. McCabe's cyclomatic complexity is far from a "silver bullet" that will suit all your needs, but as all other metrics, it can give a pointer over where refactoring may be necessary.
::::

### Code Coverage

When you have a test suite, you may already be thinking about a metric that tells you how much of your code is tested. Well, here it is: the *code coverage* metric tells you what percentage of your code base has been run when executing a test suite.

That is both the useful and damaging part of this metric: *code coverage* doesn't tell you **how well** your code is tested, just **how much code was executed**, so it's easy to incur into what I like to call "incidental coverage": the code coverage presents a higher value, when the code is merely "executed" and not thoroughly "tested".

Code coverage is split in many "sub-sets", like:

- **Statement Coverage:** how many statements of the program are executed;
- **Branch Coverage:** defines which branches (as in portions of the if/else and "switch" statements) are executed;
- **Function Coverage:** how many functions or subroutines are called.

This is also why it's better to prepare unit tests first, and delay the integration tests for a while.

To know more about those terms, head to the [testing section](#testing).

### Code Smells

Code Smells is a blanket term representing all the common (and thus known) mistakes done in a certain programming language, as well as bad practices that can be fixed more or less easily.

Some of these smells can be automatically detected by static analysis programs (sometimes called Linters), others may require dynamic execution, but all code smells should be solved at their root, since they usually entail a deeper problem.

Among code smells we find:

- Duplicated Code;
- Uncontrolled Side Effects;
- Mutating Variables;
- God Objects;
- Long Methods;
- Excessively long (and thus complex) lines of code.

### Coding Style breaches

When you are collaborating with someone, it is absolutely vital to enforce a coding style, so that everyone in the team is able to look at everyone else's code without having to put too much effort into it.

Coding style can be enforced via static analysis tools, when properly configured.

Counting (automatically) the number of coding style breaches can help you estimate how much effort working on the code is necessary, thus you would be able to foresee slowdowns in the development process.

### Depth of Inheritance

Some people say that inheritance is evil and should be avoided, some other say it's good. As with all things, *in medio stat virtus* (virtue stands in the middle), sometimes inheritance is better left where it is, other times its usage is necessary for things to make sense.

The *depth of inheritance* metric tells us how deep the inheritance hierarchy is, thus this metric will tell the us the strength of one of the possible dependency types. The deeper the inheritance, the more dependencies we have, which means that we have more classes that, if edited, will change the behavior of the "children classes".

It's better having a short inheritance depth, (although it's not necessarily wrong) having a longer chain of dependencies might mean we have a structural problem, where some classes are "too generic" and at the top of the hierarchy we have some kind of "universal object".

### Number of methods / fields / variables

Let's talk numbers: having too many methods or fields in a class can be an indicator of a so-called "god object": an object that has too many responsibilities under its wing (does too many things), this is a breach of the *single responsibility principle* and should be avoided.

We can fix this by splitting the class into smaller classes, each with its own single responsibility.

A high number of local variables instead may point to a complexity issue: your algorithm may be more complex than needed, or needs to be split into different functions.


### Number of parameters

This metric is specific for functions, when a function has a lot of parameters, it's harder to call and harder to understand. Functions should have no more than 5 parameters in most cases, more and it will be complex.

Some automated tools in your IDE may be able to warn you in case methods and functions have too many parameters.

To solve this issue, you may need to review the function (maybe it has too many responsibilities?) or pass a so-called "complex structure" to it (thus merging all the parameters into one).

### Other metrics

The metrics listed above are not the only ones available to you, some IDEs have aggregated metrics (like the "maintainability index" in Visual Studio), while there may be other metrics you want to measure, some follow:

- **Lead Time:** Time elapsed between the start and end of a process (may be a ticket, or a task);
- **MTBF:** (Mean Time Before Failure) represents the mean time before the software crashes;
- **Crash Rate:** The number of times a software crashes, over the number of times it's used.
