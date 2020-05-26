//reference: https://stackoverflow.com/questions/30218886/how-to-implement-iterator-and-intoiterator-for-a-simple-struct
struct Pixel {
    r: i8,
    g: i8,
    b: i8,
}

impl IntoIterator for Pixel {
    type Item = i8;
    type IntoIter = PixelIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        PixelIntoIterator {
            pixel: self,
            index: 0,
        }
    }
}

struct PixelIntoIterator {
    pixel: Pixel,
    index: usize,
}

impl Iterator for PixelIntoIterator {
    type Item = i8;
    fn next(&mut self) -> Option<i8> {
        let result = match self.index {
            0 => self.pixel.r,
            1 => self.pixel.g,
            2 => self.pixel.b,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

fn main() {
    let p = Pixel {
        r: 54,
        g: 23,
        b: 74,
    };
    for component in p {//for 自动调用into_iter获得迭代器， 同时p的所有权也被move进PixelIntoIterator了。
        println!("{}", component);
    }
    //后面代码不能再访问p了，因为它已经丧失所有权，失效了。
}