fn main() {

let mut имя :usize;

{
    print!("let me know your age");
    io::stdout().flush()?;
    let mut for_read = String::new();
    io::stdin().read_line(&mut for_read)?;
    имя = for_read.trim().parse::<usize>()?;
} 

}
