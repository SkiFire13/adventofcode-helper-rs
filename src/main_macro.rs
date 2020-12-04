#[macro_export]
macro_rules! main {
    ($year:literal => $($d:ident),* $(,)?) => {
        pub use aoc_helper::prelude;
        pub use aoc_helper::parse_display;
        $( mod $d; )*

        fn main() {
            const DEFAULT_DAY: Option<&str> = $crate::main!(@LATEST_DAY $( $d )*);

            aoc_helper::internal::run_clap($year, DEFAULT_DAY.map(|s| &s[3..]), |requested_day| {
                let mut total = ::std::time::Duration::default();
                let mut found = false;
                $crate::main!(@MAIN $year requested_day total found $( $d )*);
                if found {
                    println!("Took in total: {:.3?}", total);
                } else {
                    println!("No matching day was found")
                }
                println!();
            });
        }
    };
    (@LATEST_DAY) => { None };
    (@LATEST_DAY $d:ident) => { Some(stringify!($d)) };
    (@LATEST_DAY $d:ident $($o:ident)+) => { $crate::main!(@LATEST_DAY $( $o )+) };
    (@MAIN $year:literal $requested_day:ident $total:ident $found:ident $($d:ident)*) => {
        $(
            if $requested_day == Some(&stringify!($d)[3..]) || $requested_day == Some("all") {
                $found = true;
                
                const DAY: &str = stringify!($d);
                println!("Day {:<2}", &DAY[3..]);
                let input = aoc_helper::internal::get_input($year, &DAY[3..]);
            
                let now = std::time::Instant::now();
                let input = $d::input_generator(input.trim_end());
                let elapsed = now.elapsed();
                $total += elapsed;
                println!("     - Parsing input");
                println!("       Took {:.3?}", elapsed);
                println!();
            
                let now = std::time::Instant::now();
                let part1_solution = $d::part1(&input);
                let elapsed = now.elapsed();
                $total += elapsed;
                println!("     - Part 1: {}", part1_solution);
                println!("       Took {:.3?}", elapsed);
                println!();
            
                $crate::main!(@PART2 input $total $d);
            }
        )*
    };
    (@PART2 $input:ident $total:ident day25) => {};
    (@PART2 $input:ident $total:ident $d:ident) => {
        let now = std::time::Instant::now();
        let part2_solution = $d::part2(&$input);
        let elapsed = now.elapsed();
        $total += elapsed;
        println!("     - Part 2: {}", part2_solution);
        println!("       Took {:.3?}", elapsed);
        println!();
    };
}
