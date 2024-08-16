CI Design guidelines

* Keep as much of scripting as possible in scripts and outside of github action yaml files
* The docker image is rebuilt if the `Dockerfile` or `.containerversion` file is modified.
* On pull request events github will checkout a version of the tree that is PR branch merged into
  the base branch. When we look for what is modifed we can diff HEAD^1 to HEAD.

          o-----o <-- Pull requst branch
         /       \
     o--o--o------o <-- (HEAD)
            \
              github.base_ref (base being merged into, typically master)

* On push events we get hashes of last commit before and after the push. And the last commit after
  is checked out. When we look for what changed we can diff github.event.before to HEAD.

     o--o--o------o <-- github.event.after (HEAD)
            \
              github.event.before
