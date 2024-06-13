pub struct List<T> {
    head: Link<T>,
    count: u32
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>
}

impl<T: std::marker::Copy + std::cmp::PartialOrd> List<T> {
    pub fn new() -> Self 
    {
        List { head: None, count: 0 }
    }

    pub fn get_count(&self) -> u32
    {
        return self.count;
    }

    pub fn push(&mut self, elem: T) 
    {
        let new_node = Box::new(Node {
            data: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<T> 
    {
        if self.head.is_none()
        {
            return None;
        }
        let node = self.head.take().unwrap();
        self.head = node.next;
        self.count -= 1;

        return Some(node.data);
    }

    pub fn find_and_peek(&mut self, elem: T) -> Option<&mut T>
    {
        let mut curr = self.head.as_mut(); 
        while let Some(node) = curr 
        {
            if node.data == elem
            {
                return Some(&mut node.data);
            }
            curr = node.next.as_mut();
        }

        return None;
    }

    pub fn buble_sort(&mut self) 
    {
        let mut changed: bool = true;

        while changed == true 
        {
            changed = false;
            let mut curr: Option<&mut Box<Node<T>>> = self.head.as_mut();
            while let Some(node) = curr 
            {
                if node.next.is_some() 
                {
                    let next_node: &mut Box<Node<T>> = node.next.as_mut().unwrap();
                    if next_node.data < node.data 
                    {
                        changed = true;
                        let tmp: T = node.data;
                        node.data = next_node.data;
                        next_node.data = tmp;
                    }
                }
                curr = node.next.as_mut();
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) 
    {
        let mut curr: Option<Box<Node<T>>> = self.head.take();
        while let Some(mut node) = curr 
        {
            curr = node.next.take();
        }
    }
}

#[cfg(test)]
mod list_tests;
