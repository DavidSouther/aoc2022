use std::io::stdin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut elves = Vec::<u32>::new();
    let mut meal = 0u32;
    let input = &mut String::new();

    while stdin().read_line(input)? > 0 {
        if matches!(input.as_str(), "\n") {
            elves.push(meal);
            meal = 0;
        } else {
            let input = input.trim();
            eprintln!("Parsing: `{input}`");
            meal += input.parse::<u32>()?;
        }
        input.clear();
    }

    elves.sort();
    elves.reverse();

    println!("{}", elves[0]);
    println!("{}", elves[0] + elves[1] + elves[2]);

    Ok(())
}
