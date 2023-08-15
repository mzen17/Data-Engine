#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Line<T> {
    startx: T,
    starty: T,
    endx: T,
    endy: T,
}


#[derive(Copy, Clone, Debug, PartialEq)]
struct Rectangle {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    area: f32,
}


// Make sure starts are always smaller than ends to normalize the lines. Note: Will only work if the lines are horizontal or vertical due to the automatic swap.
fn correct_direction(line: Line<f32>) -> Line<f32> {
    let mut startx = line.startx;
    let mut starty = line.starty;
    let mut endx = line.endx;
    let mut endy = line.endy;

    if line.startx > line.endx {
        startx = line.endx;
        endx = line.startx;
    } else if line.starty > line.endy {
        starty = line.endy;
        endy = line.starty;
    }
    Line {
        startx,
        starty,
        endx,
        endy,
    }
}


// 2 * O(Amount of Lines) time complexity. Flatten lines so all values are between 0 and 1, while correcting their direction.
fn flatten_to_1s(vectors: Vec<Line<f32>>) -> Vec<Line<f32>> {
    // Find the overall bounding box
    let mut x = [f32::MIN, f32::MAX];
    let mut y = [f32::MIN, f32::MAX];

    let mut init = false;
    for line in &vectors {
        if !init {
            init = true;
            x = [f32::max(line.startx, line.endx), f32::min(line.startx, line.endx)];
            y = [f32::max(line.starty, line.endy), f32::min(line.starty, line.endy)];
        } else {
            x[0] = f32::max(x[0], f32::max(line.startx, line.endx));
            x[1] = f32::min(x[1], f32::min(line.startx, line.endx));
            y[0] = f32::max(y[0], f32::max(line.starty, line.endy));
            y[1] = f32::min(y[1], f32::min(line.starty, line.endy));
        }
    }

    // Convert lines to normalized coordinates
    let mut flatten = Vec::new();
    for line in &vectors {
        let startx = (line.startx - x[1]) as f32 / (x[0] - x[1]) as f32;
        let endx = (line.endx - x[1]) as f32 / (x[0] - x[1]) as f32;
        let starty = (line.starty - y[1]) as f32 / (y[0] - y[1]) as f32;
        let endy = (line.endy - y[1]) as f32 / (y[0] - y[1]) as f32;
        flatten.push(correct_direction(Line {
            startx,
            endx,
            starty,
            endy,
        }));
    }
    flatten
}


// Pulls out a part of an image based on a rectangle
use image::{DynamicImage, GenericImageView, ImageBuffer};
fn get_rectangle_chunks_from_image(img: DynamicImage, rectangles: Vec<Rectangle>) -> Vec<DynamicImage> {
    let mut sub_images = Vec::new();
    
    for rectangle in rectangles {
        let (width, height) = img.dimensions();
        let x1 = (rectangle.x1 * width as f32) as u32;
        let y1 = (rectangle.y1 * height as f32) as u32;
        let x2 = (rectangle.x2 * width as f32) as u32;
        let y2 = (rectangle.y2 * height as f32) as u32;
        
        if x1 < x2 && y1 < y2 {
            let sub_width = x2 - x1;
            let sub_height = y2 - y1;
            
            let mut sub_image = ImageBuffer::new(sub_width, sub_height);
            
            for y in 0..sub_height {
                for x in 0..sub_width {
                    let pixel = img.get_pixel(x1 + x, y1 + y);
                    sub_image.put_pixel(x, y, pixel);
                }
            }
            
            sub_images.push(DynamicImage::ImageRgba8(sub_image));
        }               // },60000);

    }
    
    sub_images
}



// Create a vector of non-overlapping rectangles from a vector of lines 
fn create_grid_from_lines(lines: Vec<Line<f32>>) -> Vec<Rectangle> {
    // Phase: Get all Recs, sort by smallest area, cut them out 1 at a time in order of smallest -> largest
    let mut all_rectangles: Vec<Rectangle> = Vec::new();               // },60000);


    let mut horizontal_lines: Vec<Line<f32>> = Vec::new();
    let mut vertical_lines: Vec<Line<f32>> = Vec::new();

    // Split lines into horizontal and vertical O(N)
    for line in lines {
        if line.startx == line.endx {
            vertical_lines.push(line);
        } else if line.starty == line.endy {
            horizontal_lines.push(line);
        }
    }

    // Permutate through every combo of horizontal and vertical lines O(N^4)
    let n = horizontal_lines.len();
    let n2 = vertical_lines.len();

    for i in 0..n {
        for j in (i+1)..n {
            for k in 0..n2 {
                for l in (k+1)..n2 {
                    let hline1 = horizontal_lines[i];
                    let hline2 = horizontal_lines[j];

                    let vline1 = vertical_lines[k];
                    let vline2 = vertical_lines[l];

                    let hlines = vec![hline1, hline2];
                    let vlines = vec![vline1, vline2];
                    let rect = create_rectangles_from_lines(hlines, vlines);

                    if rect != None {
                        all_rectangles.push(rect.unwrap().clone())
                    }

                }
            }

        }
    }

    // Sort Rectangle List by area
    all_rectangles.sort_by(|a, b| a.area.partial_cmp(&b.area).unwrap());

    let mut cutted: Vec<Rectangle> = Vec::new();
    let mut rectangles: Vec<Rectangle> = Vec::new();
    // Iterate through the rectanges
    for rectangle in all_rectangles {
        // Check if the rectangle overlaps with any other rectangle
        let mut overlap = false;
        for cut in &cutted {
            if is_overlap(&rectangle, &cut) {
                overlap = true;
                break;
            }
        }

        // If it doesn't overlap, add it to the cutted list
        if !overlap {
            cutted.push(rectangle.clone());
            rectangles.push(rectangle);
        }
    }

    return rectangles;
}


// Determine if two rectangles overlap. Runs in O(1) time.
fn is_overlap(rect1: &Rectangle, rect2: &Rectangle) -> bool {
    if rect1.x1 >= rect2.x2 || rect2.x1 >= rect1.x2 {
        return false;
    }

    if rect1.y1 >= rect2.y2 || rect2.y1 >= rect1.y2 {
        return false;
    }

    true
}


// Generates a rectangle from 4 lines, if possible. Runs in O(1) time
fn create_rectangles_from_lines(horizontal_line: Vec<Line<f32>>, vertical_line: Vec<Line<f32>>) -> Option<Rectangle> {
    if vertical_line.len() == 2 && horizontal_line.len() == 2 {
        // Normalize each pair of lines to the same value
        let min_bot = f32::max(vertical_line[0].starty, vertical_line[1].starty);
        let max_top = f32::min(vertical_line[0].endy, vertical_line[1].endy);

        let min_left = f32::max(horizontal_line[0].startx, horizontal_line[1].startx);
        let max_right = f32::min(horizontal_line[0].endx, horizontal_line[1].endx);

        // Check if the vertical line x-values are between minLeft and maxLeft
        if vertical_line[0].startx >= min_left && vertical_line[1].startx <= max_right {

            // Check if the horizontal line y-values are between minTop and maxTop
            if horizontal_line[0].starty >= min_bot && horizontal_line[1].starty <= max_top {
                let x1:f32;
                let x2:f32;
                let y1:f32;
                let y2:f32;
                if vertical_line[0].startx < vertical_line[1].startx {
                    x1 = f32::max(min_left, vertical_line[0].startx);
                    x2 = f32::min(max_right, vertical_line[1].endx);
                }else {
                    x1 = f32::max(min_left, vertical_line[1].startx);
                    x2 = f32::min(max_right, vertical_line[0].endx);
                }
                if horizontal_line[0].starty < horizontal_line[1].starty {
                    y1 = f32::max(min_bot, horizontal_line[0].starty);
                    y2 = f32::min(max_top, horizontal_line[1].endy);
                }else {
                    y1 = f32::max(min_bot, horizontal_line[1].starty);
                    y2 = f32::min(max_top, horizontal_line[0].endy);
                }
                let area = (x2 - x1) * (y2 - y1);
                if area > 0.0 {
                    return Some(Rectangle {
                        x1: x1,
                        x2: x2,
                        y1: y1,
                        y2: y2,
                        area: (x2 - x1) * (y2 - y1),
                    });
                }

            }
        }
    }
    None
}


// Public exposed function
pub fn split_image(img: DynamicImage, lines: Vec<Vec<f32>>) -> Vec<DynamicImage> {
    // Convert the lines into the Struct Lines
    let mut lines_struct: Vec<Line<f32>> = Vec::new();
    for line in lines {
        lines_struct.push(Line {
            startx: line[0],
            starty: line[1],
            endx: line[2],
            endy: line[3],
        });
    }

    // Create the grid from the lines
    let flines = flatten_to_1s(lines_struct);
    let grid: Vec<Rectangle> = create_grid_from_lines(flines);

    let new_img: Vec<DynamicImage> = get_rectangle_chunks_from_image(img, grid);
    new_img
}
