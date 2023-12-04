use std::fs;

fn part1()
{
    let mut sum = 0;

    //let path_test1 = "./test1_data.txt";
    let path_part1 = "./part1_data.txt";

    for line in fs::read_to_string(path_part1).unwrap().lines()
    {
        let mut num = String::from("");
        for ch in line.to_string().chars()
        {
            if ch.is_numeric()
            {
                num.push(ch);
                break;
            }
        }
        for ch in line.to_string().chars().rev()
        {
            if ch.is_numeric()
            {
                num.push(ch);
                break;
            }
        }
        sum += num.parse::<i32>().unwrap();
        //println!("Object value: {num}");
    }
    println!("Sum of objects in part1: {sum}");
}

fn change_string_to_number(string: String, nums: [&str;9]) -> i32
{
    for i in 1..=9
    {
        if string == *nums[i-1]
        {
            return i.try_into().unwrap();
        }
    }
    0
}

fn part2()
{
    let mut sum = 0;
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    //let path_test2 = "./test2_data.txt";
    let path_part2 = "./part2_data.txt";

    let lines = fs::read_to_string(path_part2).unwrap();
    
    for line in lines.lines()
    {
        let mut num_table: Vec<char> = Vec::new();
        for i in 0..=line.chars().count()-1
        {
            let character = line.chars().collect::<Vec<_>>()[i];
            if character.is_numeric()
            {
                num_table.push(character);
                continue;
            }
            for j in i..=line.chars().count()
            {
                let char_table = &line.chars().collect::<Vec<_>>()[i..j];
                let str_from_table: String = char_table.iter().collect();
                let changed = change_string_to_number(str_from_table, numbers);
                if changed != 0
                {
                    num_table.push((changed as u8 + b'0') as char);
                }
            }
        }
        let mut num = String::from("");
        num.push(*num_table.first().unwrap());
        num.push(*num_table.last().unwrap());
        sum += num.parse::<i32>().unwrap();
        //println!("Object value: {num}");
    }

    println!("Sum of objects in part2: {sum}");
}

fn main() 
{
    part1();
    part2();
}