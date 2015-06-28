// taken from the implementation of https://github.com/uunicorn/pyskype
pub struct Rc4 {
    s: [u8; 256],
    i: usize,
    j: usize
}

impl Rc4 {
    pub fn new(key: &[u8]) -> Rc4 {
        let mut s = [0; 256];
        for i in 0..256 {s[i] = i as u8;}

        // KSA phase
        let mut j : usize = 0;
        for i in 0..256 {
            let c = key[i % key.len()];
            j = (j + s[i] as usize + c as usize) % 256;
            s.swap(i ,j);
        }

        Rc4 {
            s: s,
            i: 0,
            j: 0
        }
    }

    pub fn crypt(&mut self, data: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(data.len());

        let mut i = self.i; 
        let mut j = self.j; 
        let ref mut s = self.s;
        for byte in data.iter() {
            i = (i + 1) % 256;
            j = (j + s[i] as usize) % 256;
            s.swap(i, j);
            out.push(
                byte ^ s[(s[i] as usize + s[j] as usize) % 256 ]
            )
        }
        self.i = i;
        self.j = j;

        out
    }
}
