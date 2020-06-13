---
title: "clone"
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

Welcome to `make-git-better` CTF 🚩

![make-git-better-ctf-logo](https://i.imgur.com/yc8VW3J.png)

- [Notes for beginners](#notes-for-beginners)
  - [🧱 CTF structure](#ctf-structure)
  - [🤔 Unsure what to do?](#unsure-what-to-do)
- [▶ I am ready to play](#i-am-ready-to-play)
  - [Connect to the game server](#connect-to-the-game-server)
  - [Clone the repository](#clone-the-repository)
- [🧩 Hints](#hints)

## Notes for beginners

### 🧱 CTF structure

This CTF, like most other CTFs, is separated into levels. You start at the "clone" level (where you are right now), and try to finish it. When you'll finish a level, you will get some information about how to start the next one!

The pages on this website for the different levels can be accessed via the **level graph browser**:

- The current level is marked with a 📍. For example, you are now in the `📍 clone` level.
- Click on a level to go to its page.
- You can zoom in and out and drag the browser around to take a look, as well.

{{< levelgraph >}}

Each page will contain information about that stage, and also some hints. All levels in this game have a page on this website, and they are all linked to from the level graph browser.

### 🤔 Unsure what to do?

There are several things you can try when you are unsure how to continue:

- First, if you know a command, but don’t know how to use it, try the manual (man page) by entering `man <command>` or `git help <command>`. E.g. if you want to learn more about the `git commit` command, type: `git help commit`. The `man` command also has a manual, try it. Press `q` to quit the `man` command.
- Second, if there is no `man` page, the command might be a shell built-in. In that case use the `help <X>` command. E.g. `help cd`
- Also, your favorite search-engine is your friend. Learn how to use it! 🔍
- For each stage, there are hints. You can use the hints if you want. [See the hints for this level](#hints).
- If you are still stuck, you can [reach out to me](https://www.mrnice.dev/about/#nc-shay-nehmad-443) and I'll try my best to help you out.

## ▶ I am ready to play

This is the first level of the challenge. The goal of this level is for you to log into the game using SSH and clone the game repository.

### Connect to the game server

The host to which you need to connect is `ctf.mrnice.dev`, on port `12345`. The username is `player` and the password is `player`. So:

```sh
ssh player@ctf.mrnice.dev -p 12345
```

The game server has some stuff installed. [See the CTF intro page for more details](../../ctf).

### Clone the repository

You need to `clone` a repository using `ssh`, as well. The server is `localhost`, the user is `gamemaster`, and the repository's path on the server is `~/ctf-repo`. So that means you need to run the following command:

```sh
git clone gamemaster@localhost:~/ctf-repo
```

Once you've done this, run `cd ctf-repo` to change the working directory to the cloned repository. You should see changes in your shell (like `master`). It should look something like this:

![Clone level screenshot](https://i.imgur.com/3fJ51oe.png "Clone level screenshot")

Finally, read the `README.md` file to see what you need to do to get to [the next level: `start-here`](../start-here).

Good luck!

## 🧩 Hints

Click on hints to reveal them.

{{% expand "`ssh`?" %}}

[How to use SSH on WikiHow](https://www.wikihow.com/Use-SSH).

{{% /expand %}}

{{% expand "`clone`?" %}}

[Here's some documentation about cloning](https://git-scm.com/book/en/v2/Git-Basics-Getting-a-Git-Repository). If you're new to `git`, you really should read this!

{{% /expand %}}

{{% expand "How do I read a file?" %}}

Run `cat README.md`. Here's information about [cat](https://man7.org/linux/man-pages/man1/cat.1.html).

{{% /expand %}}

Attribution[^1]

[^1]: These notes were adapted from [OverTheWire Bandit Intro Page](https://overthewire.org/wargames/bandit/).
