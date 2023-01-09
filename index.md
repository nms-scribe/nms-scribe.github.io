---
layout: default
---

{% comment %}
<!---
TODO: Next:
[ ] Announce the new blog on the old blog and on social media
[ ] Comments - One problem with 'issues' is that I have to create the issue, and if there are never any comments then there's no need for an issue. Not to mention, it requires logging in to Github, doesn't it? I need some way of doing anyonymous comments. How does the blogger stuff do that?

FUTURE: I'm seriously contemplating rewriting this whole thing in a language I know just to get rid of the ruby configuration files and other weirdness such as automatically creating style.css even when I want something else.
 -->
{%endcomment%}

# N M Sheldon

I am an amateur (for now) author, role-playing game designer, fantasy cartographer, software developer, husband and father. At least, those are what I think of myself as.

I live in rural eastern Wisconsin, USA, and am a member of my [local writer's group](<https://fdlw.wordpress.com/>). If you wish to contact me, use the options available through the social media links in the menu.

(he/him)

## About This Blog

This blog is dedicated to updating the world about the creative projects I’m working on, whether it wants to hear them or not. I may also post some articles on random thoughts and happenings.

As is tradition, this blog will probably begin with a burst of posts over the next few months, followed by a gradual decline until it goes ten or twenty years without anything new.

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
<li><a href="{{site.baseurl}}/series/{{post.series}}">{{post.series}}</a></li>
{%endif%}{% endfor %}
{% endif %}
</ul>

## Categories

<ul>

{% for category in site.categories %} 
{%comment%}
<!-- Apparently site.categories is an array of tuples, with the category name followed by all of the actual content in that category, hence why we need to use the | first filter. -->
{%endcomment%}
  <li><a href="{{ site.baseurl }}/categories/{{category | first }}.html" name="{{ category | first }}">{{ category | first }}</a></li>
{% endfor %}
{%comment%}
<!-- FUTURE: Delete this if I ever get rid of the uncategorized posts -->
{%endcomment%}
  <li><a href="{{ site.baseurl }}/categories/Uncategorized.html" name="uncategorized">Uncategorized</a></li>
</ul>

