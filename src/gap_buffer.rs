pub struct GapBuffer<T> {
    buffer: Vec<T>,
    gap_index: usize,
    right_index: usize,
}

impl<T: Copy + std::fmt::Debug + Default> GapBuffer<T> {
    pub fn from(vec: Vec<T>) -> Self {
        let len = vec.len();

        GapBuffer {
            buffer: vec,
            gap_index: len,
            right_index: len,
        }
    }

    fn gap_size(&self) -> usize {
        self.right_index - self.gap_index
    }

    fn allocate_gap(&mut self) {
        let gap_size = self.buffer.len() / 2;

        self.buffer = vec![
            &self.buffer[0..self.gap_index],
            &std::iter::repeat(Default::default())
                .take(gap_size)
                .collect::<Vec<_>>(),
            &self.buffer[self.gap_index..],
        ]
        .concat();

        self.right_index = self.gap_index + gap_size;
    }

    fn slide_gap_to(&mut self, index: usize) {
        if index < self.gap_index {
            self.buffer.copy_within(
                index..self.gap_index,
                self.right_index - (self.gap_index - index),
            );

            let gap_size = self.gap_size();
            self.gap_index = index;
            self.right_index = index + gap_size;
        } else if index >= self.right_index {
            self.buffer
                .copy_within(self.right_index..index, self.gap_index);

            self.gap_index = index - self.gap_size();
            self.right_index = index;
        }
    }

    fn to_actual_index(&self, virtual_index: usize) -> usize {
        if virtual_index <= self.gap_index {
            virtual_index
        } else {
            virtual_index + self.gap_size()
        }
    }

    // indexはgapを除いた部分のindex
    pub fn insert(&mut self, virtual_index: usize, elem: T) {
        if self.gap_size() == 0 {
            self.allocate_gap();
        }

        let actual_index = self.to_actual_index(virtual_index);

        self.slide_gap_to(actual_index);

        if actual_index == self.gap_index {
            self.buffer[actual_index] = elem;
            self.gap_index += 1;
        } else if actual_index == self.right_index {
            self.buffer[actual_index - 1] = elem;
            self.right_index -= 1;
        } else {
            panic!(
                "{:?} {:?} {:?} {:?}",
                self.buffer, self.gap_index, self.right_index, actual_index
            );
        }
    }

    pub fn delete(&mut self, virtual_index: usize) {
        let actual_index = self.to_actual_index(virtual_index);

        self.slide_gap_to(if actual_index < self.gap_index {
            actual_index + 1
        } else {
            actual_index
        });

        if actual_index + 1 == self.gap_index {
            self.gap_index -= 1;
        } else if actual_index == self.right_index {
            self.right_index += 1;
        } else {
            panic!(
                "{:?} {:?} {:?} {:?}",
                self.buffer, self.gap_index, self.right_index, actual_index
            );
        }
    }

    pub fn as_vec(&self) -> Vec<T> {
        vec![
            &self.buffer[0..self.gap_index],
            &self.buffer[self.right_index..],
        ]
        .concat()
    }

    pub fn operate(&mut self, index: usize, op: Operation<T>) {
        match op {
            Operation::Insert(x) => self.insert(index, x),
            Operation::Delete => self.delete(index),
        }
    }
}

pub enum Operation<T> {
    Insert(T),
    Delete,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert() {
        let cases = vec![
            ("abcde", 0, 'k', "kabcde"),
            ("abcde", 1, 'k', "akbcde"),
            ("abcde", 5, 'k', "abcdek"),
        ];

        for (init, index, char, result) in cases {
            let mut buf = GapBuffer::from(init.chars().collect::<Vec<_>>());

            buf.insert(index, char);
            assert_eq!(buf.as_vec().iter().collect::<String>(), result);
        }
    }

    #[test]
    fn insert_many() {
        let cases = vec![
            (
                "abcde",
                vec![(0, 'k'), (1, 'l'), (2, 'm'), (3, 'n')],
                "klmnabcde",
            ),
            (
                "abcde",
                vec![(0, 'k'), (1, 'l'), (2, 'm'), (7, 'n'), (9, 'o')],
                "klmabcdneo",
            ),
        ];

        for (init, ops, result) in cases {
            let mut buf = GapBuffer::from(init.chars().collect::<Vec<_>>());

            for (index, ch) in ops {
                buf.insert(index, ch);
                println!("{:?}", buf.buffer);
            }

            assert_eq!(buf.as_vec().iter().collect::<String>(), result);
        }
    }

    #[test]
    fn delete() {
        let cases = vec![
            ("abcde", 0, "bcde"),
            ("abcde", 1, "acde"),
            ("abcde", 4, "abcd"),
        ];

        for (init, index, result) in cases {
            let mut buf = GapBuffer::from(init.chars().collect::<Vec<_>>());

            buf.delete(index);
            assert_eq!(buf.as_vec().iter().collect::<String>(), result);
        }
    }

    #[test]
    fn ex() {
        let case = (
            "abcde",
            vec![
                (Operation::Insert('k'), 2, "abkcde"),
                (Operation::Delete, 4, "abkce"),
                (Operation::Insert('u'), 3, "abkuce"),
                (Operation::Insert('v'), 5, "abkucve"),
            ],
        );

        let mut buf = GapBuffer::from(case.0.chars().collect::<Vec<_>>());
        for (op, i, result) in case.1 {
            buf.operate(i, op);
            println!("{:?} {:?} {:?}", buf.buffer, buf.gap_index, buf.right_index);
            assert_eq!(buf.as_vec().iter().collect::<String>(), result);
        }
    }
}
