mod interpreter;
use interpreter::*;

fn main() {
    let input = "repeat 2 [ forward 50 turn 90 ] forward 30";
    let image_path = "img/output.svg";

    run(input, image_path);
    println!("SVG file saved to {}", image_path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_angles() {
        let input = "repeat 2 [ forward 50 turn 90 ] forward 30";
        let image_path = "img/two_angles.svg";

        run(input, image_path);
    }

    #[test]
    fn test_star() {
        let input = "to star\n
        repeat 5 [ forward 100 turn 144 ]\n
        end\n
        star";
        let image_path = "img/star.svg";

        run(input, image_path);
    }

    #[test]
    fn test_square() {
        let input = "to square :length repeat 4 [ forward :length turn 90 ] end\n
            repeat 36 [  square 100 turn 10 ]";
        let image_path = "img/square.svg";

        run(input, image_path);
    }

    #[test]
    fn test_tree() {
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
        let image_path = "img/tree.svg";

        run(input, image_path);
    }

    #[test]
    fn test_fern() {
        let input = "to fern :size :sign\n
            if :size < 1 [ stop ]\n
            forward :size\n
            turn 70 * :sign fern :size * 0.5 :sign * -1 left 70 * :sign\n
            forward :size\n
            left 70 * :sign fern :size * 0.5 :sign right 70 * :sign\n
            right 7 * :sign fern :size - 1 :sign left 7 * :sign\n
            back :size * 2\n
            end\n
            fern 25 1";
        let image_path = "img/fern.svg";

        run(input, image_path);
    }
}
