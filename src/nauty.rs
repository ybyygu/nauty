// [[file:../nauty.note::*imports][imports:1]]
use crate::*;

use anyhow::*;
// imports:1 ends here

// [[file:../nauty.note::*pub][pub:1]]
/// The canonical label is given in the form of a list of the vertices of g in
/// canonical order.
///
/// # Parameters
///
/// * nodes: a list of graph nodes (vertices)
/// * edges: a list of edges in pair of nodes
/// * colors: The node colors listed as in order of `nodes`.
///
/// # Panics
///
/// * Will panic if any invalid node in edges or colors
///
pub fn get_canonical_labels<T, M>(nodes: &[T], edges: &[(T, T)], colors: &[M]) -> Result<Vec<T>>
where
    T: Copy + std::fmt::Debug + std::hash::Hash + std::cmp::Eq,
    M: Copy + Ord + std::fmt::Debug,
{
    use std::collections::HashMap;

    let nnodes = nodes.len();
    let nedges = edges.len();
    assert_eq!(nnodes, colors.len(), "invalid colors: {:?}", colors);

    // index => node
    let map: HashMap<_, _> = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n, i as i32))
        .collect();
    assert_eq!(map.len(), nnodes, "invalid nodes: {:?}", nodes);

    let mut edges_i = vec![];
    let mut edges_j = vec![];
    for (ni, nj) in edges {
        let i = map[&ni];
        let j = map[&nj];
        if i < j {
            edges_i.push(i);
            edges_j.push(j);
        } else {
            edges_i.push(j);
            edges_j.push(i);
        }
    }

    // prepare labels and partitions in nauty's style

    // Notes in nauty doc:
    //
    // In this case, lab should contain a list of all the vertices in some order
    // such that vertices with the same colour are contiguous. The ends of the
    // colour-classes are indicated by zeros in ptn

    // labels
    let mut lab: Vec<_> = (0..nnodes).map(|i| i as i32).collect();
    lab.sort_by_key(|&i| colors[i as usize]);

    // partitions
    let mut ptn = vec![0; nnodes];
    for (i, p) in lab.windows(2).enumerate() {
        let a0 = p[0] as usize;
        let a1 = p[1] as usize;
        if colors[a0] == colors[a1] {
            ptn[i] = 1;
        } else {
            ptn[i] = 0;
        }
    }

    // external call of nauty for canonical labelling
    unsafe {
        let x = dwim(
            nnodes as i32,
            nedges as i32,
            edges_i.as_mut_ptr(),
            edges_j.as_mut_ptr(),
            lab.as_mut_ptr(),
            ptn.as_mut_ptr(),
        );
        if x != 0 {
            bail!("nauty failure: {:?}", (edges_i, edges_j, lab, ptn));
        }
    }

    // mapping back to origin nodes instead of indices
    let labels = lab.into_iter().map(|i| nodes[i as usize]).collect();
    Ok(labels)
}
// pub:1 ends here

// [[file:../nauty.note::*test][test:1]]
#[test]
fn test_nauty_labels() {
    
}
// test:1 ends here
