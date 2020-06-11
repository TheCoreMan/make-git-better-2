---
title: "merge-5"
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

![Tree fail](https://media.giphy.com/media/pwgwO5c3K0Q92/giphy.gif "Tree fail")

`runme.py` is a silly name... I'll call the script `./should_rename_this.py` for now, and I'll rename in another branch. Also, the script needs to print another thing...

This time the conflict is quite difficuly to solve, as the problem isn't only content, it's also **structure** (tree). You can solve it!

## 🧩 Hints

Click on the hint to see it.

{{% expand "Tree? 🌳" %}}
The conflict is caused because both branches renamed the same file - changed the directory tree.

[More info on `git` Tree objects](https://git-scm.com/book/en/v2/Git-Internals-Git-Objects#_tree_objects).
{{% /expand %}}
