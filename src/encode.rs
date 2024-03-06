
use crate::lzss_tuple::LzssTuple;

/*
The prefix search step of lzss.
- buffer: the byte buffer of the input file.
- i: current index. looks back w characters to find prefixes of i..n.
- w: lookback window size, length of the prefix search window (buffer(min([i - w], [0])), buffer[i]).
- n: index of the last byte in suffix window
*/
fn find_lcp(buffer: &mut [u8], i: i32, w: i32, n: i32) -> LzssTuple {
// Find prefixes, and see how long they are. longest wins.
    let mut lcp_len: i32 = 0;
    let mut lcp_d: i32 = 0;
    let mut curr_len: i32 = 0;

    // look back at most w chars
    let mut curr_d: i32 = if i - w < 0 { i } else { w };
    while i - curr_d > 0 && i + curr_len <= n {
        if buffer[(i - curr_d + curr_len) as usize] == buffer[(i + curr_len) as usize] {
            curr_len += 1;
        } else {
            // overhead for storing a tuple
            if curr_len > lcp_len && curr_len > 4 {
                lcp_len = curr_len;
                lcp_d = curr_d;
            }
            curr_len = 0;
            curr_d -= 1;
        }
    }

    if lcp_len == 0 {
        LzssTuple::NoPrefix(0, buffer[i as usize] as char)
    } else {
        LzssTuple::Prefix(lcp_d, lcp_len)
    }
}

pub fn encode(buffer: &mut Vec<u8>, buffer_size: usize, w: i32, n: i32) -> Vec<LzssTuple> {
    // encode chunk
    let mut i = 0;
    let mut end = if n < buffer_size as i32 { n } else { buffer_size as i32 - 1 };
    let mut codes: Vec<LzssTuple> = Vec::new();

    // sliding window on a buffer with soft borders. 
    while i <= end as i32 {
        let code = find_lcp(buffer, i, w, end);
        match code {
            LzssTuple::NoPrefix(_, _) => { 
                i += 1;
                if i >= w && end < buffer_size as i32 - 1 {
                    end += 1;
                }
            }
            LzssTuple::Prefix(_, lcp) => {
                i += lcp;
                if i >= w && end + lcp < buffer_size as i32 - 1 {
                    end += lcp;
                } else if end + lcp >= buffer_size as i32 - 1 {
                    end = buffer_size as i32 - 1;
                }
            }
        }
        codes.push(code);
    }
    codes
}