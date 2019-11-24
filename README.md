2D Game Development: From Zero To Hero
======================================

**Status:** ![Travis Pipeline Status](https://img.shields.io/travis/Penaz91/2DGD_F0TH?label=Travis%20Build&style=for-the-badge) ![Gitlab Pipeline Status](https://img.shields.io/gitlab/pipeline/Penaz/2DGD_F0TH?label=Gitlab%20Build&style=for-the-badge)  ![Project Activity](https://img.shields.io/badge/Activity-Active-brightgreen?style=for-the-badge)

This is a small project that aims to gather some knowledge about game development and make it available to everyone.

As well as being a source of knowledge this project aims to be a learning experience for everyone involved too, by gathering contributions from the community, teaching others how to make a game, teaching algorithms but also learning tips and tricks from people who are more experienced.

Getting the book
-----------------

This repository is configured to automatically build a PDF file at each push on the `master` branch, both on GitLab (Via its own integrated CI/CD System) and GitHub (via Travis-CI).

### GitLab

You can get a copy of the book by clicking on the "Download" button (usually on the right of the "Find File" button), then click on "build_pdf" under "Previous Artifacts": an "artifacts.zip" will be downloaded, containing the latest version of the book.

### GitHub

You can get a copy of the book by clicking on the "Releases" button (usually between the "branches" and "contributors" buttons) and then select the latest tagged release.


Contributing
-------------

Are you a game developer? A computer science student? Maybe you are an artist that wants to contribute by giving insight into pixel art or music creation?

Welcome aboard! You can contribute to this project by **forking** it and adding your own knowledge to it, after that you can create a pull request. Please check the [contributing](CONTRIBUTING.md) document.

Are you an user that would like to see something added to the book? Feel free to open an issue!

Building the book
-------------------

To build the book you need the following software:

- Pandoc
- TexLive or equivalent
- GNU Make or equivalent

Before building the book, you might want to set your programming language in the `metadata.yaml` file. The programming languages currently available are:

- Pseudocode (which is the empty "" option)
- Pseudocode with basic Syntax Hightlighting (proglang set to "pseudocode")
- Python 3 (proglang set to "python")

To Build the book you need to clone this repository and use the `make` command from the main directory.

Branches
--------

The `master` branch is usually pretty much final (its contents will be expanded but probably not modified), while the `develop` branch is more up-to-date but its content is subject to change.

License
--------

This project is meant to be a free source of knowledge for who wants to develop their first game (but also be a reference book you can come back to), currently this project is under the Creative Commons Attribution-NonCommercial License.

### Why this license?

Because I want all contributions to be accessible by everyone for free but at the same time for the contributors to be given recognition, simple as that.

The chosen license is not a "Free Culture" license, but I want to protect the "free as in price" policy of this project, as well as the "free as in freedom" one. I feel this license is the best compromise.
