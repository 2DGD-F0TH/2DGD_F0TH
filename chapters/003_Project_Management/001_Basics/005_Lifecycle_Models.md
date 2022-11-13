Software Life Cycle Models
---------------------------

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

![Diagram of the waterfall life cycle model](./images/project_management/waterfall_model.svg){width=60%}

This makes the Waterfall life cycle model *extremely rigid*, everything needs to be carefully analyzed and documented (sometimes people define this model "document-driven") and the coding is done only in its final phases.

In order to have a good result, this model requires quantifying some metrics (time spent, costs, ...) and such quantification heavily relies on the experience of the project manager and the administrators.

### Incremental Model

When a project of a certain size is involved, it's a bad idea to perform the so-called "big-bang integration" (integrating all the components together). Such approach would make troubleshooting a nightmare, so it's advisable to *incrementally integrate* the components.

The Incremental Model allows to have a "high-level analysis and planning", after that the team decides which features should be implemented first. This way the most important features are ready as soon as possible and have more time to become stable and integrate with the rest of the software.

![Diagram of the incremental life cycle model](./images/project_management/incremental_model.svg){width=60%}

This model can make use of strictly sequential phases (detail planning -> release -> detail planning -> release ...) or introduce some parallelism (for instance planning and developing frontend and backend at the same time).

As seen from the diagram, the high-level analysis and planning are not repeated, instead the detail planning and release cycle for a well-defined number of iterations, and on each iteration we will have a working release or prototype.

### Evolutionary Model

It's not always possible to perfectly know the outline of a problem in advance, that's why the evolutionary model was invented. Since needs tend to change with time, it's a good idea to maintain life cycles on different versions of your software at the same time.

![High-level diagram of the evolutionary life cycle model](./images/project_management/evolutionary_model_2.svg){width=60%}

Adding a way to implement the feedback you get from your customers and stakeholders completes the micro-managed part of the life cycle model, each time feedback and updates are implemented, a new version is released.

![Diagram of the evolutionary life cycle model](./images/project_management/evolutionary_model.svg){width=60%}

### Agile Software Development

Agile Software Development was born as a reaction to the excessive rigidity of the models we've seen so far. The basic principles of Agile Software Development are presented at the <http://agilemanifesto.org> website, but we will shortly discuss them below.

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

The term "scrum" is taken from the sport of American Football, where you have an action that is seemingly product of chaos but that instead hides a strategy, rules and organization.

Let's see some Scrum terminology:

- **Product Backlog:** This is essentially a "todo list" that keeps requirements and features our product must have;
- **Sprint:** Iteration, where we choose what to do to create a so-called "useful increment" to our product. Each Sprint lasts around 2 to 4 weeks and at the end of each sprint you obtain a version of your software that can be potentially sold to the consumer;
- **Sprint Backlog:** Essentially another "todo list" that keeps the set of user stories that will be used for the next sprint.

As seen from the terminology, the Scrum method is based on well-defined iterations (Sprints) and each sprint is composed by the following phases:

- **Sprint Planning:** You gather the product backlog and eventually the previous sprint backlogs and decide what to implement in the upcoming sprint;
- **Daily Scrum:** A daily stand-up meeting that lasts around 15 minutes where a check on the daily progress is done;
- **Sprint Review:** After the sprint is completed, we have the verification and validation of the products of the sprint (both software and documents);
- **Sprint Retrospective:** A quality control on the sprint itself is done, allowing for continuous improvement over the way of working.

##### Criticisms to the Scrum approach

The Scrum approach can quickly become chaotic if User Stories and Backlogs are not well kept and clear. Also, no matter how short it can be, the Daily Scrum is still an invasive practice that interrupts the workflow and requires everyone to be present and ready.

#### Kanban

Kanban is an Agile Development approach taken by the scheduling system used for lean and just-in-time manufacturing implemented at Toyota.

The base of Kanban is the "Kanban Board" (sometimes shortened as "Kanboard"), where plates (also called "cards" or "tickets") are moved through swimlanes that can represent:

- The status of the card (To Do, Doing, Testing, Done)
- The Kind of Work (Frontend, Backend, Database, ...)
- The team that is taking care of the work

The board helps with organization and gives a high-level view of the work status.

![Example of a Kanban Board](./images/project_management/kanboard.svg){width=70%}

#### ScrumBan

ScrumBan is a hybrid approach between Scrum and Kanban, mixing the Daily Scrum and Sprint Approach with the Kanban Board.

This approach is usually used during migration from a Scrum-Based approach to a purely Kanban-based approach.

### Lean Development

Lean development tries to bring the principles of lean manufacturing into software development. The basis of lean development is divided in 7 principles:

- **Remove Waste:** "waste" can be partial work, useless features, waiting, defects, work changing hands...
- **Amplify Learning:** coding is seen as a learning process and different ideas should be tested on the field, giving great importance to the learning process;
- **Decide late:** the later you take decisions, the more assumptions and predictions are replaced with facts, Also strong commitments should happen as late as possible, as they will make the system less flexible;
- **Deliver early:** technology evolves rapidly, and the one that survives is the fastest. If you can deliver your product free from defects as soon as possible you will get feedback quickly, and get to the next iteration sooner;
- **Empower the team:** managers are taught to listen to the developers, as well as provide suggestions;
- **Build integrity in:** the components of the system should work well together and give a cohesive experience, giving the customer and impression of integrity;
- **Optimize the whole:** optimization is done by splitting big tasks into smaller ones which helps finding and eliminating the cause of defects.

### Where to go from here

Obviously the models presented are not set in stone, but are "best practices" that have been proven to help with project management, and not even all of them.

Nothing stops you from taking elements of a model and implement them into another model. For example you could use an Evolutionary Model with a Kanban board used to manage the single increment.
