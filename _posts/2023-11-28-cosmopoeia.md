---
categories:
- Worldbuilding
- Programming
layout: post
type: post
title: Cosmopoeia
date: 2023-11-28 09:46:02.000000000 -06:00
published: true
---


When I find a monotonous task, I want to spend more time looking for shortcuts than completing the task itself[^1]. When it's a computer task, the added challenge of building a new program makes the distraction worse. This can be a problem, and I've learned tricks to avoid it, such as looking for previous work. But sometimes that previous work isn't quite what I need. And sometimes the idea sticks in my head and I can't get it out until I do something about it. This post is about one such project.<!--more-->

I call it [Cosmopoeia](https://github.com/nms-scribe/cosmopoeia)[^2]. It's a command line program. Run it, and it will create a fantasy world in a file that you can open up in a GIS program. If you want to know the details on how to use it and how it works, or see some screenshots, [go to the GitHub site](https://github.com/nms-scribe/cosmopoeia).

## How It Happened

If you've read my posts on building the world [Inannak](<series/Inannak>), you may have seen between the lines my frustration with the tools I was using. For years, I've been searching for tools to help me with that sort of work. While I could do all of that by hand, it takes a lot of care to make sure things are actually right. Coastlines must be wiggly in the right places. Rivers must travel downhill. Forests and grasslands must be scattered carefully. When guessing at national borders, it's easy to over-favor straight lines, or over-compensate the other way once you realize that's what you're doing[^3].

I've played with a lot of software to help me over the years. My goal is to be able to pull the map into [QGIS](https://www.qgis.org/en/site/), where I can use its tools to create nice looking maps. Most only get me so far. Generally, the tools I've looked at are built either for scientific simulations, video game development, or role-playing games. 

Scientific tools assume real-world data, require working knowledge beyond my education, and use software tools that are a burden to install and take long periods of time to run. Video game development targets 3D graphics over the cartography, and focuses on smaller regions than I am interested in. Role-playing game tools, at least as trends are going these days, focuses more on realistic illustration than abstract mapping. 

So far, the project that I have found that has come closest to what I want is [Azgaar's Fantasy Map Generator](https://azgaar.github.io/Fantasy-Map-Generator/) (AFMG). In a few seconds, it can generate an entire fantasy world for you just by opening that link up in your browser. It can use an existing heightmap, or generate the terrain itself. It plots out the biomes, the population, the countries, everything. The problem I had with it was that it was all done in the browser. While I could export most of the data into GIS files, working in the browser and tweaking my creation in the first place is slow and clunky.

As I was using it to do some work in Inannak, the thought occurred to me that I could do something similar. I realized that half to three quarters of the work Azgaar spent on that program was the user interface. That user interface was full of DIY code, and if I could avoid that it would be easier. If I were to build this myself, I wouldn't need to do that if I output map files that QGIS could load. The user interface would thus be QGIS. I could write this in Rust, and port a lot of the algorithms over from AFMG. Maybe, I could change some things in those algorithms that would fix things I found myself tweaking in AFMG.

After thinking about this a few weeks, I started playing around with it to see how much it would take. The next thing I knew, I was fully deep into it, despite knowing it would take me at least a few weeks to finish. 

Five months later, I have it running. So, no, it wasn't as easy as I thought it would be. 

It also wasn't easy to come up with a way to announce it, considering it's far from "complete". I've sat on it for at least a month while the rest of my life takes priority. But I want people to know about it, so the work is at least worth something. Confession time: it is rough yet. It has some failings of those scientific tools in how much work it takes to get it running, but it does run.

If you are interested in this sort of thing, I hope you enjoy it.

[^1]: This is how I turned a data entry position into a programming career. When I learned the tool I used to enter catalog orders was an in-house script, I edited it to streamline some tasks. The bosses found out, and instead of firing me, transferred me to the I.T. department. It was enough to trick my next few bosses into thinking I was a programmer, and I literally faked it until I made it.

[^2]: The name was inspired by the word [mythopoeia](https://en.wikipedia.org/wiki/Mythopoeia).

[^3]: I could write an article on how borders work, and why so many new world-builders get them wrong. They aren't just random wiggly lines. Cosmopoeia still gets them wrong, but I hope I can fix that someday.