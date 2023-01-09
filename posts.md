---
layout: default
dont_index: true
---

# Posts

{% for post in site.posts %}
{% include post_preview.html %}
{% endfor %}

{%comment%}
<!-- FUTURE: I don't think this will be a problem for a while, but if there get to be too many posts, I may want to paginate them. This will probably require some sort of scripting. The degault paginate plugin seems annoying to set up. But it shouldn't be difficult to create "posts-1" "posts-2" pages which show them in groups of twenty five or so. -->
{%endcomment%}

