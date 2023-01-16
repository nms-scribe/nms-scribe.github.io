#!/bin/env node
/*
Copyright © 2023 Neil M. Sheldon

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/


/* This script creates a search index for use in a client-side search algorithm by crawling the index.html file and all local links inside an element with the 'page-content' id.. The search index is simply an array of pages containing a title, url, and the string content extracted from the HTML body. This index is then stringified and compressed, and inserted into a script that will decompress and parse the index on load. The script is placed in "/assets/js". The "decompression" script is also copied into that directory.

A couple of data attributes can be used to limit the search. These are just boolean attributes, although you can give them an empty string value.
* data-search-dont-scan: If this is present on an <a> element, that link will not be crawled.
* data-search-dont-index: If this is present on the 'page-content' element, then the page will be scanned, but its content will not be indexed. The site's layout templates should automatically apply this to any page or post marked with "dont_index" attribute in its YAML front matter.

The compression is done to reduce file size. When this script was first created, it reduce the file size from 68k to 46k, so about 2/3 of the size.

FUTURE: There is a "compression streams" w3c draft which may someday make the compression script I'm using obsolete, but we'll see.
*/

// NOTE: Unless they finish the "compression streams" API and I can find a library using the same algorithm in Rust, I need to keep this here to make sure I'll be able to decompress with the same code.

const path = require('path');
const fs = require('fs');
const jsdom = require('jsdom');
const colors = require('colors');
const LZString = require('lz-string');
const WEBSITE_FILE_PATH = path.join(process.cwd(),"_site");
const HOME_PAGE_PATH = path.join(WEBSITE_FILE_PATH,"index.html");
const SEARCH_INDEX_PATH = path.join(process.cwd(),"assets","js","site_search_index.js");
const LZSTRING_SCRIPT_TARGET_PATH = path.join(process.cwd(),"assets","js","lz-string.js");

const error = function(message) {
    console.error(colors.red(message));
}

const walk_site = function() {
    const document_set = new Set();
    const search_index = [];

    const scan_file = function(html_file) {
        console.log(`scanning ${html_file}...`);
        const html = fs.readFileSync(html_file,{ encoding: "utf8" });
        const dom = new jsdom.JSDOM(html);
        const content = dom.window.document.querySelector('#page-content');
        if (!content.hasAttribute("data-search-dont-index")) { // this can be set on the default layout by specifying a dont_index attribute in the frontmatter yaml.
            const title = dom.window.document.querySelector('html > head > title').textContent;
            const text = content.textContent;
            search_index.push({
                title: title,
                content: text,
                url: path.relative(WEBSITE_FILE_PATH,html_file)
            });
        }
        document_set.add(html_file);
        const links = dom.window.document.querySelectorAll('a:not([data-search-dont-scan])');
        for (var link of links) {
            // When running Jekyll locally, baseurl appears to point to '/'. So, we only need to look at files that start with '/'
            let href = link.href;
            if ((href != '/') && href.startsWith('/')) {
                // get rid of query string and anchors
                href = href.split('?')[0].split('#')[0];
                // convert %20 to ' '
                href = href.replaceAll('%20',' ');
                if (href == '/') {
                    // if that leaves us with a single slash, assume it's going to the index.
                    href = '/index.html'
                }
                let link_path = path.join(WEBSITE_FILE_PATH,href);
                if (link_path.endsWith('/')) {
                    // You can turn off searching the link with data-no-search. It's pretty much useless to actually link to a directory.
                    error(`Did you mean to link to a directory "${href}" in "${html_file}"?`);
                    continue;
                }
                if (path.extname(link_path) == '') {
                    // Jekyll seems to let you link to a page by name without the ".html", so I'll allow this.
                    link_path += ".html";
                }
                if (!document_set.has(link_path)) {
                    if (path.extname(link_path) == ".html") {
                        try {
                            scan_file(link_path)
                        } catch (e) {
                            if (e.code == "ENOENT") {
                                error(`Broken internal link to "${href}" in "${html_file}"`);
                                // don't add to document_set, the link might appear in multiple places that will have to be fixed.
                                // This might also be a link from the 'posts' or a category page if it appeared in the excerpt, and
                                // we would miss where it's actually supposed to go.
                            } else {
                                throw e;
                            }
                        }
                    } else {
                        console.log(`skipping ${href}...`);
                        // add to document set so we don't get so many notices...
                        document_set.add(link_path);
                    }
                } // otherwise, it's already been scanned
            }
        }
    }

    scan_file(HOME_PAGE_PATH);
    return search_index;

}

const main = function() {
    let search_index = walk_site();

    let stringified_index = JSON.stringify(search_index);
    let compressed_index = LZString.compress(stringified_index);
    let stringified_compressed_index = JSON.stringify(compressed_index);
    
    
    fs.writeFileSync(SEARCH_INDEX_PATH,"// NOTE: This script is automatically generated, do not bother editing.\n\
    \n\
    const SITE_SEARCH_INDEX = JSON.parse(LZString.decompress(" + stringified_compressed_index + "))\n");
    console.log(require.resolve("lz-string"));
    fs.copyFileSync(require.resolve("lz-string"),LZSTRING_SCRIPT_TARGET_PATH);
    
}

if (require.main === module) {
    main();
}