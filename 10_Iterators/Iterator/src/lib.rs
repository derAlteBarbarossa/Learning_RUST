#[cfg(test)]
mod tests {

    #[test]
    fn for_loop() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.into_iter(); 

        let mut i = 1;
        for val in v1_iter {
            assert_eq!(val, i);
            i += 1;
        }
    }

    #[test]
    fn iter_demo() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.into_iter();
        assert_eq!(v1_iter.next(), Some(1)); 
        assert_eq!(v1_iter.next(), Some(2));
        assert_eq!(v1_iter.next(), Some(3)); 
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn into_iter_demo() {
        let values = vec![1, 2, 3];

        let numbers: Vec<_> = values
            .into_iter()
            .map(|x| x + 1)
            .collect();

        assert_eq!(numbers[0], 2);
        assert_eq!(numbers[1], 3);
        assert_eq!(numbers[2], 4);

        // into_iter takes ownership, therefore,
        // we can no longer access `values`
        // assert_eq!(values[0], 1);
    }

    #[test]
    fn iter_mut_demo() {
        let mut values = vec![1, 2, 3];

        let numbers: Vec<_> = values
            .iter_mut()
            .map(|x| {*x + 1})
            .collect();

        assert_eq!(numbers[0], 2);
        assert_eq!(numbers[1], 3);
        assert_eq!(numbers[2], 4);

        // iter_mut takes a mutable reference, therefore,
        // we can access `values`
        assert_eq!(values[0], 1);
    }
     
    #[test]
    fn consuming_adaptor_demo() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);

        // We can no longer access v1_iter, as its
        // ownership is moved.
        // let product: i32 = v1_iter.product();
    }

    #[test]
    fn iterator_adaptor_demo() {
        let v1: Vec<i32> = vec![1, 2, 3, 4];
        let v2: Vec<_> = v1.iter().map(|x| x%2).collect();
        assert_eq!(v2, vec![1,0,1,0]);
    }
}
