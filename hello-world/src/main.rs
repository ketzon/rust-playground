use unicode_segmentation::UnicodeSegmentation;

fn byte_len(s: &str) -> usize {
    s.len()
}

fn char_len(s: &str) -> usize {
    s.chars().count()
}

fn grapheme_len(s: &str) -> usize {
    s.graphemes(true).count()
}

fn main() {
    println!("Hello, world!");

    let a = "hello";
    let b = "café";   
    let c = "👍🏽";  

    for s in [a, b, c] {
        println!(
            "'{s}' -> bytes: {}, chars: {}, graphèmes: {}",
            byte_len(s),
            char_len(s),
            grapheme_len(s)
        );
    }
}
