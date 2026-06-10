---
title: "rebase-2"
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

![who's on first?](/images/whos-on-first.gif "who's on first?")

In this level, the commits are all out of order. Reorder them using an
interactive rebase, and push them in the correct order.

In case you've never watch the classic sketch "Who's on First?", The correct
order is:

> First base: Who
>
> Second base: What
>
> Third base: I Don't Know

(By the way, if you haven't watched it, you should. It's a classic.)

{{< youtube id="kTcRRaXV-fg" >}}

## 🧩 Hints

Click on the hint to see it.

{{% expand "I'm getting `! [rejected]` | part 1" %}}
Read the error message carefully, and think about what you're actually trying to
do. We are the tip of the current branch behind the remote? No one else is
pushing content to this branch right now...
{{% /expand %}}

{{% expand "I'm getting `! [rejected]` | part 2" %}}
You need to force push. Why? Because you're rewriting history. And that's OK.
{{% /expand %}}

{{% expand "I'm getting `! [rejected]` | part 3" %}}
`git push --force-with-lease`. See [the documentation](https://git-scm.com/docs/git-push#Documentation/git-push.txt---no-force-with-lease).
{{% /expand %}}

{{% expand "Interactive rebase?" %}}
[Really very good Documentation](https://git-scm.com/docs/git-rebase#_interactive_mode).
{{% /expand %}}

{{% expand "Wait, isn't this rewriting history?" %}}
[Yep. And that's OK](https://git-scm.com/book/en/v2/Git-Tools-Rewriting-History).
{{% /expand %}}
