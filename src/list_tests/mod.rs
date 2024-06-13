use super::List;
use std::cmp::*;

#[test]
fn basics() 
{
    let mut list: List<u32> = List::new();

    assert_eq!(list.pop(), None);

    assert_eq!(list.get_count(), 0);

    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.get_count(), 3);

    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    list.push(4);
    list.push(5);

    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);

    assert_eq!(list.get_count(), 0);
}

#[test]
fn sort()
{
    let mut list: List<u32> = List::new();

    list.push(3);
    list.push(1);
    list.push(5);
    list.push(4);
    list.push(2);

    list.buble_sort();

    let mut val = list.pop();
    assert_eq!(val, Some(1));
    val = list.pop();
    assert_eq!(val, Some(2));
    val = list.pop();
    assert_eq!(val, Some(3));
    val = list.pop();
    assert_eq!(val, Some(4));
    val = list.pop();
    assert_eq!(val, Some(5));
    val = list.pop();
    assert_eq!(val, None);
}

#[test]
fn custom_struct()
{
    #[derive(Copy, Clone)]
    struct UserNode<'a> {
        key: u32,
        data_bin: u64,
        data_str: &'a str, // WTF is 'a lifetime parameter
    }

    impl PartialOrd for UserNode<'_> 
    {
        fn partial_cmp(&self, other: &UserNode) -> Option<std::cmp::Ordering> 
        {
            if self.key == other.key 
            {
                return Some(Ordering::Equal);
            } 
            else if self.key < other.key 
            {
                return Some(Ordering::Less);
            } 
            else 
            {
                return Some(Ordering::Greater);
            }
        }
    }

    impl PartialEq for UserNode<'_> {
        fn eq(&self, other: &UserNode) -> bool 
        {
            return self.key == other.key;
        }
    }

    let mut list: List<UserNode> = List::new();

    let mut node = UserNode {
        key: 3,
        data_bin: 1020,
        data_str: "qwerty",
    };
    list.push(node);
    node.key = 1;
    node.data_bin = 100;
    node.data_str = "asdfgh";
    list.push(node);
    node.key = 2;
    node.data_bin = 500;
    node.data_str = "zxcvbn";
    list.push(node);

    node.key = 2;
    node.data_bin = 0;
    node.data_str = "";
    list.buble_sort();

    let res_mut = list.find_and_peek(node);
    if res_mut.is_some()
    {
        let value = res_mut.unwrap();
        value.data_bin = 400;
        value.data_str = "aaaaa";
    }
    let mut res = list.pop();
    assert_eq!(res.unwrap().key, 1);
    assert_eq!(res.unwrap().data_bin, 100);
    assert_eq!(res.unwrap().data_str, "asdfgh");
    res = list.pop();
    assert_eq!(res.unwrap().key, 2);
    assert_eq!(res.unwrap().data_bin, 400);
    assert_eq!(res.unwrap().data_str, "aaaaa");
    res = list.pop();
    assert_eq!(res.unwrap().key, 3);
    assert_eq!(res.unwrap().data_bin, 1020);
    assert_eq!(res.unwrap().data_str, "qwerty");
    res = list.pop();
    assert!(res == None);
    }