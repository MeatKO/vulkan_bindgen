pub fn vec_two_mut<T>(slice: &mut [T], i1: usize, i2: usize)
-> Option<(&mut T, &mut T)>
{
    if i1 == i2
    {
        return None;
    }

    if i1 >= slice.len() || i2 >= slice.len()
    {
        return None;
    }

    let mid = std::cmp::max(i1, i2);

    let (left, right) = slice.split_at_mut(mid);

    if i1 > i2
    {
        Some((&mut right[0], &mut left[i2]))
    }
    else
    {
        Some((&mut left[i1], &mut right[0]))
    }
}

#[cfg(test)]
mod tests 
{
    use crate::decs::util::vec_two_mut;

    #[test]
    fn vec_two_mut_a_smaller_than_b() 
    {
        let mut test_vec = vec![1, 2, 3, 4, 5];

        let index_a = 0;
        let index_b = 2;

        let (a, b) = vec_two_mut(&mut test_vec,index_a, index_b).unwrap();

        assert_eq!(*a, 1);
        assert_eq!(*b, 3);
    }

    #[test]
    fn vec_two_mut_a_larger_than_b() 
    {
        let mut test_vec = vec![1, 2, 3, 4, 5];

        let index_a = 2;
        let index_b = 0;

        let (a, b) = vec_two_mut(&mut test_vec,index_a, index_b).unwrap();

        assert_eq!(*a, 3);
        assert_eq!(*b, 1);
    }

    #[test]
    fn vec_two_mut_a_equal_to_b() 
    {
        let mut test_vec = vec![1, 2, 3, 4, 5];

        let index_a = 0;
        let index_b = 0;

        let result = vec_two_mut(&mut test_vec,index_a, index_b);

        assert_eq!(result, None);
    }

    #[test]
    fn vec_two_mut_invalid_index() 
    {
        let mut test_vec = vec![1, 2, 3, 4, 5];

        let index_a = 10;

        let result = vec_two_mut(&mut test_vec,index_a, 0);

        assert_eq!(result, None);

        let index_b = 10;

        let result = vec_two_mut(&mut test_vec,0, index_b);

        assert_eq!(result, None);
    }
}