use std::{io, io::prelude::*};

fn main() {
    let mut matrix = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut row = Vec::new();
        for num in line.split_whitespace() {
            let dig: u32 = num.trim().parse().unwrap();
            row.push(dig);
        }
        matrix.push(row);
    }

    let mut i = 0;
    let mut graph: Vec<Vec<u32>> = Vec::new();
    for row in matrix.iter().rev() {
        let mut j = 0;
        let mut layer: Vec<u32> = Vec::new();
        for num in row.iter() {
            if i == 0 {
                layer.push(*num);
            } else if i == 99 {
                let a = &graph[i-1][j];
                let b = &graph[i-1][j+1];
                if a > b {
                    println!("{}", a + num);
                } else {
                    println!("{}", b + num);
                }
            } else {
                let a = &graph[i-1][j];
                let b = &graph[i-1][j+1];
                if a > b {
                    layer.push(num + a);
                } else {
                    layer.push(num + b);
                }
            }
            j += 1;
        }
        graph.push(layer);
        i += 1;
    }
}
