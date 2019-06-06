# Contributing

The codebase is maintained using the "contributor workflow" where everyone without exception
contributes patch proposals using "pull requests".  This facilitates social contribution, easy
testing and peer review.

To contribute a patch, the workflow is as follows:

  1. Fork repository
  1. Create topic branch
  1. Commit patches
  1. Push changes to your fork
  1. Create pull request

In general [commits should be
atomic](https://en.wikipedia.org/wiki/Atomic_commit#Atomic_commit_convention) and diffs should be
easy to read.  For this reason do not mix any formatting fixes or code moves with actual code
changes in the same commit.

Commit messages should be verbose by default consisting of a short subject line (50 chars max), a
blank line and detailed explanatory text as separate paragraph(s), unless the title alone is
self-explanatory (like "Corrected typo in Makefile") in which case a single title line is
sufficient.  Commit messages should be helpful to people reading your code in the future, so
explain the reasoning for your decisions.  Further explanation
[here](http://chris.beams.io/posts/git-commit/).

If a particular commit references another issue, please add the reference. For example: `refs
#1234` or `fixes #4321`. 

Please refer to the [Git manual](https://git-scm.com/doc) for more information about Git.

If a pull request is not to be considered for merging (yet), please prefix the title with [WIP] or
use [Tasks Lists](https://help.github.com/articles/basic-writing-and-formatting-syntax/#task-lists)
in the body of the pull request to indicate tasks are pending.

The body of the pull request should contain enough description about what the patch does together
with any justification/reasoning.  You should include references to any discussions (for example
other tickets or chat/mail discussions).

At this stage one should expect comments and review from other contributors.  You can add more
commits to your pull request by committing them locally and pushing to your fork until you have
satisfied all feedback. Please don't squash/rebase your branch until it has been accepted.
