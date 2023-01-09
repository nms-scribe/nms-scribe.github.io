#!/bin/env node
/*
Copyright © 2023 Neil M. Sheldon

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
// FUTURE: Convert to rust and add to the publish.rs script

/*
Script reads all posts found in _posts, and creates simple categories pages in a "categories" folder. The category pages
contain only frontmatter, the rendering of these things is defined with the layout YAML property for the file, and the
name of the category passed in that matter as well.

NOTE: The site needs to be rendered into _site before this will work. It will also have to be re-rendered after in order 
for the new pages to be picked up and rendered. If you are running jekyll in serve mode during editing, that's not a problem.
*/

const fs = require('fs');
const path = require('path');
const yaml = require('yaml-front-matter');
const UNCATEGORIZED = "Uncategorized";

const POSTS_PATH = path.join(process.cwd(),"_posts");
const CATEGORIES_PATH = module.exports.CATEGORIES_PATH = path.join(process.cwd(),"categories");
const SERIES_PATH = module.exports.SERIES_PATH = path.join(process.cwd(),"series");

const main = module.exports.main = function() {
    const posts = fs.readdirSync(POSTS_PATH);

    let categories = new Set();
    let series = new Set();
    
    for (post of posts) {
        let contents = fs.readFileSync(path.join(process.cwd(),"_posts",post),"utf-8");
        let data = yaml.loadFront(contents);
        if (!(data.hasOwnProperty("categories")) || (data.categories.length == 0)) {
            categories.add(UNCATEGORIZED);
        } else {
            data.categories.forEach(categories.add,categories);
        }
        if (data.hasOwnProperty("series")) {
            series.add(data.series);
        }
    }
    
    for (category of categories) {
        fs.writeFileSync(path.join(CATEGORIES_PATH,category +".md"),
`---
tag: ${category}
layout: category
title: ${category}
dont_index: true
---`);
    // All the rest of the content is created with the category.html _layout.
    }

    for (series of series) {
        fs.writeFileSync(path.join(SERIES_PATH,series +".md"),
`---
tag: ${series}
layout: series
title: ${series}
dont_index: true
---`);
    // All the rest of the content is created with the series.html _layout.
    }
    
}


if (require.main === module) {
    main();
}
