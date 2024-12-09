fn read_input<P>(filename: P) -> std::io::Result<String>
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(filename)
}

#[derive(Debug)]
struct File {
    start: usize,
    size: usize,
}

fn get_disk_repr(disk_map: &str) -> Result<(Vec<Option<usize>>, Vec<File>), String> {
    let mut repr = vec![];
    let mut files = vec![];

    let mut is_file = [true, false].iter().cycle();
    let mut i = 0;

    for ch in disk_map.chars() {
        let n = ch
            .to_digit(10)
            .ok_or_else(|| format!("Unknown block length {}", ch))? as usize;

        let is_file = is_file.next().unwrap();

        if *is_file {
            files.push(File {
                start: repr.len(),
                size: n,
            });
            repr.extend([Some(i)].repeat(n));
            i += 1;
        } else {
            repr.extend([None].repeat(n));
        }
    }

    Ok((repr, files))
}

fn find_free_span(disk_repr: &[Option<usize>], start: usize) -> (usize, usize) {
    let mut i = start;

    while i < disk_repr.len() {
        if disk_repr[i].is_none() {
            let start = i;
            while i < disk_repr.len() && disk_repr[i].is_none() {
                i += 1;
            }
            return (start, i - start);
        }
        i += 1;
    }

    (disk_repr.len(), 0)
}

fn compact(disk_repr: &mut [Option<usize>], files: &[File]) {
    for file in files.iter().rev() {
        let (mut free_start, mut free_size) = find_free_span(disk_repr, 0);

        while free_start + free_size < disk_repr.len() && free_size < file.size {
            (free_start, free_size) = find_free_span(disk_repr, free_start + free_size);
        }

        if free_start != disk_repr.len() && free_size >= file.size && free_start < file.start {
            let mut i = free_start;
            let mut j = file.start;
            println!("free span: {} + {}", i, free_size);

            while j < file.start + file.size {
                println!("swapping {} with {}", i, j);
                disk_repr.swap(i, j);
                i += 1;
                j += 1;
            }
        }
    }
}

fn checksum(disk_repr: &[Option<usize>]) -> usize {
    disk_repr
        .iter()
        .enumerate()
        .fold(0, |acc, (i, block)| match block {
            Some(id) => acc + i * id,
            None => acc,
        })
}

fn print_disk_repr(disk_repr: &[Option<usize>]) {
    let repr: String = disk_repr
        .iter()
        .map(|block| match block {
            Some(id) => id.to_string(),
            None => ".".to_string(),
        })
        .collect();
    println!("{}", repr);
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
