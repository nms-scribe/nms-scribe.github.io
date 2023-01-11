---
layout: post
title: 'Inannak: the Website'
date: 2022-11-04 16:34:58.000000000 -05:00
type: post
published: true
categories:
- Inannak
series: Inannak
---
A big part of my [Inannak worldbuilding project](<{{ site.basurl }}/posts/2022-10-17-the-inannak-project.html>) will be the book. This is where you'll find all the information about the world and its inhabitants. It's going to take as much or more work to put that together as any of the mapping. But the good news is that you can read it now. There's a link at the end of this post.<!--more-->

You're probably asking yourself, didn't Neil just announce this project last month? How can the book be finished? I didn't say it's finished. Let me explain.

In a futile attempt to grab attention for my hobby, I'm trying something different. I've already stated that the Inannak project will be open and free online. But I'm not just talking about the finished products. I mean that the whole process will be open. That openness is achieved in two strategies. 

The first strategy are the updates to this blog. Posts in the [Innanak category](<{{ 'categories/Inannak.html' | relative_url }}>) will discuss my progress, ideas and plans.

The second strategy is the book itself, or rather, a website version of it. I'm calling it a "living document", which I explain more there. You can watch Inannak evolve as I work. I will also provide a forum to take comments, suggestions, and promote discussion. This feedback can help make Inannak into a world where you will want to play.

As the first strategy is this blog, I'd like to start with how I put the website together. If you're not interested, you can skip to the end to find the links.


## How to Build a Website the Hard Way

I had once planned for  this post to discuss the process of building the Inannak site. But most of that is off-topic for this blog, and the process was far too complicated to relate.

If you're interested in software and web design, there are plenty of resources for learning how to build a website, and you might find better ways to do it. If you're not interested, then you're not going to be interested in all the details.

So, I'll just describe how the site works. I will use some  jargon, for those inclined to understand what I'm talking about. I'll try to define this for the not so inclined. A warning, though: some of my definitions may not be precise.

### The Content

As you probably don't know, all of the roleplaying game content I'm working on, and some of my writing, is done in LaTeX (a pretentious spelling authorized by it's creators). If you're unfamiliar with that, it's a programming language for writing documents. The beauty of using that sort of tool is that I can share the same content between two or more different products.

For example, when my next adventure is released, I will be providing two PDFs: one letter-sized book, and another designed for small, mobile screens. Both of these products come from the same source text. I don't have to copy and paste between the documents whenever I make an edit. LaTeX allows me to use programming techniques to conditionally change themes, fonts, and even some of the content. It's just a little tough to configure.

The Inannak book is no different. The original source is written in LaTeX. Although LaTeX is usually used for books, I use a tool called LaTeXML to build HTML (web page) files from the same code. I then add in some CSS work (web design jargon), and I've got a working website. It looks good.

And when the time comes to produce a final PDF, or even a physical book to sell, I just need to spend a few days adding some code. From then on, I can update both the website and the book with a single command.

### The Host

I'm hosting my website on GitHub, which is a website programmers use to share code. Accounts are free, and there's plenty of space for a small static website. And, since the nature of this project is like open source, it fits right in. GitHub has a feature which allows me to serve a real website from part of that project, so it can basically be a static website host.

But GitHub has one feature which makes it superior to other free hosting options. It has tools which allow for communication with users and collaborators, including what is essentially a forum. This is perfect for that feedback I was talking about, and I don't need to deal with configuring bulletin board software.

Another advantage of GitHub is that it keeps old versions of its contents. If you know what you're doing, you can download and review older versions of the website. This might be useful if you want to see how the process of building it works.


## The Final Product

So, I hope you can check out the website. It already has a little content, built up from some early notes. There are descriptions of the cosmology, the inhabitants, and even a little mythology. Let me know in the forums what you like, maybe what you don't, and what you think I should do with it.

### The Links:

[The World of Inannak](<https://nms-scribe.github.io/inannak/The%20World%20of%20Inannak.html>).

[The World of Inannak Forums](<https://github.com/nms-scribe/inannak/discussions>).

