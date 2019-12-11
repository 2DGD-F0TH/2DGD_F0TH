\null\clearpage

Project Management Basics and tips
===================================

\epigraph{Those who plan do better than those who do not plan even though they rarely stick to their plan.}{\textit{Winston Churchill}}

Project management is a very broad topic but I feel that some basics and tips should be covered in this book, as knowing some project management can save you a lot of headaches and can make the difference between success and a colossal failure.

Some generic tips
---------------------

### Be careful of feature creep

The "it would be cool to" trap, formally called "feature creep", is a huge problem in all projects that involve any amount of passion in them.

Saying "it would be cool to do xxxx: let's implement it!" can spiral out of control and make us implement new features forever, keeping us from taking care of the basics that make a good game (or make a game at all).

Try to stick to the basics first, and then eventually expand when your game is already released, if it's worth it: First make it work, only then make it work well.

### On project duration

When it comes to project management, it's always tough to gauge the project duration, so it can prove useful to remember the following phrase:

> "If you think a project would last a month, you should add a month of time for unforeseen events. After that, you should add another month for events that you really cannot foresee."

This means that projects will last at least 3 times the time you foresee.

Software Life Cycle Models
----------------------------

When talking about project management (in itself or in the broader field of Software Engineering) it is really useful to talk about some guideline models that can be used to manage your project.

### Iteration versus Increment

Before getting to the models, we need to discuss the difference between two terms that are often used interchangeably: "iteration" and "increment".

**Iteration** is a non-deterministic process, during an iteration you are revisiting what you have already done, and such revisiting can include an advancement or a regression. While iterating, you have no idea when you will finish your job.

**Increment** is deterministic instead, with increments you are proceeding by additions over a base. Every increment creates a "new base" for the next increments, and increments are numbered and limited, giving you an idea of when you have to finish your job.

### Waterfall Model

The Waterfall model, also known as "sequential model" is the simplest one to understand, easily repeatable (in different projects) and is composed by phases that are **strictly sequential**, which means:

- There is no parallelism;
- There is no overlap between phases;
- When a phase is completed, you cannot go back to it.

![Diagram of the waterfall life cycle model](./images/project_management/waterfall_model.png){width=60%}

This makes the Waterfall life cycle model *extremely rigid*, everything needs to be carefully analyzed and documented (sometimes people define this model "document-driven") and the coding is done only in its final phases.

In order to have a good result, this model requires quantifying some metrics (time spent, costs, ...) and such quantification heavily relies on the experience of the project manager and the administrators.

### Incremental Model

When a project of a certain size is involved, it's a bad idea to perform the so-called "big-bang integration" (integrating all the components together). Such approach would make troubleshooting a nightmare, so it's advisable to *incrementally integrate* the components.

The Incremental Model allows to have a "high-level analysis and planning", after that the team decides which features should be implemented first. This way the most important features are ready as soon as possible and have more time to become stable and integrate with the rest of the software.

![Diagram of the incremental life cycle model](./images/project_management/incremental_model.png){width=60%}

This model can make use of strictly sequential phases (detail planning -> release -> detail planning -> release ...) or introduce some parallelism (for instance planning and developing frontend and backend at the same time).

As seen from the diagram, the high-level analysis and planning are not repeated, instead the detail planning and release cycle for a well-defined number of iterations, and on each iteration we will have a working release or prototype.

### Evolutionary Model

![Diagram of the evolutionary life cycle model](./images/project_management/evolutionary_model.png){width=60%}

<!-- TODO: Talk about the evolutionary life cycle: development, delivery, feedback, incorporation of feedback and back to development -->
\placeholder

### Agile Software Development

Agile Software Development was born as a reaction to the excessive rigidity of the models we've seen so far. The basic principles of Agile Software Development are presented at the <http://agilemanifesto.org> website, but we will discuss them below.

- Rigid rules are not good;
- A working software is more important than a comprehensive documentation;
- Seek collaboration with the stakeholder instead of trying to negotiate with them;
- Responding to change is better than following a plan
- Interactions and individuals are more important than processes and tools.

Obviously not everything that shines is actually gold, there are many detractors of the Agile model, bringing on the table some criticism that should be noted:

- The agile way of working entails a really high degree of discipline from the team: the line between "flexibility" and "complete lack of rules" is a thin one;
- Software without documentation is a liability more than an asset: commenting code is not enough - you need to know (and let others know) the reason behind a certain choice;
- Without a plan, you can't estimate risks and measure how the project is coming along;
- Responding to change can be good, but you need to be aware of costs and benefits such change and your response entail.

#### User Stories

Agile models are based on "User Stories", which are documents that describe the problem at hand.

Such documents are written by talking with the stakeholder/customer, listening to them, actively participating in the discussion with them, proposing solutions and improvements actively.

A User Story also defines how we want to check that the software we are producing actually satisfies our customer.

#### Scrum

<!-- TODO: Talk about the scum approach -->
\placeholder

#### Kanban

<!-- TODO: Talk about the kanban approach -->
\placeholder

#### ScrumBan

<!-- TODO: Talk about scrumban, a mix of scrum and kanban, usually used as a migration stage between the two -->
\placeholder
