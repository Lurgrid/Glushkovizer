use glushkovizer::automata::Automata;
use glushkovizer::prelude::*;
use glushkovizer::regexp::RegExp;

#[allow(unused_must_use)]

fn main() {
    let a: Automata<usize, char> = Automata::new();
    dbg!(a.add_state('a'));
    dbg!(a.add_initial(&'a'));
    dbg!(a.add_final(&'b'));
    dbg!(a.add_state('b'));
    dbg!(a.get_follow_count(&'a', &2usize));
    dbg!(&a);
    dbg!(a.remove_state(&'a'));
    dbg!(&a);
    dbg!(a.add_transition(&'b', &'b', 1usize));
    dbg!(a.add_transition(&'b', &'b', 1usize));
    dbg!(a.remove_transition(&'b', &'b', &1usize));
    dbg!(&a);
    dbg!(a.cloned());
    dbg!(a.add_initial(&'b'));
    dbg!(a.mirror());
    dbg!(a.dfs(vec!['b'], false));
    let reg = RegExp::try_from("(a+b).a*.b*.(a+b)*").unwrap();
    let auto = Automata::from(reg);
    dbg!(auto.accept(['a', 'b'].iter()));
    dbg!(auto.dfs(vec![0, 1, 2, 3, 4, 5, 6], false));
    dbg!(auto.dfs(vec![6, 5, 4, 3, 1, 2, 0], false));
    dbg!(auto.kosaraju_type());
    let reg = RegExp::try_from("a.(a.b.c.d)*").unwrap();
    let auto = Automata::from(reg);
    println!("{}", auto.to_dot(true).unwrap());
    let s1_auto = auto.subautomata(vec![&0, &1], vec![&0], vec![&1]).unwrap();
    println!("{}", s1_auto.to_dot(true).unwrap());
    auto.remove_state(&1);
    println!("{}", s1_auto.to_dot(true).unwrap());
    let serialized = serde_json::to_string(&s1_auto).unwrap();
    println!("Serialized: {}", serialized);
    drop(s1_auto);
    println!("{}", auto.to_dot(true).unwrap());
    let reg = RegExp::try_from("(a+b).a*.b*.(a+b)*").unwrap();
    let auto = Automata::from(reg);
    let serialized = serde_json::to_string(&auto).unwrap();
    println!("Serialized: {}", serialized);
    let auto_copy: Automata<char, usize> = serde_json::from_str(
        r#"{"states":[3,2,1,0,4,5,6],"inputs":[0],"outputs":[6,3,1,2,4,5],"follows":[[3,"a",5],[3,"a",3],[3,"b",4],[3,"b",6],[2,"a",3],[2,"a",5],[2,"b",6],[2,"b",4],[1,"b",6],[1,"b",4],[1,"a",5],[1,"a",3],[0,"b",2],[0,"a",1],[4,"a",5],[4,"b",4],[4,"b",6],[5,"b",6],[5,"a",5],[6,"a",5],[6,"b",6]]}"#,
    ).unwrap();
    println!("{}", auto_copy.to_dot(true).unwrap());
}
