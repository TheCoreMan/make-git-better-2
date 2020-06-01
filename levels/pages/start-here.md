---
title: "start-here"
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

This stage is just a warm-up and it shouldn't be confusing, difficult, or require any sort of special insight or research. This is basically a test to make sure all your setup is OK. Good luck! 👋

### Your challange is

1. Add 2 files to the root of the repo: `alice.txt` and `bob.txt`.
2. Commit your changes (should be only one commit!).
3. Push your changes to the remote repo.

### What do I do with the flag

You passed this stage of the CTF. *Check out* the next stage 👀

⚠ _Note:_ Don't worry if you see the following error message:

```sh
 ! [remote rejected] start-here -> start-here (pre-receive hook declined)
error: failed to push some refs to 'gamemaster@localhost:~/ctf-repo'
```

It's on purpose. The important part is that you got the flag 🚩

## 🧩 Hints

Click on hints to reveal them.

{{% expand "How can I add files to the filesystem?" %}}
To create files you can use the `touch` command.
{{% /expand %}}

{{% expand "How can I add files to git?" %}}
To add files to the [staging area](https://stackoverflow.com/questions/49228209/whats-the-use-of-the-staging-area-in-git), you can use the `git add` command.
{{% /expand %}}

{{% expand "How do I commit my changes?" %}}
Read about [`git commit`](https://www.atlassian.com/git/tutorials/saving-changes).
{{% /expand %}}

{{% expand "How do push my changes?" %}}
In this case, just running `git push` is enough.
{{% /expand %}}

{{% expand "What if I f-ed it up" %}}
Good! That's a good way to learn. You'll need to figure out how to return things to their original state and start over :)
{{% /expand %}}

{{% expand "What if I f-ed it up, for real" %}}
First, return to the original commit from which you've started the challenge (`git checkout origin/start-here`).
Then, set the local `start-here` branch to the current HEAD (`git branch -f start-here`).
Now you can try to solve again.
{{% /expand %}}
