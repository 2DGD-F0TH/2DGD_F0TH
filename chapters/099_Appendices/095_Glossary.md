{{pagebreak}}
\appendix

Glossary
========

A {-}
-----

API
: Short for "Application Programming Interface", it's a set of definitions, tools and clearly defined methods of communication between various software components.

C {-}
-----

Call by reference
: Evaluation strategy where a function parameters are bound to a function by passing a reference to the arguments, this could cause *side effects*~[g]~ since the function would be able to change variables outside its local scope.

Call by value
: Evaluation strategy where a function parameters are bound to a function by making a copy of the values used as an argument.


D {-}
-----

Dangling Pointer
: A dangling pointer is a memory pointer that references an area of memory that doesn't contain a valid object. A dangling pointer usually happens when an object is deleted from memory forcibly, but the pointers referencing said object are not invalidated (usually by setting such pointers to a null value).

Dynamic Execution
: See *out of order execution*

E {-}
-----

EULA
: Short of "End User License Agreement", is essentially a contract that estabilishes the purchaser's right to use the software, usually with some limitations on how the copy can be used.

G {-}
-----

Greedy Algorithms
: Class of algorithms that try to solve a problem by making the locally optimal choice **at each stage**, approximating the global solution, without any global planning.

H {-}
-----

Hash Function
: A hash function is a special function that is used to map data of arbitrary size to fixed-size values. This function has some features like being able to spread values in an uniform way (minimizing the different values that have the same hash, called "hash collisions"), is fast and deterministic (given the same input will generate the same hash).

HUD
: Short of "Heads Up Display", in games it usually shows your health, ammunition, minimap and other information.

I {-}
-----

IDE
: Short for "Integrated Development Environment", it is a program that integrates a text editor with syntax highlighting, a compiler, a code checker, a project explorer and other features (like a tag explorer, for instance).

Information Hiding
: Information hiding is one of the basic principles of programming: each part of a program (a "module") should not expose its inner workings, but rather expose a stable "interface" to the outside world. This will help separating modules from each other and avoid "snowball effects" when modifying the inner workings of one of them.

K {-}
-----

Kanboard
: Short for "Kanban Board", are boards used to manage work. The board is usually divided into swimlanes and "cards" that represent the work to do are moved from left to right, to represent the progress of the work itself.

L {-}
-----

Letter Notation
: Also known as "letter music notation", it's a music notation system that uses letters A through G to write music.

M {-}
-----

Malware
: Short for "malicious software", it's a "catchall term" for viruses, trojan horses and any kind of software that is programmed to behave maliciously. Such software can steal information (passwords, key presses, habits, etc...) or flat out try to make your computer unusable (deleting system files, encrypting your documents and asking for a ransom, etc...)

Memory Leak
: A memory leak is usually the result of a programming error, where the memory is not correctly managed. This usually entails allocating and using memory without releasing it, thus the program will eat more and more memory as it keeps running.

Modern music notation
: The most common way to write music, using symbols to indicate the duration and type of note, while the symbol's positioning in a 5-line staff defines its pitch.

O {-}
-----

Oscillator
: An oscillator is a device (usually hardware) used to create an alternating current. Oscillators can be used in audio synthesis to create pitches.

Out of order execution
: Paradygm used in high performance CPUs to reduce the wasted instruction cycles. The instructions are performed in an order given by the availability of the input data and execution units instead than the original order defined by the programmer.

P {-}
-----

Pre-emption
: See *preemptive multitasking*


Preemptive multitasking
: A multitasking environment where the operating system forcibly initiates context switches (saves the state and interrupts temporarily) between running processes, regardless whether their job is finished or not.

Process Starvation
: See *starvation*


R {-}
-----

Race Condition
: A condition where two or more threads are competing for a resource (variable, screen buffer, ...) and the order that the threads happen to take hold of such resource changes the result.

Rootkit
: Usually associated with malware, a rootkit is a software that manages to get access to reserved areas of an operating system, usually to hide its own, or other softwares' presence.

S {-}
-----

Side Effect
: In computer science a function is said to have a "side effect" when it changes variables outside its local environment, this can happen in languages which use *call by reference*~[g]~ evaluation strategies.

Stack Overflow
: A stack overflow is a situation where too much data is pushed into a data structure called a "stack". One of the most common cases of "stack overflow" happens during recursion: when a function is called all the current work variables are saved and pushed into a stack data structure in memory, along with a "return address" that will allow us to come back to this point of the program. When a recursion is too deep (the recursive function calls itself too many times), the call stack gets filled up and it's not able to continue the execution, leading to an aborted operation.

Starvation
: Also known as "process starvation", it's a phenomenon where a certain process (or group of processes) has a lower priority than others, and is not able to access resources (like the CPU) because it's always "overtaken" by higher priority tasks. This leads to the process itself never being executed. When this happens, a process is labeled as "in starvation".

Static Typing
: Languages characterized by *static typing* are the ones where the type of a certain variable (integer, string, class, ...) is known at compile time.

U {-}
-----

UI
: Short of *User Interface*, defines the elements shown to and interacted by the user in a software or, in this case, a videogame menu.

Unreachable Memory
: This is a phenomenon where some dynamically allocated memory has no more references pointing to it. This is a common cause of memory leaks in programming languages without automatic garbage collection (like C and C++).

W {-}
-----

Wiki
: A wiki usually refers to a knowledge base website where users collaboratively modify content and structure by using their own web browsers.
