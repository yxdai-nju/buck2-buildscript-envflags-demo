use mime_guess::from_path;
use ring::digest::{Context, SHA256};

fn main() {
    // Use ring to hash a string
    let data = b"Hello, Ring!";
    let hex_digest = hash_bytes(data);
    println!(
        "SHA-256 hash of \"{}\": {}",
        std::str::from_utf8(data).unwrap(),
        hex_digest
    );

    let file_path = "example.txt";

    // Use mime_guess to determine the MIME type based on file extension
    let mime = from_path(file_path).first_or_octet_stream();
    println!("MIME type of '{file_path}': {mime}");
}

// Function to hash bytes
fn hash_bytes(data: &[u8]) -> String {
    // Create a SHA-256 context
    let mut context = Context::new(&SHA256);

    // Update the context with the data
    context.update(data);

    // Finalize and get the digest
    let digest = context.finish();

    // Convert the digest to a hexadecimal string
    digest
        .as_ref()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>()
}
