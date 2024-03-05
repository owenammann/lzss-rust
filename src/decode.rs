use crate::lzss_tuple::LzssTuple;

pub fn decode(codes: Vec<LzssTuple>) -> String {
    let mut chars = Vec::new();
    let mut i = 0;

    for code in codes {
        match code {
            LzssTuple::NoPrefix(_, ch) => {
                chars.push(ch);
                i += 1;
            }
            LzssTuple::Prefix(d, lcp) => {
                let start = (i as i32 - d) as usize;
                let end = start + lcp as usize;

                if end as i32 <= i {
                    let ch_opt = chars.get(start..end);
                    match ch_opt {
                        None =>  { 
                            panic!("Didn't get a char to add to decoded string!");
                        }
                        Some(ch_slice) => { 
                            chars.extend(ch_slice.to_vec());
                         }
                    }
                } else {
                    //The prefix was longer than w so we wrap around.
                    let wrap_around = (end as i32 - i) as usize;
                    let ch_opt = chars.get(start..i as usize);
                    match ch_opt {
                        None =>  { 
                            panic!("Didn't get a char to add to decoded string!");
                        }
                        Some(ch_slice) => { 
                            chars.extend(ch_slice.to_vec());
                         }
                    }

                    let ch_opt_2 = chars.get(start..start + wrap_around as usize);

                    match ch_opt_2 {
                        None =>  { 
                            panic!("Didn't get a char to add to wraparound string!");
                        }
                        Some(ch_slice) => { 
                            chars.extend(ch_slice.to_vec());
                         }
                    }
                }
                i += lcp;

            }
        };
        //let debug: String =  chars.iter().collect();
        // println!("so far: {:?}", debug);
    }

    chars.iter().collect()
}