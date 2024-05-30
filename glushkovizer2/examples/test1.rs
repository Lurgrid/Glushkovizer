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
    let reg = dbg!(RegExp::try_from("(a+b).a*.b*.(a+b)*")).unwrap();
    let auto = dbg!(Automata::from(reg));
    dbg!(auto.accept(['a', 'b'].iter()));
    dbg!(auto.dfs(vec![0, 1, 2, 3, 4, 5, 6], false));
    dbg!(auto.dfs(vec![6, 5, 4, 3, 1, 2, 0], false));
    dbg!(auto.kosaraju_type());
    let reg = dbg!(RegExp::try_from("(a.b.c.d)*")).unwrap();
    let auto = dbg!(Automata::from(reg));
    println!("{}", auto.to_dot().unwrap());
    let s1_auto = dbg!(auto.subautomata(vec![&0, &1], vec![&0], vec![&1]).unwrap());
    drop(auto);
    println!("{}", s1_auto.to_dot().unwrap());
    dbg!(auto.remove_state(&1));
    println!("{}", s1_auto.to_dot().unwrap());
    drop(s1_auto);
    println!("{}", auto.to_dot().unwrap());
}
