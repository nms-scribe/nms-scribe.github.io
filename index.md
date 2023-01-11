---
layout: default
---

{% comment %}
<!---
FUTURE: I'm seriously contemplating rewriting this whole thing in a language I know just to get rid of the ruby configuration files and other weirdness such as automatically creating style.css even when I want something else. A rust-based site builder would be nice, but I need my own templating language.
 -->
{%endcomment%}

# N M Sheldon

I am an amateur (for now) author, role-playing game designer, fantasy cartographer, software developer, husband and father. At least, those are what I think of myself as.

I live in rural eastern Wisconsin, USA, and am a member of my [local writer's group](<https://fdlw.wordpress.com/>). If you wish to contact me, use the options available through the social media links in the menu.

(he/him)

## About This Blog

This blog is dedicated to updating the world about the creative projects I’m working on, whether it wants to hear them or not. I may also post some articles on random thoughts and happenings.

As is tradition, this blog will probably begin with a burst of posts over the next few months, followed by a gradual decline until it goes ten or twenty years without anything new.

If you are looking for something specific, see below for a list of categories, or a list of series. Or use the search input above to do a text search of all posts and pages. There's a section about comments way down at the bottom.

## Recent Posts

{% for post in site.posts limit:3 %}
{% include post_preview.html head_level='h3' %}
{% endfor %}

{%comment%}
<!-- Generate links to first page of series 
https://www.ayush.nz/2022/02/creating-article-series-posts-navigation-jekyll
-->
{%endcomment%}
{% assign series = "" | split: "," %}
{% assign series_posts = "" | split: "," %}
{% assign posts = site.posts | sort: 'date' %}


{% for post in posts %}
    {% if post.series %}
        {% unless series contains post.series %}
        {% assign series = series | push: post.series %}
        {% assign series_posts = series_posts | push: post %}
        {% endunless %}
    {% endif %}
{% endfor %}

{% if series.size > 0 %}
## Series

<ul>
{% for post in series_posts %}{% if post.series %}
<li><a href="{{'series/' | append: post.series | relative_url }}">{{post.series}}</a></li>
{%endif%}{% endfor %}
{% endif %}
</ul>

## Categories

<ul>

{% for category in site.categories %} 
{%comment%}
<!-- Apparently site.categories is an array of tuples, with the category name followed by all of the actual content in that category, hence why we need to use the | first filter. -->
{%endcomment%}
  {% assign category_name = category | first %}
  <li><a href="{{ 'categories/' | append: category_name | relative_url  }}">{{ category_name }}</a></li>
{% endfor %}
{%comment%}
<!-- FUTURE: Delete this if I ever get rid of the uncategorized posts -->
{%endcomment%}
  <li><a href="{{ 'categories/Uncategorized' | relative_url }}">Uncategorized</a></li>
</ul>

## On Commenting

After several days of research, I decided that there are no decent commenting systems for static websites. The best options I've found all require you to log in to some other service. I believe in the power of the shy, polite, anonymous voice, and I want my door open to them. And the only way I can do that without paying money[^1], or building my own server, is using Disqus[^2]. As I know they are a notorious site tracking system and ad server, I've set it up so it is not turned on until *you* turn it on. Just press the button on the bottom of each post to load the links, iframes, scripts, etc., and you will be able to view and add comments.

If you are uncomfortable with Disqus, and want to make your voice heard, you have a number of options, all of which require you to have an account somewhere. A github account will get you into the discussions forum linked over on the left of every page. Other accounts will give you access to the social media links at the top of every page. You're still anonymous to me and the rest of its audience, you're just not anonymous to those companies.

All comments are expected to be respectful, tolerant, legal, rated PG, and non-promotional. All comments require approval, which may take time.

[^1]: My site doesn't get enough traffic to make it worth paying money for it.
[^2]: Disqus offers the ability to comment as a guest with pseudonym and e-mail address, which many other services do not offer. After a few tests, I can't see the e-mail you use with it, and I'm not certain that the e-mail address is verified.

