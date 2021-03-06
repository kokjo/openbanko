use combinations::Combination;
use std::u64;
use typenum::consts::*;

pub struct LayoutTable {
    table: Box<[[[u64; 126]; 126]; 126]>,
}

impl LayoutTable {
    #[inline]
    pub fn get(&self, row0: u8, row1: u8, row2: u8) -> Option<u64> {
        if row0 >= 126 || row1 >= 126 || row2 >= 126 {
            None
        } else {
            let result = self.table[row0 as usize][row1 as usize][row2 as usize];

            if result == u64::MAX {
                None
            } else {
                Some(result)
            }
        }
    }
}

impl Default for LayoutTable {
    #[inline]
    fn default() -> LayoutTable {
        let table: Vec<[[u64; 126]; 126]> = vec![[[0; 126]; 126]; 126];
        let table: Box<[[[u64; 126]; 126]]> = table.into_boxed_slice();
        let table: *mut [[[u64; 126]; 126]] = Box::into_raw(table);
        let table = table as *mut [[[u64; 126]; 126]; 126];
        let mut table = unsafe { Box::from_raw(table) };

        let mut total_count = 0;

        let counts_col0 = [
            Combination::<U8, U0>::total_count(),
            Combination::<U8, U1>::total_count(),
            Combination::<U8, U2>::total_count(),
            Combination::<U8, U3>::total_count(),
        ];

        let counts_col_most = [
            Combination::<U9, U0>::total_count(),
            Combination::<U9, U1>::total_count(),
            Combination::<U9, U2>::total_count(),
            Combination::<U9, U3>::total_count(),
        ];

        let counts_col8 = [
            Combination::<U10, U0>::total_count(),
            Combination::<U10, U1>::total_count(),
            Combination::<U10, U2>::total_count(),
            Combination::<U10, U3>::total_count(),
        ];

        for (ndx0, row0) in Combination::<U8, U5>::new().enumerate() {
            for (ndx1, row1) in Combination::<U8, U5>::new().enumerate() {
                for (ndx2, row2) in Combination::<U8, U5>::new().enumerate() {
                    let mut count = [0; 9];

                    for &value in row0.iter().chain(row1.iter()).chain(row2.iter()) {
                        count[value as usize] += 1;
                    }

                    if count.iter().any(|&n| n == 0) {
                        continue;
                    }

                    table[ndx0][ndx1][ndx2] = total_count;
                    total_count += counts_col0[count[0]]
                        * (1..8).map(|n| counts_col_most[count[n]]).product::<u64>()
                        * counts_col8[count[8]];
                }
            }
        }

        LayoutTable { table }
    }
}
