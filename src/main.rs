use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug,Clone)]
struct Node {
    next: Vec<Link>,
    pub offset: u64,
    pub command: String,
}

#[derive(Debug,Clone)]
pub struct BestTransactionLog {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64,
}

impl Node {

    fn new(next: Vec<Link>, offset: u64, command: String) -> Rc<RefCell<Node>> {
        Rc::new(
            RefCell::new(
                Node { next: next, offset: offset, command: command}
            )
        )
    }
}

impl BestTransactionLog {

    pub fn new_empty() -> BestTransactionLog {
        BestTransactionLog {            
            head: None,
            tails: vec![None],
            max_level: 0,
            length: 0,
        }
    }

    fn get_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }

    pub fn append(&mut self, offset: u64, value: String) {
        let level = 1 + if self.head.is_none(){
            self.max_level
        } else {
            self.get_level()
        };

        let new = Node::new(vec![None; level], offset, value);

        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(new.clone());
            }
            self.tails[i] = Some(new.clone());
        }

        if self.head.is_none() {
            self.head = Some(new.clone());
        }
        self.length += 1;
    }

    pub fn find(&self, offset: u64) -> Option<String> {
        match self.head {
            Some(ref head) => {
                let mut start_level = self.max_level;
                let node = head.clone();
                let mut result = None;
                loop {
                    if node.borrow().next[start_level].is_some() {
                        break;
                    }
                    start_level -= 1;
                }
                let mut n = node;
                for level in (0..=start_level).rev() {
                    loop {
                        let next = n.clone();
                        match next.borrow().next[level] {
                            Some(ref next) if next.borrow().offset <= offset => n = next.clone(),
                            _ => break
                        };
                    }
                    if n.borrow().offset == offset {
                        let tmp = n.borrow();
                        result = Some(tmp.command.clone());
                        break;
                    }
                }
                result
            },
            None => None
        }
    }
}


fn main() {
    let mut transaction_log = BestTransactionLog::new_empty();
    transaction_log.append(0,String::from("First"));
    println!("Append:{:#?}",transaction_log);
    transaction_log.append(1,String::from("Second"));
    println!("Append:{:#?}",transaction_log);
    transaction_log.append(2,String::from("Third"));
    println!("Append:{:#?}",transaction_log);
    println!("Find:{:#?}",transaction_log.find(1));
}
