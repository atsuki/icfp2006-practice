use std::fs::File;
use std::num::Wrapping;
use std::io::{Read, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut reader = BufReader::new(File::open("/home/atsuki/dev/icfp2006/materials/sandmark.umz")?);
    // let mut reader = BufReader::new(File::open("/home/atsuki/dev/icfp2006/materials/codex.umz")?);
    let mut reader = BufReader::new(File::open("/home/atsuki/dev/icfp2006/materials/umix.um")?);
    let mut buf: [u8; 4] = [0; 4];
    
    // TODO: ファイルサイズ分確保する。
    let mut platters = Vec::new();
    
    while reader.read_exact(&mut buf).is_ok() {
        // TODO: unexpected EOF以外のエラーケースでの処理
        platters.push(buffer_to_platter(&buf));
    }
    
    run(platters);

    Ok(())
}

fn run(platters: Vec<u32>) {
    let mut arrays: Vec<Box<[u32]>> = Vec::new();
    let mut ef : usize = 0; // the execution finger
    let mut reg: [u32; 8] = [0; 8]; // registers
    let mut alloc_pool: Vec<usize> = Vec::new();
    let mut read_buf: [u8; 1] = [0];

    arrays.push(platters.into_boxed_slice());

    loop {
        let p = arrays[0][ef];
        ef += 1;
        match p!(OPERATOR, p) {
            // #0. Conditional Move.
            0 => {
                if reg[p!(C, p)] != 0 {
                    reg[p!(A, p)] = reg[p!(B, p)];
                }
            },
            // #1. Array Index.
            1 => {
                reg[p!(A, p)] = arrays[reg[p!(B, p)] as usize][reg[p!(C, p)] as usize];
            },
            // #2. Array Amendment.
            2 => {
                arrays[reg[p!(A, p)] as usize][reg[p!(B, p)] as usize] = reg[p!(C, p)];
            },
            // #3. Addition.
            3 => {
                reg[p!(A, p)] = (Wrapping(reg[p!(B, p)]) + Wrapping(reg[p!(C, p)])).0;
            },
            // #4. Multiplication.
            4 => {
                reg[p!(A, p)] = (Wrapping(reg[p!(B, p)]) * Wrapping(reg[p!(C, p)])).0;
            },
            // #5. Division.
            5 => {
                reg[p!(A, p)] = reg[p!(B, p)] / reg[p!(C, p)];
            },
            // #6. Not-And.
            6 => {
                reg[p!(A, p)] = !(reg[p!(B, p)] & reg[p!(C, p)]);
            },
            // #7. Halt.
            7 => {
                break;
            },
            // #8. Allocation.
            8 => {
                let a = vec![0; reg[p!(C, p)] as usize].into_boxed_slice();
                
                match alloc_pool.pop() {
                    None => {
                        arrays.push(a);
                        reg[p!(B, p)] = arrays.len() as u32 - 1;
                    },
                    Some(i) => {
                        arrays[i] = a;
                        reg[p!(B, p)] = i as u32;
                    }
                }
            },
            // #9. Abandonment.
            9 => {
                let index = reg[p!(C, p)] as usize;
                arrays[index] = Box::new([]);
                alloc_pool.push(index)
            },
            // #10. Output.
            10 => {
                let c = std::char::from_u32(reg[p!(C, p)]).unwrap();
                print!("{}", c);
            },
            // #11. Input.
            11 => {
                let mut handle = std::io::stdin().take(1);
                handle.read(&mut read_buf).unwrap();
                if read_buf[0] == 0x0A {
                    read_buf[0] = 0xFF;
                }
                reg[p!(C, p)] = read_buf[0] as u32;
            },
            // #12. Load Program.
            12 => {
                ef = reg[p!(C, p)] as usize;
                arrays[0] = arrays[reg[p!(B, p)] as usize].clone();
            },
            // #13. Orthography.
            13 => {
                reg[p!(SPECIAL_A, p)] = p!(SPECIAL_VALUE, p);
            },
            _ => {
            }
        }

    }
}

#[macro_export]
macro_rules! p {
    (OPERATOR, $p: ident) => {
        (($p & 0xF0000000) >> 28) as u8
    };
    (A, $p: ident) => {
        (($p & 0b111000000) >> 6) as usize
    };
    (B, $p: ident) => {
        (($p & 0b111000) >> 3) as usize
    };
    (C, $p: ident) => {
        ($p & 0b111) as usize
    };
    (SPECIAL_A, $p: ident) => {
        (($p & 0x0E000000) >> 25) as usize
    };
    (SPECIAL_VALUE, $p: ident) => {
        ($p & 0x01FFFFFF) as u32
    };
}

fn buffer_to_platter(buffer: &[u8; 4]) -> u32 {
    ((buffer[0] as u32) << 24) |
    ((buffer[1] as u32) << 16) |
    ((buffer[2] as u32) << 8) |
    (buffer[3] as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_to_platter() {
        assert_eq!(buffer_to_platter(&[0, 0, 0, 0]), 0);
        assert_eq!(buffer_to_platter(&[0xFF, 0xFF, 0xFF, 0xFF]), 0xFFFFFFFF);
        assert_eq!(buffer_to_platter(&[0x12, 0x34, 0x56, 0x78]), 0x12345678);
    }

    #[test]
    fn test_get_func() {
        // TODO: マクロ版のテストコード
        // assert_eq!(get_operator(0b10001000000000000000000000000000), 0b1000);
        // assert_eq!(get_operator(0b01110000000000000000000000000000), 0b0111);

        // assert_eq!(get_a(0b00000000000000000000000111000000), 0b111);
        // assert_eq!(get_a(0b00000000000000000000001000100000), 0b000);

        // assert_eq!(get_b(0b00000000000000000000000000111000), 0b111);
        // assert_eq!(get_b(0b00000000000000000000000001000100), 0b000);

        // assert_eq!(get_c(0b00000000000000000000000000000111), 0b111);
        // assert_eq!(get_c(0b10000000000000000000000000001000), 0b000);

        // assert_eq!(get_sp_a(0b00001110000000000000000000000000), 0b111);
        // assert_eq!(get_sp_a(0b00010001000000000000000000000000), 0b000);

        // assert_eq!(get_sp_value(0b00000001000000000000000000000001), 0b1000000000000000000000001);
        // assert_eq!(get_sp_value(0b10000010000000000000000000000000), 0b0000000000000000000000000);
    }
}