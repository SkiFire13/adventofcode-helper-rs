macro_rules! define_ocr {
    (
        mod $mod_name:ident {
            const WIDTH = $width:literal;
            const HEIGHT = $height:literal;
            $(const ENCODED_WIDTH = $encoded_width:literal;)?
            const MAPPED_LETTERS = $mapped_letters:literal;
            const RAW_LETTERS = $raw_letters:literal;
            type EncodedTy = $encoded_ty:ty;

            fn $ocr_letter:ident;
            fn $ocr_generic:ident;
            fn $ocr_bytes:ident;
            fn $ocr_bools:ident;
        }
    ) => {
        pub use $mod_name::ocr_bools as $ocr_bools;
        pub use $mod_name::ocr_bytes as $ocr_bytes;
        pub use $mod_name::ocr_generic as $ocr_generic;
        pub use $mod_name::ocr_letter as $ocr_letter;

        mod $mod_name {
            const WIDTH: usize = $width;
            const HEIGHT: usize = $height;
            const ENCODED_WIDTH: usize = WIDTH $(* 0 + $encoded_width)?;

            const CHAR_LETTERS: &str = $mapped_letters;
            const RAW_LETTERS: [u8; CHAR_LETTERS.len() * WIDTH * HEIGHT] = *$raw_letters;

            type EncodedTy = $encoded_ty;

            static ENCODED: [EncodedTy; CHAR_LETTERS.len()] = {
                let mut arr = [0; CHAR_LETTERS.len()];
                let mut i = 0;
                while i < CHAR_LETTERS.len() {
                    let mut y = 0;
                    while y < HEIGHT {
                        let mut x = 0;
                        while x < ENCODED_WIDTH {
                            let raw_byte = RAW_LETTERS[i * WIDTH + x + CHAR_LETTERS.len() * WIDTH * y];
                            arr[i] = arr[i] << 1 | (raw_byte == b'#') as EncodedTy;
                            x += 1;
                        }
                        y += 1
                    }
                    i += 1;
                }
                arr
            };

            pub fn ocr_letter(n: EncodedTy) -> char {
                let idx = ENCODED
                    .iter()
                    .position(|&b| b == n)
                    .expect("Invalid u32 character");
                    CHAR_LETTERS.as_bytes()[idx] as char
            }

            pub fn ocr_generic<T, F: FnMut(&T) -> bool>(slice: &[T], mut f: F) -> String {
                assert_eq!(slice.len() % (WIDTH * HEIGHT), 0, "Wrong bytes length");
                let num_letters = slice.len() / (WIDTH * HEIGHT);

                (0..num_letters)
                    .map(|i| {
                        let encoded = slice
                            .chunks_exact(WIDTH * num_letters)
                            .flat_map(|chunk| &chunk[i * WIDTH..][..ENCODED_WIDTH])
                            .fold(0, |acc, b| (acc << 1) | f(b) as EncodedTy);
                        ocr_letter(encoded)
                    })
                    .collect()
            }

            pub fn ocr_bytes(bytes: &[u8]) -> String {
                ocr_generic(bytes, |&b| b == b'#')
            }

            pub fn ocr_bools(bytes: &[bool]) -> String {
                ocr_generic(bytes, |&b| b)
            }
        }
    };
}

define_ocr! {
    mod ocr_5x6 {
        const WIDTH = 5;
        const HEIGHT = 6;
        const MAPPED_LETTERS = "ABCEFGHIJKLOPRSUYZ";
        const RAW_LETTERS = b"\
.##..###...##..####.####..##..#..#..###...##.#..#.#.....##..###..###...###.#..#.#...#####.\
#..#.#..#.#..#.#....#....#..#.#..#...#.....#.#.#..#....#..#.#..#.#..#.#....#..#.#...#...#.\
#..#.###..#....###..###..#....####...#.....#.##...#....#..#.#..#.#..#.#....#..#..#.#...#..\
####.#..#.#....#....#....#.##.#..#...#.....#.#.#..#....#..#.###..###...##..#..#...#...#...\
#..#.#..#.#..#.#....#....#..#.#..#...#..#..#.#.#..#....#..#.#....#.#.....#.#..#...#..#....\
#..#.###...##..####.#.....###.#..#..###..##..#..#.####..##..#....#..#.###...##....#..####.";
        type EncodedTy = u32;
        fn ocr_letter_5x6;
        fn ocr_generic_5x6;
        fn ocr_bytes_5x6;
        fn ocr_bools_5x6;
    }
}

define_ocr! {
    mod ocr_7x10 {
        const WIDTH = 7;
        const HEIGHT = 10;
        const ENCODED_WIDTH = 6;
        const MAPPED_LETTERS = "ABCEFGHJKLNPRXZ";
        const RAW_LETTERS = b"\
..##...#####...####..######.######..####..#....#....###.#....#.#......#....#.#####..#####..#....#.######.\
.#..#..#....#.#....#.#......#......#....#.#....#.....#..#...#..#......##...#.#....#.#....#.#....#......#.\
#....#.#....#.#......#......#......#......#....#.....#..#..#...#......##...#.#....#.#....#..#..#.......#.\
#....#.#....#.#......#......#......#......#....#.....#..#.#....#......#.#..#.#....#.#....#..#..#......#..\
#....#.#####..#......#####..#####..#......######.....#..##.....#......#.#..#.#####..#####....##......#...\
######.#....#.#......#......#......#..###.#....#.....#..##.....#......#..#.#.#......#..#.....##.....#....\
#....#.#....#.#......#......#......#....#.#....#.....#..#.#....#......#..#.#.#......#...#...#..#...#.....\
#....#.#....#.#......#......#......#....#.#....#.#...#..#..#...#......#...##.#......#...#...#..#..#......\
#....#.#....#.#....#.#......#......#...##.#....#.#...#..#...#..#......#...##.#......#....#.#....#.#......\
#....#.#####...####..######.#.......###.#.#....#..###...#....#.######.#....#.#......#....#.#....#.######.";
        type EncodedTy = u64;
        fn ocr_letter_7x10;
        fn ocr_generic_7x10;
        fn ocr_bytes_7x10;
        fn ocr_bools_7x10;
    }
}
