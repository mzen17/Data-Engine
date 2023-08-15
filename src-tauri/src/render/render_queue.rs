use image::DynamicImage;


#[derive(Clone, Debug, PartialEq)]
pub struct RenderLayer {
    img_path: String,
    layer_form: String,
    data: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderQueue {
    current: u32,
    queue: Vec<RenderLayer>,
}

impl RenderQueue {
    pub fn new() -> RenderQueue {
        RenderQueue {
            current: 0,
            queue: Vec::new(),
        }
    }

    pub fn add(&mut self, path: RenderLayer) {
        self.queue.push(path);
    }

    pub fn next(&mut self) -> Option<RenderLayer> {
        if self.current < self.queue.len() as u32 {
            let path = self.queue[self.current as usize].clone();
            self.current += 1;
            Some(path)
        } else {
            None
        }
    }

    pub fn render(&self) -> Option<DynamicImage> {
        if self.current < self.queue.len() as u32 {
            let renderlayer = &self.queue[self.current as usize];
            let img = image::open(&renderlayer.img_path).unwrap();
            
            // Only apply the layer_form if it's not empty
            if !renderlayer.layer_form.is_empty() {
                let layer = &renderlayer.layer_form;
                if (layer == "Split") {

                }else if (layer == "Gray") {
                    
                }
                Some(img)
            } else {
                Some(img)
            }
        } else {
            None
        }
    }

    pub fn completed(&self) -> bool {
        self.current >= self.queue.len() as u32
    }
}


