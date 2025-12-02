mod new_notes;

fn main() {
    let path = "notes.txt";

    // 1. CREATE (overwrite if exists)
    {
        let mut file = File::create(path)?;
        writeln!(file, "First line")?;
        writeln!(file, "Second line")?;
    }

    // 2. APPEND
    {
        let mut file = OpenOptions::new()
            .append(true)
            .open(path)?;
        writeln!(file, "Appended line")?;
    }

    // 3. READ + SEARCH
    {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        println!("--- File contents ---");
        println!("{contents}");

        // Find lines containing a word
        let query = "line";
        println!("\n--- Search results for '{query}' ---");
        for (i, line) in contents.lines().enumerate() {
            if line.contains(query) {
                println!("Line {}: {}", i + 1, line);
            }
        }
    }

    // 4. DELETE PARTS (rewrite file)
    {
        // Read entire file
        let mut contents = String::new();
        File::open(path)?.read_to_string(&mut contents)?;

        // Remove any line containing “Second”
        let new_contents: String = contents
            .lines()
            .filter(|line| !line.contains("Second"))
            .map(|line| format!("{line}\n"))
            .collect();

        // Overwrite file with updated content
        let mut file = File::create(path)?;
        file.write_all(new_contents.as_bytes())?;
    }

    println!("\nUpdated file written!");

    Ok(())
}

