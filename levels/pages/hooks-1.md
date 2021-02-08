---
title: "hooks-1"
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

![hook](https://media.giphy.com/media/sQESdTrVdURNu/giphy.gif "hook")

**For this stage, you must run the script ./setup_hooks_stage.sh first!**

You need to add more than 100 files into the add_files_here directory, and you can't do it in more than 3 commits. Shouldn't be too hard, should it?

*Once you've solved this stage, be sure to undo what `setup_hooks_stage.sh` does 😊*

## 🧩 Hints

Click on the hint to see it.

{{% expand "How to create many files in bash" %}}

```sh
for x in {1..5} ; do touch file$x ; done
```

{{% /expand %}}

{{% expand "What are hooks? 🎣" %}}
[Here's some documentation](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks).
{{% /expand %}}
