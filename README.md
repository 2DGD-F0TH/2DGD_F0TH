2D Game Development: From Zero To Hero
======================================

**Status:** +[Pipeline Status](https://img.shields.io/gitlab/pipeline/penaz/2DGD_F0TH?style=for-the-badge)  ![Project Activity](https://img.shields.io/badge/Activity-Active-brightgreen?style=for-the-badge)

This is a small project that aims to gather some knowledge about game development and make it available to everyone.

As well as being a source of knowledge this project aims to be a learning experience for everyone involved too, by gathering contributions from the community, teaching others how to make a game, teaching algorithms but also learning tips and tricks from people who are more experienced.


Building the book
-------------------

To build the book you need the following software:

- Pandoc
- TexLive or equivalent
- GNU Make or equivalent

Before building the book, you might want to set your programming language in the `metadata.yaml` file. The programming languages currently available are:

- Pseudocode (which is the empty "" option)
- Python (proglang set to "python")

To Build the book you need to clone this repository and use the `make` command from the main directory.

Branches
--------

The `master` branch is automatically compiled into PDF format by the Gitlab CI/CD system and is usually pretty much final (its contents will be expanded but probably not modified), while the `develop` branch is more up-to-date but its content is subject to change.

License
--------

This project is meant to be a free source of knowledge for who wants to develop their first game (but also be a reference book you can come back to), currently this project is under the Creative Commons Attribution-NonCommercial License.

### Why this license?

Because I want all contributions to be accessible by everyone for free but at the same time for the contributors to be given recognition, simple as that.

The chosen license is not a "Free Culture" license, but I want to protect the "free as in price" policy of this project, as well as the "free as in freedom" one. I feel this license is the best compromise.
