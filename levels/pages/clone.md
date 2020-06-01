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

This is the first level of the challenge. You need to `clone` a repository using `ssh`. The server is `localhost`, the user is `gamemaster`, and the repository's path on the server is `~/ctf-repo`. So that means you need to run the following command:

```sh
git clone gamemaster@localhost:~/ctf-repo
```

Once you've done this, run `cd ctf-repo` to change the working directory to the cloned repository. You should see changes in your shell (like `master`). It should look something like this:

![Clone level screenshot](https://i.imgur.com/3fJ51oe.png "Clone level screenshot")

Finally, read the `README.md` file to see what you need to do to get to the next level - `start-here`.

Good luck!

## 🧩 Hints

Click on hints to reveal them.

{{% expand "`clone`?" %}}

[Here's some documentation about cloning](https://git-scm.com/book/en/v2/Git-Basics-Getting-a-Git-Repository). If you're new to `git`, you really should read this!

{{% /expand %}}

{{% expand "How do I read a file?" %}}

Run `[cat](https://man7.org/linux/man-pages/man1/cat.1.html) README.md`.

{{% /expand %}}
