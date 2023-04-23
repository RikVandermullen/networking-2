use std::{net::{TcpListener, TcpStream}, io::Write, thread};
use std::io::Read;

static mut GRID: [[i32; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
static mut CLIENTS: Vec<TcpStream> = Vec::new();

fn handle_client(mut stream: &TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());

    let mut buffer = [0; 1024];
    while let Ok(n) = stream.read(&mut buffer) {
        if n == 0 {
            break;
        }

        println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

        let input = String::from_utf8_lossy(&buffer[..n]);
        let turn: Vec<_> = input.split("-").collect();
        let player: i32 = turn[2].replace("\r\n", "").as_str().parse().unwrap();

        player_turn(turn[0].parse().unwrap(), turn[1].parse().unwrap(), player, unsafe { &mut GRID });

        send_data(stream);

        // unsafe { CLIENTS.iter().for_each(|stream| {
        //     send_data(stream);
        //     println!("Sending data to {}", stream.peer_addr().unwrap());
        // }) };

    }
}

pub fn main() -> std::io::Result<()> {
    println!("Starting listener...");
    let listener = TcpListener::bind("127.0.0.1:12345").expect("Failed to bind to address");
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");
        unsafe { CLIENTS.push(stream) };
        if unsafe { CLIENTS.len() == 2 } {
            unsafe { CLIENTS.iter().for_each(|stream| {
                thread::spawn(move || {
                    handle_client(&stream);
                });
            }) };
        }
    }

    Ok(())
}

fn send_data(mut stream: &TcpStream) {
    let mut grid_string = String::new();
    for row in unsafe { &GRID } {
        for col in row {
            grid_string.push_str(&col.to_string());
            grid_string.push_str(" ");
        }
        grid_string.push_str("\r\n");
    }
    grid_string.pop();
    grid_string.push_str("\r\n");
    stream.write(grid_string.as_bytes()).expect("Failed to write to stream");
}

fn player_turn(x: usize, y: usize, player: i32, grid: &mut [[i32; 3]; 3]) {
    if x > 2 || y > 2 {
        println!("Invalid coordinates");
        return;
    }

    if grid[x][y] != 0 {
        println!("This space is already taken");
        return;
    }

    grid[x][y] = player;

    println!("Player {} played at ({}, {})", player, x, y);

    let winner = check_win(&grid);
        if winner {
            println!("Player {} won!", player);
            
        } else {
            println!("No winner");
        }

    print_grid(&grid);
}

fn print_grid(grid: &[[i32; 3]; 3]) {
    for row in grid {
        for col in row {
            print!("{} ", col);
        }
        println!();
    }
}

fn check_win(grid: &[[i32; 3]; 3]) -> bool {
    for row in grid {
        if row[0] == row[1] && row[1] == row[2] && row[0] != 0 {
            return true;
        }
    }

    for i in 0..3 {
        if grid[0][i] == grid[1][i] && grid[1][i] == grid[2][i] && grid[0][i] != 0 {
            return true;
        }
    }

    if grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] && grid[0][0] != 0 {
        return true;
    }

    if grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] && grid[0][2] != 0 {
        return true;
    }

    return false;
}
    