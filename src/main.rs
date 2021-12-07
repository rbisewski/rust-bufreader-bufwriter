use std::io::{self, Read, Stdin, Stdout, BufWriter, Write};
use std::io::BufReader;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let reader: BufReader<Stdin> = BufReader::with_capacity(1, stdin);
    let mut writer: BufWriter<Stdout> = BufWriter::with_capacity(1, stdout);

    // read stdin 1 byte at a time
    for byte in reader.bytes() {
        // either an issue or the EOF
        if byte.is_err() {
            break;
        }

        // convert the byte to a character
        let char_star = &[byte.unwrap()];
        let character = match std::str::from_utf8(char_star) {
            Ok(c) => c.to_uppercase(),
            Err(e) => panic!("{}", e),
        };

        // attempt to write Stdout, one byte at a time
        let res = writer.write(character.as_bytes());
        if res.is_err() {
            break;
        }
    }

    Ok(())
}
