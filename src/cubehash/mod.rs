use byteorder::{ByteOrder, LittleEndian};

const IV: [u32; 32] = [
    0xEA2B_D4B4,
    0xCCD6_F29F,
    0x6311_7E71,
    0x3548_1EAE,
    0x2251_2D5B,
    0xE5D9_4E63,
    0x7E62_4131,
    0xF4CC_12BE,
    0xC2D0_B696,
    0x42AF_2070,
    0xD072_0C35,
    0x3361_DA8C,
    0x28CC_ECA4,
    0x8EF8_AD83,
    0x4680_AC00,
    0x40E5_FBAB,
    0xD890_41C3,
    0x6107_FBD5,
    0x6C85_9D41,
    0xF0B2_6679,
    0x0939_2549,
    0x5FA2_5603,
    0x65C8_92FD,
    0x93CB_6285,
    0x2AF2_B5AE,
    0x9E4B_4E60,
    0x774A_BFDD,
    0x8525_4725,
    0x1581_5AEB,
    0x4AB6_AAD6,
    0x9CDA_F8AF,
    0xD603_2C0A,
];

struct CubeHash {
    x0: u32,
    x1: u32,
    x2: u32,
    x3: u32,
    x4: u32,
    x5: u32,
    x6: u32,
    x7: u32,
    x8: u32,
    x9: u32,
    xa: u32,
    xb: u32,
    xc: u32,
    xd: u32,
    xe: u32,
    xf: u32,
    xg: u32,
    xh: u32,
    xi: u32,
    xj: u32,
    xk: u32,
    xl: u32,
    xm: u32,
    xn: u32,
    xo: u32,
    xp: u32,
    xq: u32,
    xr: u32,
    xs: u32,
    xt: u32,
    xu: u32,
    xv: u32,
}

fn new_cube_hash() -> CubeHash {
    let c = CubeHash {
        x0: IV[0],
        x1: IV[1],
        x2: IV[2],
        x3: IV[3],
        x4: IV[4],
        x5: IV[5],
        x6: IV[6],
        x7: IV[7],
        x8: IV[8],
        x9: IV[9],
        xa: IV[10],
        xb: IV[11],
        xc: IV[12],
        xd: IV[13],
        xe: IV[14],
        xf: IV[15],
        xg: IV[16],
        xh: IV[17],
        xi: IV[18],
        xj: IV[19],
        xk: IV[20],
        xl: IV[21],
        xm: IV[22],
        xn: IV[23],
        xo: IV[24],
        xp: IV[25],
        xq: IV[26],
        xr: IV[27],
        xs: IV[28],
        xt: IV[29],
        xu: IV[30],
        xv: IV[31],
    };
    return c;
}

fn input_block(data: Vec<u8>, mut c: CubeHash) -> CubeHash {
    let (_, mut _right) = &data.split_at(0);
    c.x0 ^= LittleEndian::read_u32(_right);
    let (_, mut _right) = &data.split_at(4);
    c.x1 ^= LittleEndian::read_u32(_right);
    let (_, mut _right) = &data.split_at(8);
    c.x2 ^= LittleEndian::read_u32(_right);
    let (_, mut _right) = &data.split_at(12);
    c.x3 ^= LittleEndian::read_u32(_right);
    let (_, mut _right) = &data.split_at(16);
    c.x4 ^= LittleEndian::read_u32(_right);
    let (_, mut _right) = &data.split_at(20);
    c.x5 ^= LittleEndian::read_u32(_right);
    let (_, mut _right) = &data.split_at(24);
    c.x6 ^= LittleEndian::read_u32(_right);
    let (_, mut _right) = &data.split_at(28);
    c.x7 ^= LittleEndian::read_u32(_right);
    return c;
}

fn sixteen_rounds(mut c: CubeHash) -> CubeHash {
    let mut _i = 0;
    for _i in 0..8 {
        c.xg = c.x0.wrapping_add(c.xg);
        c.x0 = (c.x0 << 7) | (c.x0 >> (32 - 7));
        c.xh = c.x1.wrapping_add(c.xh);
        c.x1 = (c.x1 << 7) | (c.x1 >> (32 - 7));
        c.xi = c.x2.wrapping_add(c.xi);
        c.x2 = (c.x2 << 7) | (c.x2 >> (32 - 7));
        c.xj = c.x3.wrapping_add(c.xj);
        c.x3 = (c.x3 << 7) | (c.x3 >> (32 - 7));
        c.xk = c.x4.wrapping_add(c.xk);
        c.x4 = (c.x4 << 7) | (c.x4 >> (32 - 7));
        c.xl = c.x5.wrapping_add(c.xl);
        c.x5 = (c.x5 << 7) | (c.x5 >> (32 - 7));
        c.xm = c.x6.wrapping_add(c.xm);
        c.x6 = (c.x6 << 7) | (c.x6 >> (32 - 7));
        c.xn = c.x7.wrapping_add(c.xn);
        c.x7 = (c.x7 << 7) | (c.x7 >> (32 - 7));
        c.xo = c.x8.wrapping_add(c.xo);
        c.x8 = (c.x8 << 7) | (c.x8 >> (32 - 7));
        c.xp = c.x9.wrapping_add(c.xp);
        c.x9 = (c.x9 << 7) | (c.x9 >> (32 - 7));
        c.xq = c.xa.wrapping_add(c.xq);
        c.xa = (c.xa << 7) | (c.xa >> (32 - 7));
        c.xr = c.xb.wrapping_add(c.xr);
        c.xb = (c.xb << 7) | (c.xb >> (32 - 7));
        c.xs = c.xc.wrapping_add(c.xs);
        c.xc = (c.xc << 7) | (c.xc >> (32 - 7));
        c.xt = c.xd.wrapping_add(c.xt);
        c.xd = (c.xd << 7) | (c.xd >> (32 - 7));
        c.xu = c.xe.wrapping_add(c.xu);
        c.xe = (c.xe << 7) | (c.xe >> (32 - 7));
        c.xv = c.xf.wrapping_add(c.xv);
        c.xf = (c.xf << 7) | (c.xf >> (32 - 7));
        c.x8 ^= c.xg;
        c.x9 ^= c.xh;
        c.xa ^= c.xi;
        c.xb ^= c.xj;
        c.xc ^= c.xk;
        c.xd ^= c.xl;
        c.xe ^= c.xm;
        c.xf ^= c.xn;
        c.x0 ^= c.xo;
        c.x1 ^= c.xp;
        c.x2 ^= c.xq;
        c.x3 ^= c.xr;
        c.x4 ^= c.xs;
        c.x5 ^= c.xt;
        c.x6 ^= c.xu;
        c.x7 ^= c.xv;
        c.xi = c.x8.wrapping_add(c.xi);
        c.x8 = (c.x8 << 11) | (c.x8 >> (32 - 11));
        c.xj = c.x9.wrapping_add(c.xj);
        c.x9 = (c.x9 << 11) | (c.x9 >> (32 - 11));
        c.xg = c.xa.wrapping_add(c.xg);
        c.xa = (c.xa << 11) | (c.xa >> (32 - 11));
        c.xh = c.xb.wrapping_add(c.xh);
        c.xb = (c.xb << 11) | (c.xb >> (32 - 11));
        c.xm = c.xc.wrapping_add(c.xm);
        c.xc = (c.xc << 11) | (c.xc >> (32 - 11));
        c.xn = c.xd.wrapping_add(c.xn);
        c.xd = (c.xd << 11) | (c.xd >> (32 - 11));
        c.xk = c.xe.wrapping_add(c.xk);
        c.xe = (c.xe << 11) | (c.xe >> (32 - 11));
        c.xl = c.xf.wrapping_add(c.xl);
        c.xf = (c.xf << 11) | (c.xf >> (32 - 11));
        c.xq = c.x0.wrapping_add(c.xq);
        c.x0 = (c.x0 << 11) | (c.x0 >> (32 - 11));
        c.xr = c.x1.wrapping_add(c.xr);
        c.x1 = (c.x1 << 11) | (c.x1 >> (32 - 11));
        c.xo = c.x2.wrapping_add(c.xo);
        c.x2 = (c.x2 << 11) | (c.x2 >> (32 - 11));
        c.xp = c.x3.wrapping_add(c.xp);
        c.x3 = (c.x3 << 11) | (c.x3 >> (32 - 11));
        c.xu = c.x4.wrapping_add(c.xu);
        c.x4 = (c.x4 << 11) | (c.x4 >> (32 - 11));
        c.xv = c.x5.wrapping_add(c.xv);
        c.x5 = (c.x5 << 11) | (c.x5 >> (32 - 11));
        c.xs = c.x6.wrapping_add(c.xs);
        c.x6 = (c.x6 << 11) | (c.x6 >> (32 - 11));
        c.xt = c.x7.wrapping_add(c.xt);
        c.x7 = (c.x7 << 11) | (c.x7 >> (32 - 11));
        c.xc ^= c.xi;
        c.xd ^= c.xj;
        c.xe ^= c.xg;
        c.xf ^= c.xh;
        c.x8 ^= c.xm;
        c.x9 ^= c.xn;
        c.xa ^= c.xk;
        c.xb ^= c.xl;
        c.x4 ^= c.xq;
        c.x5 ^= c.xr;
        c.x6 ^= c.xo;
        c.x7 ^= c.xp;
        c.x0 ^= c.xu;
        c.x1 ^= c.xv;
        c.x2 ^= c.xs;
        c.x3 ^= c.xt;

        c.xj = c.xc.wrapping_add(c.xj);
        c.xc = (c.xc << 7) | (c.xc >> (32 - 7));
        c.xi = c.xd.wrapping_add(c.xi);
        c.xd = (c.xd << 7) | (c.xd >> (32 - 7));
        c.xh = c.xe.wrapping_add(c.xh);
        c.xe = (c.xe << 7) | (c.xe >> (32 - 7));
        c.xg = c.xf.wrapping_add(c.xg);
        c.xf = (c.xf << 7) | (c.xf >> (32 - 7));
        c.xn = c.x8.wrapping_add(c.xn);
        c.x8 = (c.x8 << 7) | (c.x8 >> (32 - 7));
        c.xm = c.x9.wrapping_add(c.xm);
        c.x9 = (c.x9 << 7) | (c.x9 >> (32 - 7));
        c.xl = c.xa.wrapping_add(c.xl);
        c.xa = (c.xa << 7) | (c.xa >> (32 - 7));
        c.xk = c.xb.wrapping_add(c.xk);
        c.xb = (c.xb << 7) | (c.xb >> (32 - 7));
        c.xr = c.x4.wrapping_add(c.xr);
        c.x4 = (c.x4 << 7) | (c.x4 >> (32 - 7));
        c.xq = c.x5.wrapping_add(c.xq);
        c.x5 = (c.x5 << 7) | (c.x5 >> (32 - 7));
        c.xp = c.x6.wrapping_add(c.xp);
        c.x6 = (c.x6 << 7) | (c.x6 >> (32 - 7));
        c.xo = c.x7.wrapping_add(c.xo);
        c.x7 = (c.x7 << 7) | (c.x7 >> (32 - 7));
        c.xv = c.x0.wrapping_add(c.xv);
        c.x0 = (c.x0 << 7) | (c.x0 >> (32 - 7));
        c.xu = c.x1.wrapping_add(c.xu);
        c.x1 = (c.x1 << 7) | (c.x1 >> (32 - 7));
        c.xt = c.x2.wrapping_add(c.xt);
        c.x2 = (c.x2 << 7) | (c.x2 >> (32 - 7));
        c.xs = c.x3.wrapping_add(c.xs);
        c.x3 = (c.x3 << 7) | (c.x3 >> (32 - 7));
        c.x4 ^= c.xj;
        c.x5 ^= c.xi;
        c.x6 ^= c.xh;
        c.x7 ^= c.xg;
        c.x0 ^= c.xn;
        c.x1 ^= c.xm;
        c.x2 ^= c.xl;
        c.x3 ^= c.xk;
        c.xc ^= c.xr;
        c.xd ^= c.xq;
        c.xe ^= c.xp;
        c.xf ^= c.xo;
        c.x8 ^= c.xv;
        c.x9 ^= c.xu;
        c.xa ^= c.xt;
        c.xb ^= c.xs;
        c.xh = c.x4.wrapping_add(c.xh);
        c.x4 = (c.x4 << 11) | (c.x4 >> (32 - 11));
        c.xg = c.x5.wrapping_add(c.xg);
        c.x5 = (c.x5 << 11) | (c.x5 >> (32 - 11));
        c.xj = c.x6.wrapping_add(c.xj);
        c.x6 = (c.x6 << 11) | (c.x6 >> (32 - 11));
        c.xi = c.x7.wrapping_add(c.xi);
        c.x7 = (c.x7 << 11) | (c.x7 >> (32 - 11));
        c.xl = c.x0.wrapping_add(c.xl);
        c.x0 = (c.x0 << 11) | (c.x0 >> (32 - 11));
        c.xk = c.x1.wrapping_add(c.xk);
        c.x1 = (c.x1 << 11) | (c.x1 >> (32 - 11));
        c.xn = c.x2.wrapping_add(c.xn);
        c.x2 = (c.x2 << 11) | (c.x2 >> (32 - 11));
        c.xm = c.x3.wrapping_add(c.xm);
        c.x3 = (c.x3 << 11) | (c.x3 >> (32 - 11));
        c.xp = c.xc.wrapping_add(c.xp);
        c.xc = (c.xc << 11) | (c.xc >> (32 - 11));
        c.xo = c.xd.wrapping_add(c.xo);
        c.xd = (c.xd << 11) | (c.xd >> (32 - 11));
        c.xr = c.xe.wrapping_add(c.xr);
        c.xe = (c.xe << 11) | (c.xe >> (32 - 11));
        c.xq = c.xf.wrapping_add(c.xq);
        c.xf = (c.xf << 11) | (c.xf >> (32 - 11));
        c.xt = c.x8.wrapping_add(c.xt);
        c.x8 = (c.x8 << 11) | (c.x8 >> (32 - 11));
        c.xs = c.x9.wrapping_add(c.xs);
        c.x9 = (c.x9 << 11) | (c.x9 >> (32 - 11));
        c.xv = c.xa.wrapping_add(c.xv);
        c.xa = (c.xa << 11) | (c.xa >> (32 - 11));
        c.xu = c.xb.wrapping_add(c.xu);
        c.xb = (c.xb << 11) | (c.xb >> (32 - 11));
        c.x0 ^= c.xh;
        c.x1 ^= c.xg;
        c.x2 ^= c.xj;
        c.x3 ^= c.xi;
        c.x4 ^= c.xl;
        c.x5 ^= c.xk;
        c.x6 ^= c.xn;
        c.x7 ^= c.xm;
        c.x8 ^= c.xp;
        c.x9 ^= c.xo;
        c.xa ^= c.xr;
        c.xb ^= c.xq;
        c.xc ^= c.xt;
        c.xd ^= c.xs;
        c.xe ^= c.xv;
        c.xf ^= c.xu;
    }
    return c;
}

//cubehash256 calculates cubuhash256.
//length of data must be 32 bytes.
pub fn sum(data: Vec<u8>) -> Vec<u8> {
    let mut c = new_cube_hash();
    let mut buf = vec![0; 32];
    buf[0] = 0x80;
    //let mut inputdata = data.clone();
    //for _i in data.len()..32 {
    //	inputdata.push(0);
    //}
    //c = input_block(inputdata, c);
    c = input_block(data, c);
    c = sixteen_rounds(c);
    c = input_block(buf, c);
    c = sixteen_rounds(c);
    c.xv ^= 1;
    for _j in 0..10 {
        c = sixteen_rounds(c);
    }
    let mut out = vec![];
    out.extend_from_slice(&c.x0.to_le_bytes());
    out.extend_from_slice(&c.x1.to_le_bytes());
    out.extend_from_slice(&c.x2.to_le_bytes());
    out.extend_from_slice(&c.x3.to_le_bytes());
    out.extend_from_slice(&c.x4.to_le_bytes());
    out.extend_from_slice(&c.x5.to_le_bytes());
    out.extend_from_slice(&c.x6.to_le_bytes());
    out.extend_from_slice(&c.x7.to_le_bytes());
    return out;
}

#[test]
fn cubehash_hash_cal() {
    let base1 = "00000000000000000000000000000000".as_bytes().to_vec();
    let cubehash_result1 = sum(base1);
    assert_eq!(
        "f83989901eb3c366e7d7469f8ea8ef0694043cd42deb6252089ff38fb7892f3d",
        cubehash_result1
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );
}
