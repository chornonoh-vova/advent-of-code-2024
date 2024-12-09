fn read_input<P>(filename: P) -> std::io::Result<String>
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(filename)
}

#[derive(Debug, PartialEq, Eq)]
enum Block {
    File(usize),
    Empty,
}

#[derive(Debug)]
struct File {
    start: usize,
    size: usize,
}

fn get_disk_repr(disk_map: &str) -> Result<(Vec<Block>, Vec<File>), String> {
    let mut repr = vec![];
    let mut files = vec![];

    let mut is_file = true;
    let mut i = 0;

    for ch in disk_map.chars() {
        let n = ch
            .to_digit(10)
            .ok_or(format!("Unknown block length {}", ch))?;

        if is_file {
            files.push(File {
                start: repr.len(),
                size: n as usize,
            });
        }

        for _ in 0..n {
            match is_file {
                true => repr.push(Block::File(i)),
                false => repr.push(Block::Empty),
            };
        }

        if is_file {
            i += 1;
        }

        is_file = !is_file;
    }

    Ok((repr, files))
}

fn find_free_span(disk_repr: &[Block], start: usize) -> (usize, usize) {
    let mut i = start;
    let mut size = 0;

    while i < disk_repr.len() && disk_repr[i] != Block::Empty {
        i += 1;
    }

    while i + size < disk_repr.len() && disk_repr[i + size] == Block::Empty {
        size += 1;
    }

    (i, size)
}

fn compact(disk_repr: &mut [Block], files: &[File]) {
    for curr_file in files.iter().rev() {
        let (mut free_span_start, mut free_span_size) = find_free_span(disk_repr, 0);

        while free_span_start + free_span_size < disk_repr.len() && free_span_size < curr_file.size
        {
            (free_span_start, free_span_size) =
                find_free_span(disk_repr, free_span_start + free_span_size);
        }

        if free_span_start != disk_repr.len()
            && free_span_size >= curr_file.size
            && free_span_start < curr_file.start
        {
            let mut i = free_span_start;
            let mut j = curr_file.start;
            println!("free span: {} + {}", i, free_span_size);

            while j < curr_file.start + curr_file.size {
                println!("swapping {} with {}", i, j);
                disk_repr.swap(i, j);
                i += 1;
                j += 1;
            }
        }
    }
}

fn checksum(disk_repr: &[Block]) -> usize {
    let mut sum = 0;

    for (i, block) in disk_repr.iter().enumerate() {
        match block {
            Block::File(id) => sum += i * id,
            _ => {}
        }
    }

    sum
}

fn print_disk_repr(disk_repr: &[Block]) {
    for el in disk_repr {
        match el {
            Block::File(file_id) => print!("{}", file_id),
            Block::Empty => print!("."),
        }
    }
    print!("\n");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let disk_map = read_input(filename)?;
    println!("disk map: {}", disk_map);

    let (mut disk_repr, files) = get_disk_repr(&disk_map)?;
    print_disk_repr(&disk_repr);
    println!("files count: {}", files.len());

    compact(&mut disk_repr, &files);
    print_disk_repr(&disk_repr);
    println!("checksum: {}", checksum(&disk_repr));

    Ok(())
}
