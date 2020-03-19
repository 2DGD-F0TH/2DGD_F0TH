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

### Brainstorming: the good, the bad and the ugly

Brainstorming is an activity that involves the design team writing down all the ideas they possibly can (without caring about their quality yet).

This is a productive activity to perform at the beginning of the game development and design process, but it can be a huge source of feature creep if done further down the line.

After the initial phase of brainstorming, the team analyzes the ideas and discards the impossible ones, followed by the ones that are not "as good as they sounded at first". The remaining ideas can come together to either form a concept of a videogame or some secondary component of it.

In short: brainstorming is a great activity for innovation, but since it's essentially "throwing stuff at a wall and see what sticks" it can also be unproductive or even "excessively productive" and in both cases we end up with nothing in our hands.

### On Sequels

In case your game becomes a hit, you will probably think about making a sequel: this is not inherently a bad thing, but you need to remember some things.

When developing a sequel, you will have to live up to your previous game, as well as the expectations of the players, and this becomes more and more difficult as the "successful sequels" go on.

Not only a sequel must be "as good or better" than its predecessor, but also it should add something to the original game, as well as the established lore (if there is one).

Your time and resource management must be top-notch to be able to "bring more with less", since your resource need cannot skyrocket without a reason.

Also don't get caught in the some kind of "sequel disease" where you end up making a sequel just to "milk the intellectual property": you will end up ruining the whole series.

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

![Diagram of the waterfall life cycle model](./images/project_management/waterfall_model.pdf){width=60%}

This makes the Waterfall life cycle model *extremely rigid*, everything needs to be carefully analyzed and documented (sometimes people define this model "document-driven") and the coding is done only in its final phases.

In order to have a good result, this model requires quantifying some metrics (time spent, costs, ...) and such quantification heavily relies on the experience of the project manager and the administrators.

### Incremental Model

When a project of a certain size is involved, it's a bad idea to perform the so-called "big-bang integration" (integrating all the components together). Such approach would make troubleshooting a nightmare, so it's advisable to *incrementally integrate* the components.

The Incremental Model allows to have a "high-level analysis and planning", after that the team decides which features should be implemented first. This way the most important features are ready as soon as possible and have more time to become stable and integrate with the rest of the software.

![Diagram of the incremental life cycle model](./images/project_management/incremental_model.pdf){width=60%}

This model can make use of strictly sequential phases (detail planning -> release -> detail planning -> release ...) or introduce some parallelism (for instance planning and developing frontend and backend at the same time).

As seen from the diagram, the high-level analysis and planning are not repeated, instead the detail planning and release cycle for a well-defined number of iterations, and on each iteration we will have a working release or prototype.

### Evolutionary Model

It's not always possible to perfectly know the outline of a problem in advance, that's why the evolutionary model was invented. Since needs tend to change with time, it's a good idea to maintain life cycles on different versions of your software at the same time.

![High-level diagram of the evolutionary life cycle model](./images/project_management/evolutionary_model_2.pdf){width=60%}

Adding a way to implement the feedback you get from your customers and stakeholders completes the micro-managed part of the life cycle model, each time feedback and updates are implemented, a new version is released.

![Diagram of the evolutionary life cycle model](./images/project_management/evolutionary_model.pdf){width=60%}

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

![Example of a Kanban Board](./images/project_management/kanboard.pdf){width=70%}

#### ScrumBan

ScrumBan is a hybrid approach between Scrum and Kanban, mixing the Daily Scrum and Sprint Approach with the Kanban Board.

This approach is usually used during migration from a Scrum-Based approach to a purely Kanban-based approach.

### Where to go from here

Obviously the models presented are not set in stone, but are "best practices" that have been proven to help with project management, and not even all of them.

Nothing stops you from taking elements of a model and implement them into another model. For example you could use an Evolutionary Model with a Kanban board used to manage the single increment.

Version Control
----------------

When it comes to managing any resource that is important to the development process of a software, it is vitally important that a version control system is put in place to manage such resources.

Code is not the only thing that we may want to keep under versioning, but also documentation can be subject to it.

Version Control Systems (VCS) allow you to keep track of edits in your code and documents, know (and blame) users for certain changes and eventually revert such changes when necessary. They also help saving on bandwidth by uploading only the differences between commits and make your development environment more robust (for instance, by decentralizing the code repositories).

The most used Version Control system used in coding is Git, it's decentralized and works extremely well for tracking text-based files, like code or documentation, but thanks to the LFS extension it is possible for it to handle large files efficiently.

![An example screen from Git, a version control system](./images/project_management/git_example.png){width=60%}

Other used version control systems are Mercurial and SVN (subversion).

Another useful feature of many version control systems are remote sources, which allow you to upload and synchronize your repositories with a remote location (like GitHub, GitLab or BitBucket for instance) and have it safe on the cloud, where safety by redundancy is most surely ensured.
