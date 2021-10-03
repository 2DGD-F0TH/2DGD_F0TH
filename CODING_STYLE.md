Style Guide For Code Listings
=============================

This book was created with the intention of being as clear as possible, thus it should follow some "clarity rules", like:

General Style Guide
-------------------

**Comments should be clear and "not scarce" in number:** Things should be explained thoroughly, step by step;

**Absolutely no obscure shortcuts:** some languages allow you to do some conversions or operations that severely hinder readability. So instead of writing the following:

```
// In javascript you can convert a number to boolean like follows (Type Coercion)
let num = 1;
let b = !!num;
// This is not readable, and may make no sense to many people
```

Write this instead:

```
let num = 1;
let b = Boolean(num);
```

**Try to avoid ternary conditionals:** as nifty and short as they can be, sometimes they are not as readable as necessary in a beginner-friendly book.

Pseudocode
----------

Pseudocode doesn't really have a structure, which makes things a lot harder to read. Here is a style guide that should help you inserting new code while keeping everything as coherent as possible.

### Basics

The pseudocode used in this book will be C-like, thus it should follow the broad strokes of that language:

```
int function addNumbers (int a, int b){
    return a + b;
}
```

Keep operators separated by using a single space and use 4 spaces for indentation.

### Conditionals

Conditionals follow the same pattern as C-Like languages, with the condition wrapped in parentheses but written in "structured English":

```
if (i is divisible by 3){
    // Do stuff...
}
```

### Iteration

As with conditionals, the condition is wrapped in parentheses and written in "structured English". Iterations allowed are:

- While (condition)
- Do .... While (Condition)
- For (Condition)

Here are some examples:

```
for (each element of list){
    show element on screen;
}

while (i is less than 10){
    print i;
    add 1 to i;
}
```

Other Languages
---------------

Simply use your best judgement, the rules stated above are just guidelines made to keep the book simple and explain everything as deeply as possible. This document is mostly here to regulate how pseudocode is written.
