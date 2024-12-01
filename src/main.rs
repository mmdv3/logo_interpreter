mod code;
use code::*;

fn main() {
    let input = "repeat 2 [ forward 50 turn 90 ] forward 30";
    let (commands, cmd_env) = parse(input);
    let image_path = "img/output.svg";

    run(commands.into_iter(), cmd_env, image_path);

    println!("SVG file saved to {}", image_path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_angles() {
        let input = "repeat 2 [ forward 50 turn 90 ] forward 30";
        let (commands, cmd_env) = parse(input);
        let image_path = "img/two_angles.svg";
    
        run(commands.into_iter(), cmd_env, image_path);
    
        println!("SVG file saved to {}", image_path);
    }

    #[test]
    fn test_star() {
        let input = "to star repeat 5 [ forward 100 turn 144 ] end star";

        let (commands, cmd_env) = parse(input);
        let image_path = "img/star.svg";

        run(commands.into_iter(), cmd_env, image_path);
    }

    #[test]
    fn test_square() {
        let input = "to square :length repeat 4 [ forward :length turn 90 ] end  repeat 3 [  square 100 turn 10 ]";

        let (commands, cmd_env) = parse(input);
        let image_path = "img/square.svg";

        run(commands.into_iter(), cmd_env, image_path);
    }
}
