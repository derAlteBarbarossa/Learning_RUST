#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.into_iter();
        assert_eq!(v1_iter.next(), Some(v1[0])); 
        assert_eq!(v1_iter.next(), Some(v1[1])); 
        assert_eq!(v1_iter.next(), Some(v1[2])); 
        assert_eq!(v1_iter.next(), None);
    }
        
}
