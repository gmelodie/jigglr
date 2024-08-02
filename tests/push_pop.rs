use jigglr::Queue;

#[test]
fn test_new() {
    let _q: Queue<usize> = Queue::new(3); // should not panic
}

#[test]
fn test_simple_push_pop() {
    let mut q: Queue<usize> = Queue::new(3);
    assert!(q.push(1).is_ok());
    assert_eq!(q.pop(), Some(1));
    assert_eq!(q.pop(), None);
}

#[test]
fn test_push_at_cap() {
    let q: Queue<usize> = Queue::new(3);
    assert!(q.push(1).is_ok());
    assert!(q.push(2).is_ok());
    assert!(q.push(3).is_ok());

    assert!(q.push(4).is_err());
}

#[test]
fn test_pop_none() {
    let mut q: Queue<usize> = Queue::new(3);
    assert!(q.pop().is_none());
}
