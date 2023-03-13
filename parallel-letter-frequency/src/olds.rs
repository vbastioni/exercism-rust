
pub fn frequency2(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let id = Arc::new(Mutex::new((0, HashMap::new())));
    let v: Vec<_> = input.iter().map(|s| s.to_string()).collect();
    let inputs = Arc::new(v);
    let mut handles = vec![];
    for _ in 0..worker_count {
        let cloned = Arc::clone(&inputs);
        let cid = Arc::clone(&id);
        handles.push(std::thread::spawn(move || loop {
            let mut n = cid.lock().unwrap();
            if n.0 >= cloned.len() {
                break;
            }
            for c in (&cloned[n.0]).chars().filter(|c| c.is_alphabetic()) {
                n.1.entry(c.to_ascii_lowercase()).or_insert(0).add_assign(1);
            }
            n.0.add_assign(1);
        }));
    }
    for jh in handles.into_iter() {
        jh.join().unwrap();
    }
    let m = id.lock().unwrap();
    m.1.clone()
}

pub fn frequency3(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut s = HashMap::new();
    if input.len() == 0 {
        return s;
    } else if input.len() == 1 {
        map_chars_mut(input[0], &mut s);
        return s;
    }
    let q = ArrayQueue::new(input.len());
    for i in input {
        q.push(i.to_string()).expect("could not push to ArrayQueue");
    }
    let wg = WaitGroup::new();
    let mh = Arc::new(Mutex::new(s));
    // let aq: ArrayQueue<&str> = queue::ArrayQueue::from(input);
    let arc_q = Arc::new(q);
    let mut handles = vec![];
    for _ in 0..worker_count.clamp(1, input.len()) {
        let q = Arc::clone(&arc_q);
        let wg = wg.clone();
        let mh = Arc::clone(&mh);
        handles.push(std::thread::spawn(move || loop {
            match q.pop() {
                None => {
                    drop(wg);
                    break;
                }
                Some(s) => {
                    let mut mh = mh.lock().expect("could not lock");
                    map_chars_mut(&s, &mut mh);
                }
            }
        }));
    }
    wg.wait();
    let v = mh.lock().unwrap();
    v.to_owned()
}
