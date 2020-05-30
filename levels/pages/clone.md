---
title: "clone"
date: 2020-05-23T13:04:22+03:00
draft: false
scripts: 
  - "https://unpkg.com/vis-network/standalone/umd/vis-network.min.js"
twitter:
  card: "summary_large_image"
  site: "@ShayNehmad"
  creator: "@ShayNehmad"
  title: "mrnice.dev"
  description: "mrnice.dev: Shay Nehmad's blog 🧔"
  image: "https://i.imgur.com/ROzkHYp.png"
---

{{< levelgraph >}}

Welcome to `make-git-better` CTF 🚩

This is the first level of the challenge. You need to `clone` a repository using `ssh`. The server is `localhost`, the user is `gamemaster`, and the repository's path on the server is `~/ctf-repo`.

Once you've done this, read the `README.md` file in the repository to see what you need to do to get to the next level - `start-here`.

Good luck!

## 🧩 Hints

Click on hints to see them.

{{% expand "`clone`?" %}}
[Here's some documentation about cloning](https://git-scm.com/book/en/v2/Git-Basics-Getting-a-Git-Repository). You really should read this!
{{% /expand %}}

{{% expand "So what do I need to run?" %}}
To solve this level, run the following command:

```sh
git clone gamemaster@localhost:~/ctf-repo
```

{{% /expand %}}

{{% expand "Hint 1" %}}
[StackOverflow link](https://stackoverflow.com/questions/2047465/how-can-i-delete-a-file-from-a-git-repository).
{{% /expand %}}
