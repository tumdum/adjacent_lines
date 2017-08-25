extern crate iterslide;
extern crate itertools;

use std::io::{Read, BufRead, BufReader};

/// Applies `f` to each pair of adjacent lines in `r`.
pub fn visit<R: Read, F: FnMut(&str, &str)>(r: R, mut f: F) {
    use iterslide::SlideIterator;
    use itertools::Itertools;

    BufReader::new(r)
        .lines()
        .map(|r| r.ok())
        .fuse()
        .map(|o| o.unwrap())
        .slide(2)
        .foreach(|s| f(s.get(0).unwrap(), s.get(1).unwrap()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut seen = vec![];
        let input = "aa\nbb\ncc\ndd";
        {
            let collector = |a: &str, b: &str| { seen.push((a.to_owned(), b.to_owned())); };
            visit(std::io::Cursor::new(input), collector);
        }
        assert_eq!(
            seen,
            vec![
                ("aa".into(), "bb".into()),
                ("bb".into(), "cc".into()),
                ("cc".into(), "dd".into()),
            ]
        );
    }
}
