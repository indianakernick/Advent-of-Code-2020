use adventofcode2020::*;

const IMAGE_SIZE: usize = 10;

type ImageIdx = u8;
type ImageID = u16;
type EdgeIdx = u8;
type ImageData = Vec<bool>;
type ImageBorder = Vec<bool>;

#[derive(Copy, Clone)]
struct EdgeRef {
    image: ImageIdx,
    edge: EdgeIdx,
}

struct Image {
    id: ImageID,
    data: ImageData,
    edges: [EdgeRef; 4],
    position: (i32, i32),
}

fn parse_input() -> Vec<Image> {
    let mut images = Vec::new();
    let mut line_iter = line_iter_from_file("input/test.txt");
    while let Some(title_line) = line_iter.next() {
        let id = title_line[5..9].parse::<ImageID>().unwrap();
        let mut data = ImageData::new();
        for _ in 0..IMAGE_SIZE {
            let mut line = line_iter.next().unwrap().chars().map(|ch| ch == '#').collect();
            data.append(&mut line);
        }
        line_iter.next(); // blank line
        if data.len() != IMAGE_SIZE * IMAGE_SIZE {
            panic!();
        }
        images.push(Image {
            id,
            data,
            edges: [EdgeRef { image: ImageIdx::MAX, edge: 0 }; 4],
            position: (i32::MAX, i32::MAX),
        });
    }
    images
}

fn top_edge(image: &ImageData) -> ImageBorder {
    image[0..IMAGE_SIZE].to_vec()
}

fn bottom_edge(image: &ImageData) -> ImageBorder {
    image[((IMAGE_SIZE - 1) * IMAGE_SIZE)..(IMAGE_SIZE * IMAGE_SIZE)].to_vec()
}

fn left_edge(image: &ImageData) -> ImageBorder {
    image.iter().step_by(IMAGE_SIZE).map(|p|*p).collect()
}

fn right_edge(image: &ImageData) -> ImageBorder {
    image.iter().skip(IMAGE_SIZE - 1).step_by(IMAGE_SIZE).map(|p|*p).collect()
}

fn edge(image: &ImageData, idx: EdgeIdx) -> ImageBorder {
    match idx {
        0 => top_edge(image),
        1 => right_edge(image),
        2 => bottom_edge(image),
        3 => left_edge(image),
        _ => panic!()
    }
}

fn reverse(border: &ImageBorder) -> ImageBorder {
    border.iter().rev().map(|p|*p).collect()
}

fn set_edges(images: &mut Vec<Image>) {
    for left_image_idx in 0..(images.len() - 1) {
        for left_edge_idx in 0..4 {
            let left_edge = edge(&images[left_image_idx].data, left_edge_idx);
            for right_image_idx in (left_image_idx + 1)..images.len() {
                for right_edge_idx in 0..4 {
                    let right_edge = edge(&images[right_image_idx].data, right_edge_idx);
                    if left_edge == right_edge {
                        images[left_image_idx].edges[left_edge_idx as usize].image = right_image_idx as ImageIdx;
                        images[left_image_idx].edges[left_edge_idx as usize].edge = right_edge_idx;
                        images[right_image_idx].edges[right_edge_idx as usize].image = left_image_idx as ImageIdx;
                        images[right_image_idx].edges[right_edge_idx as usize].edge = left_edge_idx;
                    } else if left_edge == reverse(&right_edge) {
                        images[left_image_idx].edges[left_edge_idx as usize].image = right_image_idx as ImageIdx;
                        images[left_image_idx].edges[left_edge_idx as usize].edge = 4 + right_edge_idx;
                        images[right_image_idx].edges[right_edge_idx as usize].image = left_image_idx as ImageIdx;
                        images[right_image_idx].edges[right_edge_idx as usize].edge = 4 + left_edge_idx;
                    }
                }
            }
        }
    }
}

fn corner_product(images: &Vec<Image>) -> u64 {
    let mut product = 1;
    for image in images.iter() {
        if image.edges.iter().filter(|edge| edge.image != ImageIdx::MAX).count() == 2 {
            product *= image.id as u64;
        }
    }
    product
}

fn flip_vert(data: &ImageData) -> ImageData {
    let mut result = ImageData::new();
    for y in (0..IMAGE_SIZE).rev() {
        for x in 0..IMAGE_SIZE {
            result.push(data[y * IMAGE_SIZE + x]);
        }
    }
    result
}

fn flip_hori(data: &ImageData) -> ImageData {
    let mut result = ImageData::new();
    for y in 0..IMAGE_SIZE {
        for x in (0..IMAGE_SIZE).rev() {
            result.push(data[y * IMAGE_SIZE + x]);
        }
    }
    result
}

fn transpose(data: &ImageData) -> ImageData {
    let mut result = ImageData::new();
    for x in 0..IMAGE_SIZE {
        for y in 0..IMAGE_SIZE {
            result.push(data[y * IMAGE_SIZE + x]);
        }
    }
    result
}

fn rotate_cw(data: &ImageData) -> ImageData {
    flip_hori(&transpose(data))
}

fn rotate_ccw(data: &ImageData) -> ImageData {
    flip_vert(&transpose(data))
}

fn flip_idx(edge_idx: &mut EdgeIdx) {
    if *edge_idx >= 4 {
        *edge_idx = (*edge_idx + 2) % 4 + 4;
    } else {
        *edge_idx = (*edge_idx + 2) % 4;
    }
}

fn rotate_idx_ccw(edge_idx: &mut EdgeIdx) {
    if *edge_idx >= 4 {
        *edge_idx = (*edge_idx + 3) % 4 + 4
    } else {
        *edge_idx = (*edge_idx + 3) % 4;
    }
}

fn rotate_idx_cw(edge_idx: &mut EdgeIdx) {
    if *edge_idx >= 4 {
        *edge_idx = (*edge_idx + 1) % 4 + 4
    } else {
        *edge_idx = (*edge_idx + 1) % 4;
    }
}

fn reverse_edge(edge_idx: &mut EdgeIdx) {
    *edge_idx = (*edge_idx + 4) % 8;
}

fn direction(edge_idx: EdgeIdx) -> (i32, i32) {
    match edge_idx {
        0 => (0, -1),
        1 => (1, 1),
        2 => (0, 1),
        3 => (-1, 0),
        _ => panic!(),
    }
}

fn align_images(images: &mut Vec<Image>, processing: &mut Vec<bool>, origin_image_idx: usize, origin_pos: (i32, i32)) {
    processing[origin_image_idx] = true;
    images[origin_image_idx].position = origin_pos;

    // the borrow checker gets in your way more often than it helps you
    for origin_edge_idx in 0..4 {
        if images[origin_image_idx].edges[origin_edge_idx].image == ImageIdx::MAX {
            continue;
        }

        let neighbor_image_idx = images[origin_image_idx].edges[origin_edge_idx].image as usize;
        if processing[neighbor_image_idx] {
            continue;
        }

        // println!("Image {}:{} references {}:{}", origin_image_idx, origin_edge_idx, images[origin_image_idx].edges[origin_edge_idx].image, images[origin_image_idx].edges[origin_edge_idx].edge);

        // Check if top matches right or right matches bottom or bottom matches left or left matches top
        let neighbor_edge_idx = images[origin_image_idx].edges[origin_edge_idx].edge as usize;
        if (origin_edge_idx + 1) % 4 == neighbor_edge_idx % 4 {
            rotate_idx_cw(&mut images[origin_image_idx].edges[origin_edge_idx].edge);
            images[neighbor_image_idx].data = rotate_cw(&images[neighbor_image_idx].data);
            images[neighbor_image_idx].edges.rotate_right(1);
        }

        // Check if top matches left or left matches bottom or bottom matches right or right matches top
        let neighbor_edge_idx = images[origin_image_idx].edges[origin_edge_idx].edge as usize;
        if (origin_edge_idx + 3) % 4 == neighbor_edge_idx % 4 {
            rotate_idx_ccw(&mut images[origin_image_idx].edges[origin_edge_idx].edge);
            images[neighbor_image_idx].data = rotate_ccw(&images[neighbor_image_idx].data);
            images[neighbor_image_idx].edges.rotate_left(1);
        }

        // Check if top edge matches top edge or bottom edge matches bottom edge
        let neighbor_edge_idx = images[origin_image_idx].edges[origin_edge_idx].edge as usize;
        if (origin_edge_idx & 1) == 0 && (neighbor_edge_idx & 1) == 0 && (origin_edge_idx & 2) == (neighbor_edge_idx & 2) {
            flip_idx(&mut images[origin_image_idx].edges[origin_edge_idx].edge);
            images[neighbor_image_idx].data = flip_vert(&images[neighbor_image_idx].data);
            images[neighbor_image_idx].edges.swap(0, 2);
            if images[neighbor_image_idx].edges[1].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[1].edge);
                // Reversing the edge of the tile that the neighbor edge refers to
                /*let neighbor_neighbor_image_idx = images[neighbor_image_idx].edges[1].image as usize;
                for neighbor_neighbor_edge_idx in 0..4 {
                    if images[neighbor_neighbor_image_idx].edges[neighbor_neighbor_edge_idx].image as usize == neighbor_image_idx {
                        reverse_edge(&mut images[neighbor_neighbor_image_idx].edges[neighbor_neighbor_edge_idx].edge);
                        break;
                    }
                }*/
            }
            if images[neighbor_image_idx].edges[3].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[3].edge);
                // Reversing the edge of the tile that the neighbor edge refers to
                /*let neighbor_neighbor_image_idx = images[neighbor_image_idx].edges[3].image as usize;
                for neighbor_neighbor_edge_idx in 0..4 {
                    if images[neighbor_neighbor_image_idx].edges[neighbor_neighbor_edge_idx].image as usize == neighbor_image_idx {
                        reverse_edge(&mut images[neighbor_neighbor_image_idx].edges[neighbor_neighbor_edge_idx].edge);
                        break;
                    }
                }*/
            }
        }

        // Check if left edge matches left edge or right edge matches right edge
        let neighbor_edge_idx = images[origin_image_idx].edges[origin_edge_idx].edge as usize;
        if (origin_edge_idx & 1) == 1 && (neighbor_edge_idx & 1) == 1 && (origin_edge_idx & 2) == (neighbor_edge_idx & 2) {
            flip_idx(&mut images[origin_image_idx].edges[origin_edge_idx].edge);
            images[neighbor_image_idx].data = flip_hori(&images[neighbor_image_idx].data);
            images[neighbor_image_idx].edges.swap(1, 3);
            if images[neighbor_image_idx].edges[0].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[0].edge);
                // Reversing the edge of the tile that the neighbor edge refers to
                /*let neighbor_neighbor_image_idx = images[neighbor_image_idx].edges[0].image as usize;
                for neighbor_neighbor_edge_idx in 0..4 {
                    if images[neighbor_neighbor_image_idx].edges[neighbor_neighbor_edge_idx].image as usize == neighbor_image_idx {
                        reverse_edge(&mut images[neighbor_neighbor_image_idx].edges[neighbor_neighbor_edge_idx].edge);
                        break;
                    }
                }*/
            }
            if images[neighbor_image_idx].edges[2].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[2].edge);
                // Reversing the edge of the tile that the neighbor edge refers to
                // Might not need to do this.
                // Since the neighbor will be the next image being processed,
                // it will traverse to the edge to the neighbor-neighbor.
                /*let neighbor_neighbor_image_idx = images[neighbor_image_idx].edges[2].image as usize;
                for neighbor_neighbor_edge_idx in 0..4 {
                    if images[neighbor_neighbor_image_idx].edges[neighbor_neighbor_edge_idx].image as usize == neighbor_image_idx {
                        reverse_edge(&mut images[neighbor_neighbor_image_idx].edges[neighbor_neighbor_edge_idx].edge);
                        break;
                    }
                }*/
            }
        }

        // Check if the top edge is reversed or the bottom edge is reversed
        let neighbor_edge_idx = images[origin_image_idx].edges[origin_edge_idx].edge as usize;
        if neighbor_edge_idx >= 4 && (neighbor_edge_idx - 4) & 1 == 0 {
            images[origin_image_idx].edges[origin_edge_idx].edge -= 4;
            images[neighbor_image_idx].data = flip_hori(&images[neighbor_image_idx].data);
            images[neighbor_image_idx].edges.swap(1, 3);
            if images[neighbor_image_idx].edges[0].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[0].edge);
            }
            if images[neighbor_image_idx].edges[2].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[2].edge);
            }
        }

        // Check if the left edge is reversed or the right edge is reversed
        let neighbor_edge_idx = images[origin_image_idx].edges[origin_edge_idx].edge as usize;
        if neighbor_edge_idx >= 4 && (neighbor_edge_idx - 4) & 1 == 1 {
            images[origin_image_idx].edges[origin_edge_idx].edge -= 4;
            images[neighbor_image_idx].data = flip_vert(&images[neighbor_image_idx].data);
            images[neighbor_image_idx].edges.swap(0, 2);
            if images[neighbor_image_idx].edges[1].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[1].edge);
            }
            if images[neighbor_image_idx].edges[3].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[3].edge);
            }
        }

        // println!("Image {}:{} references {}:{}", origin_image_idx, origin_edge_idx, images[origin_image_idx].edges[origin_edge_idx].image, images[origin_image_idx].edges[origin_edge_idx].edge);

        let neighbor_edge_idx = images[origin_image_idx].edges[origin_edge_idx].edge as usize;
        if (origin_edge_idx + 2) % 4 != neighbor_edge_idx {
            panic!();
        }

        if images[neighbor_image_idx].data.len() != IMAGE_SIZE * IMAGE_SIZE {
            panic!();
        }

        let dir = direction(origin_edge_idx as EdgeIdx);
        let neighbor_pos = (origin_pos.0 + dir.0, origin_pos.0 + dir.1);
        align_images(images, processing, neighbor_image_idx, neighbor_pos);
    }
}

fn shift_positions(images: &mut Vec<Image>, count: i32) {
    let mut min_x = 0;
    let mut min_y = 0;
    for image in images.iter() {
        min_x = min_x.min(image.position.0);
        min_y = min_y.min(image.position.1);
    }
    for image in images.iter_mut() {
        image.position.0 -= min_x;
        image.position.1 -= min_y;
        //image.position.0 %= count;
        //image.position.1 %= count;
    }
}

fn construct_image(images: &Vec<Image>, count: usize) -> ImageData {
    if images.len() > count * count {
        panic!();
    }
    let mut full_image = vec![false; IMAGE_SIZE * IMAGE_SIZE * count * count];
    for image in images {
        if image.position.0 < 0 || image.position.1 < 0 {
            panic!();
        }
        if image.position.0 >= count as i32 || image.position.1 >= count as i32 {
            println!("{} {}", image.position.0, image.position.1);
            panic!();
        }
        println!("{}: {} {}", image.id, image.position.0, image.position.1);
        let pos_x = image.position.0 as usize * IMAGE_SIZE;
        let pos_y = image.position.1 as usize * IMAGE_SIZE;
        if image.data.len() != 0 {
            for y in 0..IMAGE_SIZE {
                for x in 0..IMAGE_SIZE {
                    full_image[(pos_y + y) as usize * IMAGE_SIZE + (pos_x + x) as usize] = image.data[y as usize * IMAGE_SIZE + x as usize];
                }
            }
        } else {
            println!("empty");
        }
    }
    full_image
}

fn print_image(image: &ImageData, size: usize) {
    if image.len() != size * size {
        panic!();
    }
    for y in 0..size {
        for x in 0..size {
            if image[y * size + x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let mut images = parse_input();
    println!("{}", images.len());
    set_edges(&mut images);
    println!("Part one: {}", corner_product(&images));
    let mut processing = vec![false; images.len()];
    align_images(&mut images, &mut processing, 0, (0, 0));
    shift_positions(&mut images, 3);
    let full_image = construct_image(&mut images, 4);
    print_image(&full_image, 4 * 10);
}
