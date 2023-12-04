use std::fs;

static RED_CAP: u8 = 12;
static GREEN_CAP: u8 = 13;
static BLUE_CAP: u8 = 14;

struct Game {
    id: u16,
    is_valid: bool
}

struct Game2 {
    id: u16,
    least_red: u16,
    least_green: u16,
    least_blue: u16
}

struct Round {
    red: u8,
    blue: u8,
    green: u8
}

fn check_rounds(rounds: Vec<Round>) -> bool{
    for round in rounds
    {
        if round.red > RED_CAP
        {
            return false;
        }
        if round.green > GREEN_CAP
        {
            return false;
        }
        if round.blue > BLUE_CAP
        {
            return false;
        }
    }
    true
}

fn part1(){
    //let path_test1 = "./test1_data.txt";
    let path_part1 = "./part1_data.txt";
    let lines = fs::read_to_string(path_part1).unwrap();

    //PREPARE DATA
    let mut games = Vec::<Game>::new();

    for line in lines.lines()
    {
        let splitted: Vec<&str> = line.split_terminator(':').collect();
        let game_id = splitted[0].split_terminator(' ').collect::<Vec<_>>()[1].parse::<u16>().unwrap();
        
        let mut game = Game {
            id: game_id,
            is_valid: false
        };

        let mut rounds = Vec::<Round>::new();
        let rounds_str: Vec<&str> = splitted[1].split_terminator(';').collect::<Vec<_>>();
        
        for round_str in rounds_str
        {
            let blocks_str = round_str.split_terminator(',').collect::<Vec<_>>();
            let mut round = Round{
                red: 0,
                blue: 0,
                green: 0
            };

            for block in blocks_str
            {
                let round_parts = block.split_terminator(' ').collect::<Vec<_>>();
                let num = round_parts[1].parse::<u8>().unwrap();
                let color = round_parts[2];

                match color {
                    "red"=>round.red=num,
                    "blue"=>round.blue=num,
                    "green"=>round.green=num,
                    _=>println!("Something went wrong!")
                }
                //println!("Num: {num} Color: {color}\n");
            }
            rounds.push(round);
        }
        if check_rounds(rounds) {
            game.is_valid=true;
        }
        games.push(game);
    }

    let mut sum: u16 = 0;

    for g in games
    {
        if g.is_valid
        {
            sum += g.id;
            //println!("Game ID {0} is valid", g.id);
        }
    }
    println!("Sum of game ID's: {sum}");
}

fn find_least(rounds: Vec<Round>, mut game: Game2)->Game2{
    for round in rounds
    {
        if round.red as u16 > game.least_red
        {
            game.least_red = round.red as u16;
        }
        if round.green as u16 > game.least_green
        {
            game.least_green = round.green as u16;
        }
        if round.blue as u16 > game.least_blue
        {
            game.least_blue = round.blue as u16;
        }
    }
    game
}

fn part2(){
    //let path_test2 = "./test2_data.txt";
    let path_part2 = "./part2_data.txt";
    let lines = fs::read_to_string(path_part2).unwrap();

    //PREPARE DATA
    let mut games = Vec::<Game2>::new();

    for line in lines.lines()
    {
        let splitted: Vec<&str> = line.split_terminator(':').collect();
        let game_id = splitted[0].split_terminator(' ').collect::<Vec<_>>()[1].parse::<u16>().unwrap();
        
        let mut game = Game2 {
            id: game_id,
            least_red: 0,
            least_blue: 0,
            least_green: 0
        };

        let mut rounds = Vec::<Round>::new();
        let rounds_str: Vec<&str> = splitted[1].split_terminator(';').collect::<Vec<_>>();
        
        for round_str in rounds_str
        {
            let blocks_str = round_str.split_terminator(',').collect::<Vec<_>>();
            let mut round = Round{
                red: 0,
                blue: 0,
                green: 0
            };

            for block in blocks_str
            {
                let round_parts = block.split_terminator(' ').collect::<Vec<_>>();
                let num = round_parts[1].parse::<u8>().unwrap();
                let color = round_parts[2];

                match color {
                    "red"=>round.red=num,
                    "blue"=>round.blue=num,
                    "green"=>round.green=num,
                    _=>println!("Something went wrong!")
                }
                //println!("Num: {num} Color: {color}\n");
            }
            rounds.push(round);
        }
        game = find_least(rounds, game);
        //println!("Game ID: {0}, red: {1}, green: {2}, blue: {3}",game.id, game.least_red, game.least_green, game.least_blue);
        games.push(game);
    }

    let mut sum: u16 = 0;

    for g in games
    {
        sum += g.least_blue * g.least_green * g.least_red;
        println!("Multiplication of balls from game {0}: {1}", g.id, g.least_blue * g.least_green * g.least_red);
    }
    println!("Sum of multiplications: {sum}");
}

fn main() {
    part1();
    part2();
}