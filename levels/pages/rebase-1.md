---
title: "rebase-1"
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

This level is a little harder. Don't panic!

![change history](https://media.giphy.com/media/3EiExzquN7mzTCVK3m/giphy.gif "change history")

In this level, we have a **base** branch and a **topic** branch.

```plaintext
The base branch is              parallelizing-barnhardtite-base
and the topic branch is         parallelizing-barnhardtite-topic
Notice the difference:                                     ^^^^^
```

The base branch introduced a script called `runme.py`, which prints all the resources in the `runme_resources/` folder.

Try to run it now.

In the topic branch, we will need to add specific resources. To do this, run the `add_resources.sh` script and commit the new changes. **Make sure you also remove the `add_resources.sh` script! There's no need for it in the base branch!**

Instead of merging and pushing, we want to **rebase** the topic branch on the base branch, and push the base branch with the new resources, the new printing, and with a clean linear history.

## 🧩 Hints

Click on the hint to see it.

{{% expand "What? Why?!?! 😱" %}}
[Here's why](https://git-scm.com/book/en/v2/Git-Branching-Rebasing).
{{% /expand %}}

{{% expand "Can't run `add_resources.sh`" %}}
`chmod +x`
{{% /expand %}}
