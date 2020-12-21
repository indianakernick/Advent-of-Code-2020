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

fn parse_input(path: &str) -> Vec<Image> {
    let mut images = Vec::new();
    let mut line_iter = line_iter_from_file(path);
    while let Some(title_line) = line_iter.next() {
        let id = title_line[5..9].parse::<ImageID>().unwrap();
        let mut data = ImageData::new();
        for _ in 0..IMAGE_SIZE {
            let mut line = line_iter.next().unwrap().chars().map(|ch| ch == '#').collect();
            data.append(&mut line);
        }
        line_iter.next(); // blank line
        assert_eq!(data.len(), IMAGE_SIZE * IMAGE_SIZE);
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
    let mut border = Vec::with_capacity(IMAGE_SIZE);
    for i in 0..IMAGE_SIZE {
        border.push(image[0 * IMAGE_SIZE + i]);
    }
    border
}

fn bottom_edge(image: &ImageData) -> ImageBorder {
    let mut border = Vec::with_capacity(IMAGE_SIZE);
    let last = IMAGE_SIZE - 1;
    for i in 0..IMAGE_SIZE {
        border.push(image[last * IMAGE_SIZE + last - i]);
    }
    border
}

fn left_edge(image: &ImageData) -> ImageBorder {
    let mut border = Vec::with_capacity(IMAGE_SIZE);
    let last = IMAGE_SIZE - 1;
    for i in 0..IMAGE_SIZE {
        border.push(image[(last - i) * IMAGE_SIZE + 0]);
    }
    border
}

fn right_edge(image: &ImageData) -> ImageBorder {
    let mut border = Vec::with_capacity(IMAGE_SIZE);
    let last = IMAGE_SIZE - 1;
    for i in 0..IMAGE_SIZE {
        border.push(image[i * IMAGE_SIZE + last]);
    }
    border
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

fn flip_vert_sized(data: &ImageData, size: usize) -> ImageData {
    assert_eq!(data.len(), size * size);
    let mut result = ImageData::new();
    for y in (0..size).rev() {
        for x in 0..size {
            result.push(data[y * size + x]);
        }
    }
    result
}

fn flip_vert(data: &ImageData) -> ImageData {
    flip_vert_sized(data, IMAGE_SIZE)
}

fn flip_hori_sized(data: &ImageData, size: usize) -> ImageData {
    assert_eq!(data.len(), size * size);
    let mut result = ImageData::new();
    for y in 0..size {
        for x in (0..size).rev() {
            result.push(data[y * size + x]);
        }
    }
    result
}

fn flip_hori(data: &ImageData) -> ImageData {
    flip_hori_sized(data, IMAGE_SIZE)
}

fn transpose_sized(data: &ImageData, size: usize) -> ImageData {
    assert_eq!(data.len(), size * size);
    let mut result = ImageData::new();
    for x in 0..size {
        for y in 0..size {
            result.push(data[y * size + x]);
        }
    }
    result
}

fn rotate_cw_sized(data: &ImageData, size: usize) -> ImageData {
    flip_hori_sized(&transpose_sized(data, size), size)
}

fn rotate_ccw_sized(data: &ImageData, size: usize) -> ImageData {
    flip_vert_sized(&transpose_sized(data, size), size)
}

fn rotate_cw(data: &ImageData) -> ImageData {
    rotate_cw_sized(data, IMAGE_SIZE)
}

fn rotate_ccw(data: &ImageData) -> ImageData {
    rotate_ccw_sized(data, IMAGE_SIZE)
}

fn flip_idx(edge_idx: &mut EdgeIdx) {
    if *edge_idx >= 4 {
        *edge_idx = (*edge_idx + 2) % 4;
    } else {
        *edge_idx = (*edge_idx + 2) % 4 + 4;
    }
}

fn rotate_idx_ccw(edge_idx: &mut EdgeIdx) {
    if *edge_idx >= 4 {
        *edge_idx = (*edge_idx + 3) % 4 + 4;
    } else {
        *edge_idx = (*edge_idx + 3) % 4;
    }
}

fn rotate_idx_cw(edge_idx: &mut EdgeIdx) {
    if *edge_idx >= 4 {
        *edge_idx = (*edge_idx + 1) % 4 + 4;
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
        1 => (1, 0),
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

        // Check if top matches right or (right matches bottom) or bottom matches left or (left matches top)
        let neighbor_edge_idx = images[origin_image_idx].edges[origin_edge_idx].edge as usize;
        if (origin_edge_idx + 1) % 4 == neighbor_edge_idx % 4 {
            rotate_idx_cw(&mut images[origin_image_idx].edges[origin_edge_idx].edge);
            images[neighbor_image_idx].data = rotate_cw(&images[neighbor_image_idx].data);
            images[neighbor_image_idx].edges.rotate_right(1);
        }

        // Check if (top matches left) or left matches bottom or (bottom matches right) or right matches top
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
            if images[neighbor_image_idx].edges[0].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[0].edge);
            }
            if images[neighbor_image_idx].edges[1].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[1].edge);
            }
            if images[neighbor_image_idx].edges[2].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[2].edge);
            }
            if images[neighbor_image_idx].edges[3].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[3].edge);
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
            }
            if images[neighbor_image_idx].edges[1].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[1].edge);
            }
            if images[neighbor_image_idx].edges[2].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[2].edge);
            }
            if images[neighbor_image_idx].edges[3].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[3].edge);
            }
        }

        // Check if the top edge is reversed or the bottom edge is reversed
        let neighbor_edge_idx = images[origin_image_idx].edges[origin_edge_idx].edge as usize;
        if neighbor_edge_idx < 4 && (neighbor_edge_idx) & 1 == 0 {
            reverse_edge(&mut images[origin_image_idx].edges[origin_edge_idx].edge);
            images[neighbor_image_idx].data = flip_hori(&images[neighbor_image_idx].data);
            images[neighbor_image_idx].edges.swap(1, 3);
            if images[neighbor_image_idx].edges[0].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[0].edge);
            }
            if images[neighbor_image_idx].edges[1].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[1].edge);
            }
            if images[neighbor_image_idx].edges[2].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[2].edge);
            }
            if images[neighbor_image_idx].edges[3].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[3].edge);
            }
        }

        // Check if the left edge is reversed or the right edge is reversed
        let neighbor_edge_idx = images[origin_image_idx].edges[origin_edge_idx].edge as usize;
        if neighbor_edge_idx < 4 && (neighbor_edge_idx) & 1 == 1 {
            reverse_edge(&mut images[origin_image_idx].edges[origin_edge_idx].edge);
            images[neighbor_image_idx].data = flip_vert(&images[neighbor_image_idx].data);
            images[neighbor_image_idx].edges.swap(0, 2);
            if images[neighbor_image_idx].edges[0].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[0].edge);
            }
            if images[neighbor_image_idx].edges[1].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[1].edge);
            }
            if images[neighbor_image_idx].edges[2].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[2].edge);
            }
            if images[neighbor_image_idx].edges[3].image != ImageIdx::MAX {
                reverse_edge(&mut images[neighbor_image_idx].edges[3].edge);
            }
        }

        let neighbor_edge_idx = images[origin_image_idx].edges[origin_edge_idx].edge as usize;
        assert_eq!(neighbor_edge_idx, (origin_edge_idx + 2) % 4 + 4);

        let dir = direction(origin_edge_idx as EdgeIdx);
        let neighbor_pos = (origin_pos.0 + dir.0, origin_pos.1 + dir.1);
        align_images(images, processing, neighbor_image_idx, neighbor_pos);
    }
}

fn shift_positions(images: &mut Vec<Image>) {
    let mut min_x = 0;
    let mut min_y = 0;
    for image in images.iter() {
        min_x = min_x.min(image.position.0);
        min_y = min_y.min(image.position.1);
    }
    for image in images.iter_mut() {
        image.position.0 -= min_x;
        image.position.1 -= min_y;
    }
}

fn construct_image(images: &Vec<Image>, count: usize) -> ImageData {
    assert_eq!(images.len(), count * count);
    let tile_size = IMAGE_SIZE - 2;
    let full_size = tile_size * count;
    let mut full_image = vec![false; full_size * full_size];
    for image in images {
        assert!((image.position.0 as usize) < count && (image.position.1 as usize) < count);
        let pos_x = image.position.0 as usize * tile_size;
        let pos_y = image.position.1 as usize * tile_size;
        for y in 0..tile_size {
            for x in 0..tile_size {
                full_image[(pos_y + y) * full_size + pos_x + x] = image.data[(y + 1) * IMAGE_SIZE + (x + 1)];
            }
        }
    }
    full_image
}

fn print_image(image: &ImageData, size: usize) {
    assert_eq!(image.len(), size * size);
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

fn print_ids(images: &Vec<Image>, count: usize) {
    assert_eq!(images.len(), count * count);
    let mut ids = vec![0; count * count];
    for image in images {
        assert!((image.position.0 as usize) < count && (image.position.1 as usize) < count);
        ids[image.position.1 as usize * count + image.position.0 as usize] = image.id;
    }
    for y in 0..count {
        for x in 0..count {
            if ids[y * count + x] == 0 {
                print!("     ");
            } else {
                print!("{} ", ids[y * count + x]);
            }
        }
        println!();
    }
}

fn count_set_pixels(image: &ImageData) -> usize {
    image.iter().filter(|p| **p).count()
}

const MONSTER_PIXELS: usize = 15;

fn count_sea_monsters(image: &ImageData, image_size: usize) -> usize {
    //                   #
    // #    ##    ##    ###
    //  #  #  #  #  #  #
    let monster = vec![
        false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true , false,
        true , false, false, false, false, true , true , false, false, false, false, true , true , false, false, false, false, true , true , true ,
        false, true , false, false, true , false, false, true , false, false, true , false, false, true , false, false, true , false, false, false,
    ];
    let monster_width = 20usize;
    let monster_height = 3usize;
    assert_eq!(monster.len(), monster_width * monster_height);
    assert_eq!(count_set_pixels(&monster), MONSTER_PIXELS);

    let mut count = 0;

    for img_y in 0..(image_size - monster_height + 1) {
        for img_x in 0..(image_size - monster_width + 1) {
            let mut found_monster = true;
            for mon_y in 0..monster_height {
                for mon_x in 0..monster_width {
                    let mon_px = monster[mon_y * monster_width + mon_x];
                    let img_px = image[(img_y + mon_y) * image_size + (img_x + mon_x)];
                    if mon_px && !img_px {
                        found_monster = false;
                        break;
                    }
                }
                if !found_monster {
                    break;
                }
            }
            if found_monster {
                count += 1;
            }
        }
    }

    count
}

fn transform_count_sea_monsters(image: &mut ImageData, image_size: usize) -> usize {
    // It would be a lot more efficient to transform the sea monster instead of
    // the full image but who cares

    for _ in 0..4 {
        let count = count_sea_monsters(image, image_size);
        if count != 0 { return count; }
        *image = rotate_cw_sized(image, image_size);
    }

    *image = flip_vert_sized(image, image_size);

    for _ in 0..4 {
        let count = count_sea_monsters(image, image_size);
        if count != 0 { return count; }
        *image = rotate_cw_sized(image, image_size);
    }

    *image = flip_hori_sized(image, image_size);

    for _ in 0..4 {
        let count = count_sea_monsters(image, image_size);
        if count != 0 { return count; }
        *image = rotate_cw_sized(image, image_size);
    }

    *image = flip_vert_sized(image, image_size);

    for _ in 0..4 {
        let count = count_sea_monsters(image, image_size);
        if count != 0 { return count; }
        *image = rotate_cw_sized(image, image_size);
    }

    return 0;
}

fn main() {
    let mut images = parse_input("input/day_20.txt");
    set_edges(&mut images);
    println!("Part one: {}", corner_product(&images));
    let mut processing = vec![false; images.len()];
    align_images(&mut images, &mut processing, 0, (0, 0));
    shift_positions(&mut images);

    let count = 12usize;
    let image_size = count * (IMAGE_SIZE - 2);

    let mut full_image = construct_image(&mut images, count);
    let set_pixels = count_set_pixels(&full_image);
    let monster_count = transform_count_sea_monsters(&mut full_image, image_size);
    let roughness = set_pixels - monster_count * MONSTER_PIXELS;
    println!("Part two: {}", roughness);
}
