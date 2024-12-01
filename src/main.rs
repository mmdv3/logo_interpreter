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
        let input = "to star\n
        repeat 5 [ forward 100 turn 144 ]\n
        end\n
        star";

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

    #[test]
    fn test_tree_simplified() { // normalnie if size < 5
        let input = "to tree :size\n
   if :size < 100 [forward :size back :size stop]\n
   forward :size/3\n
   tree :size/2\n
end\n
tree 150"; // any amount of recurrence causes test overflow

let (commands, cmd_env) = parse(input);
let image_path = "img/tree_simplified.svg";

run(commands.into_iter(), cmd_env, image_path);
    }

    #[test]
    fn test_tree() { // normalnie if size < 5
        let input = "to tree :size\n
   if :size < 5 [forward :size back :size stop]\n
   forward :size/3\n
   left 30 tree :size*2/3 right 30\n
   forward :size/6\n
   right 25 tree :size/2 left 25\n
   forward :size/3\n
   right 25 tree :size/2 left 25\n
   forward :size/6\n
   back :size\n
end\n
tree 150";

let (commands, cmd_env) = parse(input);
let image_path = "img/tree.svg";

run(commands.into_iter(), cmd_env, image_path);
    }
}



