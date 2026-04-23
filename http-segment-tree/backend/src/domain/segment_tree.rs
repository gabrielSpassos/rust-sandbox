pub struct SegmentTree {
    n: usize,
    max_index: usize,
    array: Vec<i32>,
    tree: Vec<i32>,
}

impl SegmentTree {
    pub fn new(input: Vec<i32>) -> Self {
        let n = input.len();
        let mut st = SegmentTree {
            n,
            max_index: 0,
            array: input,
            tree: vec![0; 4 * n], // safer than 2*n for recursive tree
        };

        if n > 0 {
            st.build(0, 0, n - 1);
        }

        st
    }

    pub fn get_tree(&self) -> &Vec<i32> {
        &self.tree
    }

    pub fn get_array(&self) -> &Vec<i32> {
        &self.array
    }

    pub fn update(&mut self, idx: usize, new_value: i32) {
        if self.n > 0 {
            self.update_internal(0, 0, self.n - 1, idx, new_value);
        }
    }

    pub fn query(&self, left: usize, right: usize) -> i32 {
        if self.n == 0 {
            return 0;
        }
        self.query_internal(0, 0, self.n - 1, left, right)
    }

    pub fn logical_size(&self) -> usize {
        self.max_index + 1
    }

    fn build(&mut self, node: usize, start: usize, end: usize) {
        self.max_index = self.max_index.max(node);

        if start == end {
            self.tree[node] = self.array[start];
        } else {
            let mid = (start + end) / 2;

            self.build(2 * node + 1, start, mid);
            self.build(2 * node + 2, mid + 1, end);

            self.tree[node] =
                self.tree[2 * node + 1] + self.tree[2 * node + 2];
        }
    }

    fn update_internal(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        idx: usize,
        new_value: i32,
    ) {
        if start == end {
            self.array[idx] = new_value;
            self.tree[node] = new_value;
        } else {
            let mid = (start + end) / 2;

            if idx <= mid {
                self.update_internal(2 * node + 1, start, mid, idx, new_value);
            } else {
                self.update_internal(2 * node + 2, mid + 1, end, idx, new_value);
            }

            self.tree[node] =
                self.tree[2 * node + 1] + self.tree[2 * node + 2];
        }
    }

    fn query_internal(
        &self,
        node: usize,
        start: usize,
        end: usize,
        left: usize,
        right: usize,
    ) -> i32 {
        if right < start || left > end {
            return 0;
        }

        if left <= start && end <= right {
            return self.tree[node];
        }

        let mid = (start + end) / 2;

        self.query_internal(2 * node + 1, start, mid, left, right)
            + self.query_internal(2 * node + 2, mid + 1, end, left, right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_segment_tree() {
        let input = vec![5, 8, 7, 2, 10, 2, 2];
        let expected = vec![36, 22, 14, 13, 9, 12, 2, 5, 8, 7, 2, 10, 2];

        let segment_tree = SegmentTree::new(input.clone());

        let array = segment_tree.get_array();
        assert!(!array.is_empty());
        assert_eq!(&input, array);

        let tree = segment_tree.get_tree();
        assert!(!tree.is_empty());

        let logical_size = segment_tree.logical_size();
        assert_eq!(13, logical_size);

        let actual: Vec<i32> = tree.iter().take(logical_size).cloned().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_create_segment_tree_with_negative_value() {
        let input = vec![0, 1, 3, 5, -2, 3 ];
        let expected = vec![10, 4, 6, 1, 3, 3, 3, 0, 1, 0, 0, 5, -2];

        let segment_tree = SegmentTree::new(input.clone());

        let array = segment_tree.get_array();
        assert!(!array.is_empty());
        assert_eq!(&input, array);

        let tree = segment_tree.get_tree();
        assert!(!tree.is_empty());

        let logical_size = segment_tree.logical_size();
        assert_eq!(13, logical_size);

        let actual: Vec<i32> = tree.iter().take(logical_size).cloned().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_query_segment_tree() {
        let input = vec![5, 8, 7, 2, 10, 2, 2];

        let segment_tree = SegmentTree::new(input);

        assert_eq!(36, segment_tree.query(0, 6));
        assert_eq!(22, segment_tree.query(0, 3));
        assert_eq!(14, segment_tree.query(4, 6));
    }

    #[test]
    fn should_update_segment_tree() {
        let input = vec![5, 8, 7, 2, 10, 2, 2];
        let updated_array = vec![5, 8, 7, 6, 10, 2, 2];
        let expected = vec![40, 26, 14, 13, 13, 12, 2, 5, 8, 7, 6, 10, 2];

        let mut segment_tree = SegmentTree::new(input);
        segment_tree.update(3, 6);

        let array = segment_tree.get_array();
        assert!(!array.is_empty());
        assert_eq!(&updated_array, array);

        let tree = segment_tree.get_tree();
        assert!(!tree.is_empty());

        let logical_size = segment_tree.logical_size();
        assert_eq!(13, logical_size);

        let actual: Vec<i32> = tree.iter().take(logical_size).cloned().collect();
        assert_eq!(expected, actual);
    }
}