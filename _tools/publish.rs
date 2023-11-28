#!/usr/bin/env -S rust-script --cargo-output
//! This is a regular crate doc comment, but it also contains a partial
//! Cargo manifest.  Note the use of a *fenced* code block, and the
//! `cargo` "language".
//!
//! ```cargo
//! [dependencies]
//! chrono = "0.4.23" # FUTURE: This is a rather heavy package for just dealing with dates and times
//! serde_yaml = "0.9.16"
//! regex = "1.7.0"
//! slugify = "0.1.0"
//! enum_dispatch = "0.3.9"
//! ```
/*
Copyright © 2023 Neil M. Sheldon

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/


// Ordinarily, I'd just address the warnings from cargo build or cargo test before cargo run. But as this script is automatically cargo run, I can't really do that. So this means you can't run it, so there. Nanananaa!
#![deny(warnings)] 

// FUTURE: Get republishing of images done when we need it.

use std::path::PathBuf;
use std::path::Path;
use std::fs::read_to_string;
use std::fs::write;
use std::fs::create_dir_all;
use std::fs::rename;
use std::error::Error;
use std::fmt::Display;
use std::process::ExitCode;
use std::str::FromStr;
use std::fmt::Formatter;
use std::process::Command;
use std::ffi::OsString;
use std::rc::Rc;
use std::cell::RefCell;

use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::Timelike;
use chrono::Datelike;
use regex::Regex;
use regex::Captures;
use serde_yaml::Mapping;
use serde_yaml::Value;
use slugify::slugify;
use enum_dispatch::enum_dispatch;

mod io {
    use std::io;
    use std::io::Write;
    pub use std::io::Error;
    use std::fmt::Display;
    use std::path::Path;
    use std::fs::DirEntry;
        

    pub fn read(prompt: &str) -> Result<String,io::Error> {
        let mut stdout = io::stdout();
        stdout.write(prompt.as_bytes())?;
        stdout.write(b" ")?;
        stdout.flush()?;
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        Ok(user_input.trim().to_owned())
    }

    pub fn read_valid<Data, Validator: Fn(&str) -> Result<Data,&str>>(prompt: &str, validate: Validator, elaboration: Option<&str>) -> Result<Data,io::Error> {
        loop {
            let answer = read(prompt)?;
            match validate(&answer) {
                Ok(result) => return Ok(result),
                Err(err) => {
                    write_warning(err)?;
                    if let Some(elaboration) = elaboration {
                        write(elaboration)?;
                    }
                }
            }
        } 
    }

    pub fn read_integer<IntType>(prompt: &str, min: Option<IntType>, max: Option<IntType>) -> Result<IntType,io::Error>
    where IntType: std::cmp::PartialOrd + 
                   std::str::FromStr + 
                   Display + 
                   std::ops::Add<Output = IntType> + 
                   std::ops::Sub<Output = IntType> +
                   std::convert::From<u8> +
                   Copy {

        let prompt = match (min,max) {
            (None,None) => prompt.to_owned(),
            (Some(min),None) => format!("[{}+] {}",min,prompt),
            (None,Some(max)) => format!("[-{}] {}",max,prompt),
            (Some(min),Some(max)) => format!("[{}-{}] {}",min,max,prompt)
        };

        let elaboration = match (min,max) {
            (None,None) => format!("Please enter an integer."),
            (Some(min),None) => format!("Please enter an integer greater than {}.",min-1.into()),
            (None,Some(max)) => format!("Please enter an integer less than {}.",max+1.into()),
            (Some(min),Some(max)) => format!("Please enter an integer from {} to {}.",min,max)
        };

        read_valid(&prompt, |answer| {
            if let Ok(answer) = answer.parse::<IntType>() {
                match (min,max) {
                    (Some(min),_) if answer < min => {
                        Err("Input is too low.")
                    },
                    (_,Some(max)) if answer > max => {
                        Err("Input is too high.")
                    },
                    _ => Ok(answer)
        
                }


            } else {
                Err("Input is not an integer")
            }

        },Some(&elaboration))
        
    }

    pub fn read_choice<ChoiceType: Display, ChoiceList: Iterator<Item=ChoiceType>>(message: &str, choices: ChoiceList) -> Result<ChoiceType,io::Error> {
        write(message)?;
        let mut items = vec![];
        for (i,choice) in choices.enumerate() {
            write(&format!("[{}] {}",i+1,choice))?;
            items.push(choice);
        }
        let answer = read_integer("?", Some(1), Some(items.len()))?;
        Ok(items.remove(answer - 1))

    }

    pub fn read_yes_no(message: &str) -> Result<bool,io::Error> {
        read_valid(&format!("[Y/N] {}",message), |answer| {
            match answer.to_lowercase().as_ref() {
                "y" | "yes" => Ok(true),
                "n" | "no" => Ok(false),
                _ => Err("Please answer 'Y' or 'N'.")
            }
        }, None)
    }

    fn _write_with_color(message: &str, color: &str) -> Result<(),io::Error> {
        io::stdout().write(format!("\x1b[{}m{}\x1b[0m\n",color,message).as_bytes())?;
        Ok(())

    }

    pub fn write(message: &str) -> Result<(),io::Error> {
        let mut stdout = io::stdout();
        stdout.write(format!("{}\n",message).as_bytes())?;
        stdout.flush()?;
        Ok(())
    }

    pub fn write_warning(message: &str) -> Result<(),io::Error> {
        _write_with_color(message, "33")
    }

    pub fn write_error(message: &str) -> Result<(),io::Error> {
        _write_with_color(message, "31")
    }

    pub fn write_info(message: &str) -> Result<(),io::Error> {
        _write_with_color(message, "32")
    }

    pub fn list_files<ItemType, FilterMap: Fn(DirEntry) -> Option<ItemType>>(dir: &Path, filter: FilterMap) -> Result<Vec<ItemType>,io::Error> {
        let mut list = vec![];
        for result in dir.read_dir()? {
            let file = result?;
            if file.file_type()?.is_file() {
                if let Some(file) = (filter)(file) {
                    list.push(file);
                }
            }
        }
        Ok(list)
    }
    
    

}

fn format_yaml_date(date: DateTime<Local>) -> String {
    // The yaml metadata format used in jekyll has the following not-quite-ISO format, and I want to make sure it remains, just in case:
    // 2022-01-11 11:43:05.000000000 -06:00 -- and the time is in my local timezone already, so I think this is right.
    let tz_offset = date.offset().local_minus_utc();
    let tz_date = NaiveDateTime::from_timestamp_opt(tz_offset.abs().into(),0).expect("Timezone offset should have been valid.");

    format!("{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}.000000000 {}{:0>2}:{:0>2}",
            date.year(),
            date.month(),
            date.day(),
            date.hour(),
            date.minute(),
            date.second(),
            if tz_offset > 0 { "+" } else { "-" },
            tz_date.hour(),
            tz_date.minute()
        )
}

fn confirm_category(category_list: &Vec<String>, category: &str) -> Result<(),Box<dyn Error>> {
    if !category_list.contains(&category.to_string()) {
        if !io::read_yes_no(&format!("Did you intend to create the new category {}?",category))? {
            Err("Please correct property categories in the post metadata.")?
        }
    }
    Ok(())

}

fn confirm_no_watermark(filename: &str) -> Result<(),Box<dyn Error>> {
    if !io::read_yes_no(&format!("Are you sure you don't want to add a watermark to {}?",filename))? {
        Err("Please edit the post and try again.")?
    } else {
        Ok(())
    }
}

fn confirm_no_resize(filename: &str) -> Result<(),Box<dyn Error>> {
    if !io::read_yes_no(&format!("Are you sure you don't want to resize {}?",filename))? {
        Err("Please edit the post and try again.")?
    } else {
        Ok(())
    }
}

fn confirm_no_size_for_thumbnail(filename: &str) -> Result<(),Box<dyn Error>> {
    if !io::read_yes_no(&format!("No size was given for '{}', but a full link was requested. A thumbnail can't be generated without knowing the full size. Are you sure you don't want to specify the size?",filename))? {
        Err("Please edit the post and try again.")?
    } else {
        Ok(())
    }
}

fn confirm_large_image(filename: &str, size: &ImageSize, format: &ImageFormat) -> Result<(),Box<dyn Error>> {
    if !io::read_yes_no(&format!("An image size of {} {} is rather large to include without a thumbnail. Are you sure you don't want to add a 'full-size' attribute to the tag for '{}'?",size,format,filename))? {
        Err("Please edit the post and try again.")?
    } else {
        Ok(())
    }
}


fn set_property_if_not_set(post: &Rc<RefCell<Post>>, property: &str, default: &str, tasks: &mut TaskList) -> Result<String,Box<dyn Error>> {
    let yaml_property = serde_yaml::to_value(property)?;
    let yaml_default = serde_yaml::to_value(default)?;
    if let Some(existing) = post.borrow().frontmatter.get(&yaml_property) {
        io::write_info(&format!("Property {} is set to {:?}",property,existing))?;
        // don't do anything...
        Ok(serde_yaml::from_value(existing.clone())?)
    } else {
        tasks.add_set_property_task(post,property,yaml_default)?;
        Ok(default.to_owned())
    }
}


fn confirm_categories(post: &Rc<RefCell<Post>>, category_list: &Vec<String>) -> Result<(),Box<dyn Error>> {
    let yaml_property = serde_yaml::to_value("categories".to_owned())?;
    let categories = if let Some(existing) = post.borrow().frontmatter.get(yaml_property) {
        Some(existing.clone())
    } else {
        if !io::read_yes_no(&format!("Property categories is not set. Is that correct?"))? {
            Err(format!("Please correct property categories in the post metadata."))?
        } else {
            None
        }

    };
    if let Some(categories) = categories {
        if let Some(categories) = categories.as_sequence() {
            for category in categories {
                if let Some(category) = category.as_str() {
                    confirm_category(&category_list,category)?
                } else if !io::read_yes_no(&format!("Property categories contains a non-string {:?}. Are you sure that is correct?",category))? {
                    Err(format!("Please correct property categories in the post metadata."))?
                } 
    
            }

        } else if let Some(category) = categories.as_str() {
            confirm_category(&category_list,category)?

        } else {
            if !io::read_yes_no(&format!("Property categories is not a sequence or string. Are you sure that is correct?"))? {
                Err(format!("Please correct property categories in the post metadata."))?
            } 
        }

    };

    Ok(())


}



fn confirm_series(post: &Rc<RefCell<Post>>, series_list: &Vec<String>) -> Result<(),Box<dyn Error>> {
    // similar to categories, but if no series is set, there's no need to confirm.
    let yaml_property = serde_yaml::to_value("series".to_owned())?;
    let series = if let Some(existing) = post.borrow().frontmatter.get(yaml_property) {
        Some(existing.clone())
    } else {
        None
    };
    if let Some(series) = series {
        if let Some(series) = series.as_str() {
            if !series_list.contains(&series.to_string()) {
                if !io::read_yes_no(&format!("Did you intend to create the new series {}?",series))? {
                    Err("Please correct property series in the post metadata.")?
                }
            }
        } else {
            if !io::read_yes_no(&format!("Property series is not a string. Are you sure that is correct?"))? {
                Err(format!("Please correct property series in the post metadata."))?
            } 
        }

    };

    Ok(())



}


fn confirm_excerpt(post: &Rc<RefCell<Post>>) -> Result<(),Box<dyn Error>> {
    let yaml_property = serde_yaml::to_value("excerpt".to_owned())?;
    if let Some(_) = post.borrow().frontmatter.get(yaml_property) {
        Ok(())
    } else if post.borrow().body.contains("<!--more-->") {
        Ok(())
    } else {
        if !io::read_yes_no(&format!("You don't have <!--more--> tag or an excerpt property. Are you sure that is correct?"))? {
            Err(format!("Please correct excerpt in the post."))?
        } else {
            Ok(())
        }
    }
}





fn replace_image_tag(attrs: &str, alt: &str, date_slug: &str, environment: &Environment, tasks: &mut TaskList) -> Result<String,Box<dyn Error>> {
    let ImageArguments { alt, filename, size_format, watermark, full_link } = ImageArguments::parse(attrs,alt)?;

    let source = environment.get_original_image_filename(&filename)?;
    let new_file_path = format!("{}/{}",date_slug,filename);

    // confirm some validations
    match &size_format {
        Some((size,format)) => {
            if (size.width(&format) > CONTENT_WIDTH) && !full_link {
                confirm_large_image(&filename,&size,&format)? 
            } 
        },
        None => confirm_no_resize(&filename)?
    }

    if !watermark {
        confirm_no_watermark(&filename)?;
    }


    tasks.add_assets_directory_task_if_not_present(&environment.assets_folder,date_slug)?;
    let target = environment.assets_folder.join(&new_file_path);

    tasks.add_image_task(environment,source,target.clone(),watermark,&size_format)?;

    let _result = if full_link {
        let thumbnail_path = if let Some((size,format)) = size_format {
            if size.width(&format) > CONTENT_WIDTH  {
                // NOTE: This will cause ImageMagick to always convert the thumbnail to png
                let thumbnail_path = format!("{}/{}.thumbnail.png",date_slug,filename);
                let thumbnail_target = environment.assets_folder.join(&thumbnail_path);

                tasks.add_image_task(environment,target,thumbnail_target,false,&Some(size.thumbnail_size(&format)))?;
                
                thumbnail_path
            } else {
                new_file_path.clone()
            }
        } else {
            confirm_no_size_for_thumbnail(&filename)?;
            new_file_path.clone()
        };

        format!("[![{}](<{{{{'assets/' | append: {} | relative_url}}}}>)](<{{{{'assets/' | append: {} | relative_url}}}}>)",alt,thumbnail_path,new_file_path);
        todo!("The full-link attribute hasn't been tested yet.")
    } else {
        format!("![{}](<{{{{'assets/' | append: {} | relative_url}}}}>)",alt,new_file_path);
        todo!("Test the modified image formatting results")
    };


//TODO:    Ok(result)



}


fn fix_images(post: &Rc<RefCell<Post>>, date_slug: &str, environment: &Environment, tasks: &mut TaskList) -> Result<(),Box<dyn Error>> {

    let borrowed = post.borrow();
    match environment.replace_all_draft_image_tags(&borrowed.body,|args,alt| {
        replace_image_tag(args, alt, date_slug, environment, tasks)
    }) {
        Err(err) => Err(format!("Error processing images, please edit post and try again: {}",err).into()),
        Ok(fixed) => {
            if fixed != post.borrow().body {
                tasks.add(UpdateBodyTask::new(post.clone(),fixed).into())?
            }
            Ok(())
        }
    }

}




#[derive(PartialEq,Clone)]
enum ImageFormat {
    Landscape,
    Portrait
}

#[derive(Debug)]
enum ImageFormatParseError {
    UnknownValue(String)
}

impl Error for ImageFormatParseError {}

impl Display for ImageFormatParseError {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            Self::UnknownValue(name) => write!(f,"Unknown image format '{}'",name)
        }
    }
}

impl FromStr for ImageFormat {

    type Err = ImageFormatParseError;

    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> { 
        match input.to_lowercase().as_ref() {
            "l" | "landscape" => Ok(Self::Landscape),
            "p" | "portrait" => Ok(Self::Portrait),
            _ => Err(ImageFormatParseError::UnknownValue(input.to_owned()))
        }
     }

}

impl Display for ImageFormat {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            Self::Landscape => write!(f,"Landscape"),
            Self::Portrait => write!(f,"Portrait")
        }
    }

}


const BASE_DPCM: usize = 59;
const CONTENT_WIDTH: usize = 650; // FUTURE: This is based off of the content-width given in the CSS.

#[derive(PartialEq,Clone)]
enum ImageSize { 
// The pixels assume a dpcm of 59, which is close to a dpi of 150.
// The images will be reduced to fit into the specified size, but retain their original aspect ratio.
    A5, // short_mm:  148, long_mm:  210, short_in: 5.8, long_in: 8.3, short_px: 873, long_px: 1239
    A6, // short_mm:  105, long_mm:  148, short_in: 4.1, long_in: 5.8, short_px: 620, long_px:  873
    A7, // short_mm:   74, long_mm:  105, short_in: 2.9, long_in: 4.1, short_px: 437, long_px:  620
    A8, // short_mm:   52, long_mm:   74, short_in: 2.0, long_in: 2.9, short_px: 307, long_px:  437
    A9, // short_mm:   37, long_mm:   52, short_in: 1.5, long_in: 2.0, short_px: 218, long_px:  307
    A10,// short_mm:   26, long_mm:   37, short_in: 1.0, long_in: 1.5, short_px: 153, long_px:  218
    Custom(usize,usize) // NOTE: This is not width by height, these are two dimensions, the Format specifies which is width and which is height.
}

impl ImageSize {

    fn short_dimension(&self) -> usize {
        match self {
            // The measurements are calculated from mm below because the specifications are in mm.
            // I'm not worried about weird differences due to platform issues.
            Self::A5 => Self::mm_to_px(148),
            Self::A6 => Self::mm_to_px(105),
            Self::A7 => Self::mm_to_px(74),
            Self::A8 => Self::mm_to_px(52),
            Self::A9 => Self::mm_to_px(37),
            Self::A10 => Self::mm_to_px(26),
            Self::Custom(x,y) => *x.min(y) 
        }

    }

    fn long_dimension(&self) -> usize {
        match self {
            Self::A5 => Self::mm_to_px(210),
            Self::A6 => Self::mm_to_px(148),
            Self::A7 => Self::mm_to_px(105),
            Self::A8 => Self::mm_to_px(74),
            Self::A9 => Self::mm_to_px(52),
            Self::A10 => Self::mm_to_px(37),
            Self::Custom(x,y) => *x.max(y) 
        }

    }

    fn mm_to_px(mm: usize) -> usize {
        ((mm as f32)  * 0.1 * (BASE_DPCM as f32)).round() as usize
    }

    fn width(&self, format: &ImageFormat) -> usize {
        match format {
            ImageFormat::Landscape => self.long_dimension(),
            ImageFormat::Portrait => self.short_dimension()
        }
    }

    fn height(&self, format: &ImageFormat) -> usize {
        match format {
            ImageFormat::Landscape => self.short_dimension(),
            ImageFormat::Portrait => self.long_dimension()
        }
    }

    fn thumbnail_size(&self, format: &ImageFormat) -> (ImageSize,ImageFormat) {
        match format {
            ImageFormat::Landscape => (ImageSize::A7,ImageFormat::Landscape),
            ImageFormat::Portrait => (ImageSize::A6,ImageFormat::Portrait),
        }
    }

}

#[derive(Debug)]
enum ImageSizeParseError{
    InvalidImageSize(String)
}

impl Error for ImageSizeParseError {}

impl Display for ImageSizeParseError {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            Self::InvalidImageSize(size) => write!(f,"Invalid value for image size '{}'",size)
        }
    }

}

impl FromStr for ImageSize {

    type Err = ImageSizeParseError;

    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> { 
        match input {
            "A5" => Ok(Self::A5),
            "A6" => Ok(Self::A6),
            "A7" => Ok(Self::A7),
            "A8" => Ok(Self::A8),
            "A9" => Ok(Self::A9),
            "A10" => Ok(Self::A10),
            _ => {
                let mut dimensions: Vec<usize> = vec![];
                for term in input.splitn(2,&['x','X','×']).map(|a| a.trim()) {
                    let mut dimension: f32;
                    let mut chars = term.chars().peekable();
                    if let Some(c) = chars.peek() {
                        if let Some(d) = c.to_digit(10) {
                            chars.next();

                            dimension = d as f32;
                            
                            while let Some(c) = chars.peek() {
                                if let Some(d) = c.to_digit(10) {
                                    chars.next();
                                    dimension = (dimension * 10.0) + d as f32;
                                } else {
                                    break;
                                }
                            }
                            if let Some('.') = chars.peek() {
                                chars.next();
                                let mut place = 1.0;
                                while let Some(c) = chars.peek() {
                                    if let Some(d) = c.to_digit(10) {
                                        chars.next();
                                        place *= 10.0;
                                        dimension = dimension + (d as f32/place);
                                    } else {
                                        break;
                                    }
                                }
        
                            }
                            let unit: String = chars.collect();
                            match unit.to_lowercase().as_ref() {
                                "mm" => dimension = (dimension * 0.1 * (BASE_DPCM as f32)).round(),
                                "cm" => dimension = (dimension * (BASE_DPCM as f32)).round(),
                                "in" | "\"" => dimension = (dimension * 2.54 * (BASE_DPCM as f32)).round(),
                                "px" | "" => dimension = dimension.round(),
                                _ => break
                            }
                            dimensions.push(dimension as usize);                        
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }

                }
                if dimensions.len() == 2 {
                    return Ok(Self::Custom(dimensions[0],dimensions[1]))

                } else {
                    Err(ImageSizeParseError::InvalidImageSize(input.to_owned()))
                }

            }            
        }
        
    }

}

impl Display for ImageSize {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            Self::A5 => write!(f,"A5"),
            Self::A6 => write!(f,"A6"),
            Self::A7 => write!(f,"A7"),
            Self::A8 => write!(f,"A8"),
            Self::A9 => write!(f,"A9"),
            Self::A10 => write!(f,"A10"),
            Self::Custom(a,b) => write!(f,"{}px×{}px",a,b)
        }

    }

}

struct ImageArguments {
    filename: String,
    alt: String,
    size_format: Option<(ImageSize,ImageFormat)>,
    watermark: bool,
    full_link: bool
}

impl ImageArguments {


    fn parse(attributes: &str, alt: &str) -> Result<Self,Box<dyn Error>> {
        let mut filename = None;
        let mut size = None;
        let mut format = None;
        let mut watermark = true;
        let mut full_link = false;

        macro_rules! parse_err {
            ($attr: literal, $value: ident) => {
                $value.parse().map_err(|e| format!("error parsing {} attribute for {}: {}",$attr,filename.clone().unwrap_or_else(|| "an image".into()),e))?
            }
        }
        
        for attribute in attributes.split(",") {
            let attribute = attribute.trim();
            let attribute: Vec<&str> = attribute.splitn(2,"=").collect();
            if attribute.len() > 1 {
                let (name,value) = (attribute[0],attribute[1]);
                match name {
                    "source" => filename = Some(parse_err!("source",value)),
                    "size" => size = Some(parse_err!("size",value)),
                    "format" => format = Some(parse_err!("format",value)),
                    "watermark" => watermark = parse_err!("watermark",value),
                    "full-link" => full_link = parse_err!("full-link",value),
                    _ => Err(format!("Found unknown attribute in a draft.image tag: {}={}",name,value))?
                }
            } else if attribute.len() > 0 {
                if attribute[0] != "" {
                    Err(format!("Found unknown attribute in a draft.image tag: {}",attribute[0]))?
                }
            }
        }

        let filename = if let Some(filename) = filename  {
            filename
        } else {
            Err("A draft.image tag was found without an image name.")?
        };
        
        let size_format = match (size, format) {
            (Some(_),None) => Err("A draft image specified a size attribute without a format.")?,
            (None,Some(_)) => Err("A draft image specified a format attribute without a size.")?,
            (Some(size),Some(format)) => Some((size,format)),
            (None,None) => None
        };

        let alt = alt.to_owned();
    
        Ok(Self {
            filename,
            alt,
            size_format,
            watermark,
            full_link
        })

    }
}

struct FileChoice {
    name: String,
    entry: PathBuf
}

impl Display for FileChoice {

    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> { 
        write!(f,"{}",self.name)
    }

}


struct Environment {
    draft_folder: PathBuf,
    assets_folder: PathBuf,
    posts_folder: PathBuf,
    categories_folder: PathBuf,
    series_folder: PathBuf,
    categories_script: PathBuf,
    search_index_script: PathBuf,
    draft_image_pattern: Regex
    
}

impl Environment {

    fn new() -> Result<Self,Box<dyn Error>> {
        let cd = std::env::current_dir()?;
        Ok(Self {
            draft_folder: cd.join("_drafts"),
            assets_folder: cd.join("assets"),
            posts_folder: cd.join("_posts"),
            categories_folder: cd.join("categories"),
            series_folder: cd.join("series"),
            categories_script: cd.join("_tools").join("create_categories.js"),
            search_index_script: cd.join("_tools").join("create_search_index.js"),
            draft_image_pattern: Regex::new(r"\\drafting\\image *(?:\[([^]]*)\])? *(?:\{([^\}]*)\})?")?,
            // '\drafting\image[(<arg>=<value>),*]{<alt-text>}
            // 1st capture is the arguments, which are parsed with 'split', second is the alt text.
        })
        
    }

    fn replace_all_draft_image_tags<Callback: FnMut(&str, &str) -> Result<String,Box<dyn Error>>>(&self, source_str: &str, mut callback: Callback) -> Result<String,Box<dyn Error>> {
        let mut error = Default::default();
        let fixed = self.draft_image_pattern.replace_all(source_str,|captures: &Captures| {

            macro_rules! default {
                () => {
                    format!("{}",&captures[0])
                }
            }

            match (captures.get(1),captures.get(2)) {
                (Some(args),Some(alt)) => match callback(args.as_str(),alt.as_str()) {
                    Ok(fixed) => fixed,
                    Err(err) => {
                        error = Some(err);
                        default!()                        
                    }
                },
                (Some(_),None) => {
                    error = Some("A drafting.image tag was found without alt text.".into());
                    default!()
                },
                (None,_) => {
                    error = Some("A drafting.image tag was found without attributes.".into());
                    default!()
                }
            }
        });
        if let Some(error) = error {
            Err(error)
        } else {
            Ok(fixed.into_owned())
        }

    }


    fn list_draft_file_choices(&self) -> Result<Vec<FileChoice>,io::Error> {
        io::list_files(&self.draft_folder, |file| {
            let path = file.path();
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    if let Some(name) = path.file_name() {
                        return Some(FileChoice {
                            name: name.to_string_lossy().into_owned(),
                            entry: file.path()
                        })
    
                    }
                }
            }
            None
        })
    
    }


    fn list_categories(&self) -> Result<Vec<String>,io::Error> {
        io::list_files(&self.categories_folder, |file| {
            let path = file.path();
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    if let Some(name) = path.file_stem() {
                        if name != "Uncategorized" {
                            return Some(name.to_string_lossy().into_owned())
                        }
                    }
                }
            }
            None
    
        })
    }

    fn list_series(&self) -> Result<Vec<String>,io::Error> {
        io::list_files(&self.series_folder, |file| {
            let path = file.path();
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    if let Some(name) = path.file_stem() {
                        return Some(name.to_string_lossy().into_owned())
                    }
                }
            }
            None
        })
    }

    fn get_original_image_filename(&self, filename: &str) -> Result<PathBuf,io::Error> {
        let file_path = self.draft_folder.join(filename);
        if !file_path.try_exists()? {
            Err(io::Error::new(std::io::ErrorKind::NotFound, format!("The image {} was not found. Please edit your post and try again.",filename)))
        } else {
            Ok(file_path)
        }
    }

}



struct Post {
    frontmatter: Mapping,
    body: String
}

impl Post {

    fn load_from_file(path: &Path) -> Result<Rc<RefCell<Self>>,Box<dyn Error>> {

        let file_contents = read_to_string(path)?;
        
        let content_pattern = Regex::new(r"^(?:-{3}(?:\n|\r)([\w\W]+?)(?:\n|\r)-{3})?([\w\W]*)*")?;
        //  - first capture group captures the metadata
        //  - second capture group captures the text contents
        let (frontmatter, body) = if let Some(captures) = content_pattern.captures(&file_contents) {

            (if let Some(capture) = captures.get(1) {
                if let Some(yaml) = serde_yaml::from_str(capture.as_str())? {
                    yaml
                } else {
                    Mapping::new()
                }
            } else {
                Mapping::new()
            },
    
            if let Some(capture) = captures.get(2) {
                capture.as_str().to_owned()
            } else {
                String::new()
            })
    
        } else {
            (Mapping::new(),String::new())
        };

        Ok(Rc::from(RefCell::from(Self {
            frontmatter,
            body
        })))


    }

    fn save_to_file(&self, path: &Path) -> Result<(),TaskError> {


        let mut new_content = String::new();
        new_content.push_str("---\n");
        new_content.push_str(&serde_yaml::to_string(&self.frontmatter)?);
        new_content.push_str("---\n");
        new_content.push_str(&self.body);
        write(path,new_content)?;
        Ok(())

    } 



}

#[enum_dispatch]
trait TaskBehavior {

    fn description(&self) -> String;

    fn perform(self) -> Result<(),TaskError>;

}

struct CommandTask {
    command: (OsString,Vec<OsString>)
}

impl CommandTask {

    fn perform(self) -> Result<(),TaskError> {
        let mut child = Command::new(&self.command.0).args(&self.command.1).spawn()?;
        if child.wait()?.success() {
           Ok(()) 
        } else {
            Err(TaskError::CommandFailed(format!("{} {}",self.command.0.to_string_lossy(),self.command.1.iter().map(|a| a.to_string_lossy().into_owned()).collect::<Vec<String>>().join(" "))))
        }

    }
}

struct ImageTask {
    source_file: PathBuf,
    target_file: PathBuf,
    add_watermark: bool,
    resize: Option<(ImageSize,ImageFormat)>,
    command: CommandTask,
    
}

impl ImageTask {

    fn get_watermark(environment: &Environment) -> PathBuf {
        environment.draft_folder.join("watermark.png")
    }

    fn new(environment: &Environment, source_file: PathBuf, target_file: PathBuf, add_watermark: bool, resize: &Option<(ImageSize,ImageFormat)>) -> Self {
        let command = if let Some((size,format)) = &resize {
            if add_watermark {
                ("magick".into(),vec![
                    source_file.clone().into(),
                    "-set".into(),
                    "option:wmwidth".into(),
                    "%[fx:int(w)]".into(),
                    "-set".into(),
                    "option:wmheight".into(),
                    "%[fx:int(h)]".into(),
                    "(".into(),
                    Self::get_watermark(environment).into(),
                    "-resize".into(),
                    "%[wmwidth]x%[wmheight]!".into(),
                    ")".into(),
                    "-gravity".into(),
                    "Center".into(),
                    "-composite".into(),
                    "-resize".into(),
                    format!("{}x{}>",size.width(format),size.height(format)).into(),
                    target_file.clone().into()
                ])
    

            } else {
                //convert $f -resize 874x620\> ${f%.*}-A6-150.${f##*.}
                ("magick".into(),vec![
                    source_file.clone().into(),
                    "-resize".into(),
                    format!("{}x{}>",size.width(format),size.height(format)).into(),
                    target_file.clone().into(),
                ])
            }
        } else if add_watermark {
            ("magick".into(),vec![
                source_file.clone().into(),
                "-set".into(),
                "option:wmwidth".into(),
                "%[fx:int(w)]".into(),
                "-set".into(),
                "option:wmheight".into(),
                "%[fx:int(h)]".into(),
                "(".into(),
                Self::get_watermark(environment).into(),
                "-resize".into(),
                "%[wmwidth]x%[wmheight]!".into(),
                ")".into(),
                "-gravity".into(),
                "Center".into(),
                "-composite".into(),
                target_file.clone().into()
            ])
        } else {
            ("cp".into(),vec![
                source_file.clone().into(),
                target_file.clone().into(),
            ])
        };
        Self {
            source_file,
            target_file,
            add_watermark,
            resize: resize.clone(),
            command: CommandTask {
                command
            }
        }
    }
}

impl TaskBehavior for ImageTask {

    fn description(&self) -> String {
        if let Some((size,format)) = &self.resize {
            if self.add_watermark {
                format!("Resize {} at {} {}, watermark and copy to {}",self.source_file.to_string_lossy(),format,size,self.target_file.to_string_lossy())
            } else {
                format!("Resize {} at {} {} and copy to {}",self.source_file.to_string_lossy(),format,size,self.target_file.to_string_lossy())
            }
        } else if self.add_watermark {
            format!("Watermark and copy {} to {}",self.source_file.to_string_lossy(),self.target_file.to_string_lossy())
        } else {
            format!("Copy {} to {}",self.source_file.to_string_lossy(),self.target_file.to_string_lossy())
        }
    }

    fn perform(self) -> Result<(),TaskError> {
        self.command.perform()
    }

}

struct PropertyTask {
    name: Value,
    value: Value,
    post: Rc<RefCell<Post>>
}

impl PropertyTask {

    fn new(post: Rc<RefCell<Post>>, name: Value, value: Value) -> Self {
        Self {
            name,
            value,
            post
        }
    }
}

impl TaskBehavior for PropertyTask {

    fn description(&self) -> String {
        let name = self.name.as_str().unwrap();
        format!("Set property {} to {:?}",name,self.value)
    }

    fn perform(self) -> Result<(),TaskError> {
        self.post.borrow_mut().frontmatter.insert(self.name,self.value);
        Ok(())
    }


}

struct EnsureDirectoryTask {
    path: PathBuf
}

impl EnsureDirectoryTask {

    fn new(path: PathBuf) -> Self {
        Self {
            path
        }
    }
}

impl TaskBehavior for EnsureDirectoryTask {

    fn description(&self) -> String {
        format!("Ensure directory {} exists",self.path.to_string_lossy())
    }

    fn perform(self) -> Result<(),TaskError> {
        Ok(create_dir_all(self.path)?)
    }

}

struct RenderSiteTask {
    rerender: bool,
    command: CommandTask
}

impl RenderSiteTask {

    fn new(rerender: bool) -> Self {
        Self {
            rerender,
            command: CommandTask {
                command: ("bundle".into(),
                          vec!["exec".into(), 
                               "jekyll".into(),
                               "build".into()])
            }
        }
    }
}

impl TaskBehavior for RenderSiteTask {

    fn description(&self) -> String {
        if self.rerender {
            format!("Re-render site to pick up new categories and search index.")
        } else {
            format!("Render the site to pick up the new post for indexing.")
        }
    }

    fn perform(self) -> Result<(),TaskError> {
        // I could check to see if it's already running, but I don't think that there will be any problems
        // if I just build.
        self.command.perform()
    }

}

struct UpdateSearchIndexTask {
    command: CommandTask
}

impl UpdateSearchIndexTask {

    fn new(environment: &Environment) -> Self {
        Self {
            command: CommandTask {
                command: (environment.search_index_script.clone().into(),vec![])
            }
        }
    }
}

impl TaskBehavior for UpdateSearchIndexTask {

    fn description(&self) -> String {
        format!("Update search index.")
    }

    fn perform(self) -> Result<(),TaskError> {
        self.command.perform()
    }

}


struct UpdateCategoriesTask {
    command: CommandTask
}

impl UpdateCategoriesTask {

    fn new(environment: &Environment) -> Self {
        Self {
            command: CommandTask {
                command: (environment.categories_script.clone().into(),vec![])
            }
        }
    }
}

impl TaskBehavior for UpdateCategoriesTask {

    fn description(&self) -> String {
        format!("Update categories and series files.")
    }

    fn perform(self) -> Result<(),TaskError> {
        self.command.perform()
    }

}


struct UpdateBodyTask {
    new_content: String,
    post: Rc<RefCell<Post>>
}

impl UpdateBodyTask {

    fn new(post: Rc<RefCell<Post>>, new_content: String) -> Self {
        Self {
            new_content,
            post
        }
    }
}

impl TaskBehavior for UpdateBodyTask {

    fn description(&self) -> String {
        format!("Update image links in content.")
    }

    fn perform(self) -> Result<(),TaskError> {
        self.post.borrow_mut().body = self.new_content.clone();
        Ok(())
    }

}

struct SavePostTask {
    target_path: PathBuf,
    post: Rc<RefCell<Post>>
}

impl SavePostTask {

    fn new(post: Rc<RefCell<Post>>, target_path: PathBuf) -> Self {
        Self {
            target_path,
            post
        }
    }
}

impl TaskBehavior for SavePostTask {

    fn description(&self) -> String {
        format!("Save post to {}.",self.target_path.to_string_lossy())
    }

    fn perform(self) -> Result<(),TaskError> {
        self.post.borrow_mut().save_to_file(&self.target_path)
    }

}

struct MarkPublishedTask {
    source_path: PathBuf,
    date_slug: String
}

impl MarkPublishedTask {

    fn new(source_path: PathBuf, date_slug: String) -> Self {
        Self {
            source_path,
            date_slug
        }
    }
}

impl TaskBehavior for MarkPublishedTask {

    fn description(&self) -> String {
        format!("Add '.{}.published' extension to '{}'.",self.date_slug,self.source_path.to_string_lossy())
    }

    fn perform(self) -> Result<(),TaskError> {
        let mut extension = if let Some(old_ext) = self.source_path.extension() {
            vec![old_ext.to_owned()]
        } else {
            vec![]
        };
        extension.push(OsString::from(self.date_slug));
        extension.push(OsString::from("published"));
        let extension = extension.join(&OsString::from("."));
        let target_path = self.source_path.with_extension(extension);
        rename(self.source_path,target_path)?;
        Ok(())
    }

}

#[derive(PartialEq)]
#[enum_dispatch(TaskBehavior)]
enum Task {
    ImageTask,
    PropertyTask,
    EnsureDirectoryTask,
    RenderSiteTask,
    UpdateSearchIndexTask,
    UpdateCategoriesTask,
    UpdateBodyTask,
    SavePostTask,
    MarkPublishedTask
}

#[derive(Debug)]
// I want to limit the kinds of errors that can happen in tasks. As many errors should be triggered before the task is actually run as possible. The series of tasks are a major change that can't easily be rolled back (although maybe someday).
enum TaskError {
    Io(io::Error),
    Yaml(serde_yaml::Error), // Only because I need it to serialize the edited yaml code for the post. I believe that the only error that can occur is an io error, but it uses the same errors for serializing as for deserializing.
    CommandFailed(String)
}

impl Error for TaskError {}

impl From<io::Error> for TaskError {

    fn from(err: io::Error) -> Self { 
        Self::Io(err)
    }


}

impl From<serde_yaml::Error> for TaskError {

    fn from(err: serde_yaml::Error) -> Self { 
        Self::Yaml(err)
    }

}

impl Display for TaskError {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Io(err) => write!(f,"{}",err),
            Self::Yaml(err) => write!(f,"{}",err),
            Self::CommandFailed(command) => write!(f,"Child command failed: {}",command)
        }
    }

}

struct TaskList {
    tasks: Vec<Task>
}

impl TaskList {

    fn new() -> Self {
        Self {
            tasks: vec![]
        }
    }

    fn add(&mut self, task: Task)  -> Result<(),Box<dyn Error>> {
        io::write_info(&format!("Adding task: {}",task.description()))?;

        self.tasks.push(task);

        Ok(())

    }

    // NOTE: To make things easier for closing variables, tasks can only be run once, which means they get eaten up when they are run, which means you need to own the task list to call this.
    fn perform(self) -> Result<(),TaskError> {
        for task in self.tasks.into_iter() {
            io::write_info(&format!("Completing task: {}",task.description()))?;
            task.perform()?;
        }
        Ok(())
    }

    fn add_set_property_task(&mut self, post: &Rc<RefCell<Post>>, property: &str, yaml_value: Value) -> Result<(),Box<dyn Error>> {
        let yaml_property = serde_yaml::to_value(property)?;
        self.add(PropertyTask::new(post.clone(),yaml_property,yaml_value).into())?;
        Ok(())
    }

    fn add_image_task(&mut self, environment: &Environment, source_file: PathBuf, target_file: PathBuf, add_watermark: bool, resize: &Option<(ImageSize,ImageFormat)>) -> Result<(),Box<dyn Error>> {

        let new_task = ImageTask::new(environment, source_file, target_file, add_watermark, resize);

        let mut found_conflicting = false;
        let mut found_duplicate = false;
        for task in &self.tasks {

            if let Task::ImageTask(task) = task {
                if task.source_file == new_task.source_file {
                    if (task.add_watermark != new_task.add_watermark) ||
                       (task.resize != new_task.resize) {
                        found_conflicting = true;
                        break;
                    } else {
                        found_duplicate = true;
                    }
                }

            }

        };

        if found_conflicting {
            if !io::read_yes_no(&format!("image {} is included more than once with conflicting arguments, only the attributes on the first tag will be used for processing. Are you okay with this?",new_task.source_file.to_string_lossy()))? {
                Err("Please edit the post and try again.")?
            }
        } else if !found_duplicate {
            self.add(new_task.into())?;
        }



        Ok(())

    }

    fn add_assets_directory_task_if_not_present(&mut self, assets_dir: &Path, date_slug: &str) -> Result<(),Box<dyn Error>> {
        let directory_path = assets_dir.join(date_slug);
        let new_task = EnsureDirectoryTask::new(directory_path).into();
        for task in &self.tasks {
            if task == &new_task {
                return Ok(())
            }
        }
        self.add(new_task)?;
        Ok(())
    }

    fn add_mark_published_task(&mut self, draft_file: PathBuf, date_slug: &str) -> Result<(),Box<dyn Error>> {
        let mut files_to_mark = vec![];
        files_to_mark.push(draft_file);
        for task in &self.tasks {
            match task {
                Task::PropertyTask(_) |
                Task::EnsureDirectoryTask(_) |
                Task::RenderSiteTask(_) |
                Task::UpdateSearchIndexTask(_) |
                Task::UpdateCategoriesTask(_) |
                Task::UpdateBodyTask(_) |
                Task::SavePostTask(_) |
                Task::MarkPublishedTask(_) => (), // these guys don't have files...
                Task::ImageTask(task) => {
                    let file = &task.source_file;
                    if !files_to_mark.contains(file) {
                        files_to_mark.push(file.to_path_buf())
                    }
                }
            }
        }
        for file in files_to_mark {
            self.add(MarkPublishedTask::new(file,date_slug.to_owned()).into())?

        }
        Ok(())

    }


}

enum Recipe {
    Publish(FileChoice),
    // There's no option for republishing a draft. Once published, the post can be edited directly to fix errors.
    // I'm only providing a republish image because that's the only one that's difficult to fix.
    RepublishImage,
    UpdateSite,
}

impl Display for Recipe {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            Self::Publish(choice) => write!(f,"Publish '{}'.",choice),
            Self::RepublishImage => write!(f,"Republish an image."),
            Self::UpdateSite => write!(f,"Run indexing scripts only.") 
        }
    }
}

fn choose_recipe(environment: &Environment) -> Result<Recipe,Box<dyn Error>> {
    let draft_file_list = environment.list_draft_file_choices()?;
    let mut recipes = if draft_file_list.len() == 0 {
        io::write_info("There are no drafts to publish.")?;
        vec![]
    } else {
        draft_file_list.into_iter().map(|file| Recipe::Publish(file)).collect()
    };

    recipes.push(Recipe::RepublishImage);
    recipes.push(Recipe::UpdateSite);
    Ok(io::read_choice("What would you like to do?", recipes.into_iter())?)
}

fn publish(environment: &Environment, tasks: &mut TaskList, draft_file: FileChoice) -> Result<(),Box<dyn Error>> {


    io::write_info(&format!("Preparing \"{}\" for publishing.",draft_file))?;

    let post = Post::load_from_file(&draft_file.entry)?;

    /* properties layout, type -- in general this should be post, but if the user already put something else there, confirm */
    // If the user set these, we don't need to second-guess them.
    set_property_if_not_set(&post,"layout","post",tasks)?;
    set_property_if_not_set(&post,"type","post",tasks)?;

    let default_title = draft_file.entry.file_stem().unwrap().to_string_lossy().into_owned();
    // If the user set the title, I probably don't need to second-guess them. If they didn't, then they probably did want the
    // filename to be the title and we still don't need to second-guess them.
    // FUTURE: Alternatively, allow the first '#' to be the title?
    let draft_file_title = set_property_if_not_set(&post,"title",&default_title, tasks)?;
    let title_slug = slugify(&draft_file_title,"","-",None);

    // If the user set the date, then we probably don't need to second-guess them. If they didn't, then they're going to want
    // it automatically set.
    let draft_file_date = set_property_if_not_set(&post,"date", &format_yaml_date(Local::now()), tasks)?;
    let date_slug = draft_file_date.chars().take(10).collect::<String>();

    // If the user set the categeories, don't worry about what they set it to. I do want to verify the category is an existing category though.
    // And, I do want to confirm that they didn't want to add categories.
    confirm_categories(&post,&environment.list_categories()?)?;
    // If the user didn't set the series, that's okay. If they set it that's okay. The only thing to catch is whether the series
    // is an existing or new one.
    confirm_series(&post,&environment.list_series()?)?;

    confirm_excerpt(&post)?;

    tasks.add_set_property_task(&post, "published", serde_yaml::to_value(true)?)?;

    fix_images(&post,&date_slug,&environment,tasks)?;

    let new_path = environment.posts_folder.join(format!("{}-{}.md",&date_slug,title_slug));

    tasks.add(SavePostTask::new(post,new_path).into())?;

    // very last thing before rendering the site, mark them as published so they don't get picked up on the publish script next time.
    tasks.add_mark_published_task(draft_file.entry,&date_slug)?;    

    Ok(())
}

fn update_site(environment: &Environment, tasks: &mut TaskList) -> Result<(),Box<dyn Error>> {

    // This stuff can be done after the publishing, in case there was an error and we need to redo.
    tasks.add(RenderSiteTask::new(false).into())?;

    // FUTURE: Should only do this if there are new categories...
    tasks.add(UpdateCategoriesTask::new(&environment).into())?;

    tasks.add(UpdateSearchIndexTask::new(&environment).into())?;

    tasks.add(RenderSiteTask::new(true).into())?;

    Ok(())

}

fn run() -> Result<(),Box<dyn Error>> {

    let environment = Environment::new()?;
    let mut tasks = TaskList::new();

    io::write("Type ctrl-C to cancel.")?;
    io::write("TIP: You can use '\\drafting.image[attrs]{alt-text}' in your post to automate image processing")?;

    match choose_recipe(&environment)? {
        Recipe::Publish(draft_file) => {
            publish(&environment,&mut tasks,draft_file)?;
            update_site(&environment,&mut tasks)?;
        },
        Recipe::RepublishImage => {
            todo!("Republish an image")
            // Need to do the following:
            // 1. User chooses an image in _drafts which are marked as published (<name>.png.<date>.published)
            //    - this should also give us the date-slug to use to find the correct target in assets.
            //    - might support other image types, but I don't foresee ever using jpg or gif.
            // 2. Prompt user for new size/format, custom, or no resize
            // 3. Propmt user for whether they want to add a watermark or not
            // 4. Add image task with appropriate values.
        },
        Recipe::UpdateSite => {
            update_site(&environment,&mut tasks)?;
        }
    
    }

    if io::read_yes_no("Tasks prepared. Should I go ahead?")? {
        tasks.perform()?;
    } else {
        io::write_warning("I didn't do anything.")?;
    }
    Ok(())

}

fn main() -> ExitCode {

    if let Err(err) = run() {
        io::write_error(&format!("{}",err)).expect(&format!("{}",err));
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }

}
