mod problems;
mod my_linked_list;

use std::{fs::OpenOptions, io::{stdin, stdout, Read, Seek, SeekFrom, Write}};

use crossterm::{cursor::MoveTo, execute, terminal::{Clear, ClearType}};
use problems::Problem;

fn main() {
    let mut file = OpenOptions::new().create(true).read(true).write(true).open("recent").unwrap();
    
    let mut recent:String = String::new();
    println!("{}", &recent);
    file.read_to_string(&mut recent).unwrap();
    println!("{}", &recent);
    let recent:usize = recent.trim().parse().unwrap_or(0);
    println!("{}",&recent);

    let mut input = String::new();
    let mut num:usize = 0;

    execute!(stdout(), Clear(ClearType::All),MoveTo(0,0)).unwrap();
    print!("지구인128의 백준풀이!\n\n실행을 위해 문제 번호를 입력하십시오. ");
    if (recent != 0) {
        print!("(미입력시 최근 문제 실행)");
    }
    print!("\n");
    stdin().read_line(&mut input).unwrap();
    let res:usize = input.trim().parse().unwrap_or(0);
    if (res > 0) {
        num = res;
        file.set_len(0).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        writeln!(file,"{}", num).unwrap();
    }else {
        if (recent > 0) {
            println!("최근 문제 실행");
            num = recent;
        }else {
            println!("최근 문제 없음 -> 종료");
            return;
        }
    }

    let map = Problem::problem_map();
    let res = map.get(&num);

    println!("문제 보기 : https://www.acmicpc.net/problem/{num}");
    match res {
        Some(p) => p.execute(),
        None => println!("문제가 데이터에 없습니다!"),
    }
}
