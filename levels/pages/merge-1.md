---
title: "merge-1"
date: 2020-05-23T13:04:22+03:00
draft: false
scripts: 
  - "/js/vis-network.min.js"
twitter:
  card: "summary_large_image"
  site: "@ShayNehmad"
  creator: "@ShayNehmad"
  title: "mrnice.dev"
  description: "mrnice.dev: Shay Nehmad's blog 🧔"
  image: "https://i.imgur.com/ROzkHYp.png"
---

{{< levelgraph >}}

There's a script called `runme.py`. Try to run it.

![staircase](https://media.giphy.com/media/6laM7KdXvzBba/giphy.gif)

Once you're done with this level, remember to push!

## 🧩 Hints

Click on hints to reveal them.

{{% expand "How do I run it?" %}}
`./runme.py`

See why in [this AskUbuntu thread](https://askubuntu.com/questions/320632/why-do-i-need-to-type-before-executing-a-program-in-the-current-directory).
{{% /expand %}}

{{% expand "I ran it, now what?" %}}
Well, it didn't work, and raised an exception. What does the error say?
{{% /expand %}}

{{% expand "I got `not something we can merge` (⊙_⊙;)" %}}
Make sure you're trying to merge something with the correct name - remember that there are remote branches and local branches, and [they are different](https://stackoverflow.com/a/16408515/4119906).
{{% /expand %}}

{{% expand "Useful commands for this level" %}}
`git merge` ([documentation](https://git-scm.com/docs/git-merge) and [explanation](https://git-scm.com/book/en/v2/Git-Branching-Basic-Branching-and-Merging)).
{{% /expand %}}
