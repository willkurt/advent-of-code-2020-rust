#[macro_export]
macro_rules! with_read_lines {
    ($file_name:literal, $line_var:expr,$in_loop:expr) => {
        let filename = $file_name;
        if let Ok(lines) = crate::util::read_lines(filename) {
            for line in lines {
                if let Ok(line) = line {
                    $line_var = line;
                    $in_loop
                }
            }
        }
    };
    ($file_name:literal, $line_var:expr, $in_loop:expr, $final_exp:expr) => {
        let filename = $file_name;
        if let Ok(lines) = crate::util::read_lines(filename) {
            for line in lines {
                if let Ok(line) = line {
                    $line_var = line;
                    $in_loop
                }
            }
        }
        $final_exp
    };
}
