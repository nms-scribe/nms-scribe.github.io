## Testing

To test on local: `bundle exec jekyll serve`

To test with drafts published: `bundle exec jekyll serve --drafts`

It took a bit of work to get ruby, bundle and jekyll stuff set up on the local machine. I don't recall all the tweaks I had to make, hopefully it goes smoother the next time I have to install.

## New Posts

To add a new blog entry, create it in the `_drafts` folder first. 

**Images.** To include an image, put it also in the same folder and reference it with the following pattern: `'\drafting\image[(<arg>=<value>),*]{<alt-text>}`. The syntax comes from a homemade template language I've not finished working on. Arguments include:
* `source`: (required) The name of the file relative to _drafts.
* `size`: A size to resize the image to. Values include `A5`-`A10` based on the standard paper sizes with those names with a dots per cm of 59, or `<integer>x<integer>` in pixels.
* `format`: The format to resize the image to. Values include `l`, `p`, `landscape`, or `portrait`.
* `watermark`: Whether to place a watermark on the image. Values include `true` or `false`.
* `full-link`: Whether to turn the image into a thumbnail link to a full version of the image. The thumbnail will be size A6 or A7 depending on the format of the converted image. Values include `true` or `false`.

*NOTE:* Some changes were made since the last publish attempt regarding the images, and the script might not work until they are tested.

**Publishing.** To finally publish it:

1) Run `_tools/publish.rs` (requires rust-script). The script will prompt you for which post you wish to publish, and run through a checklist of things to make sure you caught everything. 
2) Make sure the website builds.
3) Push the changes to GitHub.

Modified files will be placed in the _posts and _assets folders as necessary, and the originals will be given a `.published` extension.