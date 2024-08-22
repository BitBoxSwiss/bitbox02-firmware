CI Design guidelines

* It is more maintainable to create scripts in `.ci` and then call them from the workflows than to
  have scripts inline in the workflows. However, it is also good to split up scripts in multiple
  steps and jobs depending on what is being done.

* The docker image is rebuilt if the `Dockerfile` or `.containerversion` file is modified. (In case
  of a push event it is also automatically published to docker hub).

* If there are changes in the `Dockerfile`, then `.containerversion` must be updated with an
  unpublished version number.

* We listen to two kinds of events, `pull_request` and `push` using two different workflows,
  `pr-ci.yml` and `ci.yml`.
  * On pull request events, github will checkout a version of the tree that is the PR branch merged
    into the base branch. When we look for what is modifed we can diff HEAD^1 to HEAD. If github
    didn't do this, it would've missed commits added to the base branch since the PR branch was
    forked.

     o--o--o--o <-- (base branch, typically 'master', parent 1)
      \        \
       \        o <-- (HEAD)
        \      /
         o----o <-- Pull requst branch (parent 2)

  * On push events we get hashes of last commit before and after the push. When we look for what
    changed we can diff github.event.before with HEAD.

       o--o--o--o--o--o <-- github.event.after (HEAD)
              \
                github.event.before
