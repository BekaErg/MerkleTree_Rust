use super::*;

impl treenode::TreeNode {
    fn depth(&self) -> usize {
        let mut ans = 0;
        for &next in [&self.left, &self.right].iter() {
            if let Some(x) = next {
                ans = std::cmp::max(ans, x.borrow().depth());
            }
        }
        ans + 1
    }

    fn vectorize(&self) -> Vec<Vec<Option<i32>>> {
        self.vectorize_with_depth(self.depth())
    }

    fn vectorize_with_depth(&self, depth: usize) -> Vec<Vec<Option<i32>>> {
        let mut ans = vec![vec![Some(1i32)]];
        if depth == 1 {
            return ans;
        }

        let mut lr: Vec<Vec<Vec<Option<i32>>>> = vec![vec![], vec![]];

        for (i, &next) in [&self.left, &self.right].iter().enumerate() {
            if let Some(ref node) = next {
                lr[i].extend(node.borrow().vectorize_with_depth(depth - 1));
            } else {
                for j in 0..depth - 1 {
                    lr[i].push(vec![None; 2_usize.pow(j as u32)])
                }
            }
        }
        let (r, l) = (lr.pop().unwrap(), lr.pop().unwrap());
        ans.extend(merge(l, r));
        ans
    }
}

impl MerkleTree {
    pub fn display(self) {
        let v = self.root.unwrap().borrow().vectorize();
        for i in 0..v.len() {
            println!("{:?}", v[i]);
        }
    }
}

#[cfg(test)]
fn same_structure_hashnodes(
    option_node1: &Option<Rc<RefCell<TreeNode>>>,
    option_node2: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if let (Some(ref node1), Some(ref node2)) = (&option_node1, &option_node2) {
        same_structure_hashnodes(&node1.borrow().left, &node2.borrow().left)
            && same_structure_hashnodes(&node1.borrow().right, &node2.borrow().right)
    } else if let Some(_) = option_node1 {
        false
    } else if let Some(_) = option_node2 {
        false
    } else {
        true
    }
}
#[cfg(test)]
pub fn same_structure(tree1: &MerkleTree, tree2: &MerkleTree) -> bool {
    same_structure_hashnodes(&tree1.root, &tree2.root)
}

fn merge<T: Copy>(x: Vec<Vec<T>>, y: Vec<Vec<T>>) -> Vec<Vec<T>> {
    x.into_iter()
        .zip(y.into_iter())
        .map(|(a, b)| [a, b].concat())
        .collect::<Vec<_>>()
}
