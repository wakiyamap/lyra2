//! # lyra2
//!
//! `lyra2` crate has necessary formulas to calculate `lyra2`.
use crate::utils::read_u64_le;

const BLAKE2BIV: [u64; 8] = [
    0x6a09_e667_f3bc_c908,
    0xbb67_ae85_84ca_a73b,
    0x3c6e_f372_fe94_f82b,
    0xa54f_f53a_5f1d_36f1,
    0x510e_527f_ade6_82d1,
    0x9b05_688c_2b3e_6c1f,
    0x1f83_d9ab_fb41_bd6b,
    0x5be0_cd19_137e_2179,
];

const BLOCKLENINT64: i64 = 12; //Block length: 768 bits (=96 bytes, =12 uint64_t)
const BLOCKLENBYTES: i64 = BLOCKLENINT64 * 8; //Block length, in bytes
const BLOCKLENBLAKE2SAFEINT64: i64 = 8; //512 bits (=64 bytes, =8 uint64_t)
const BLOCKLENBLAKE2SAFEBYTES: i64 = BLOCKLENBLAKE2SAFEINT64 * 8; //same as above, in bytes

/*Blake2b's rotation*/
fn rotr64(w: u64, c: u8) -> u64 {
    (w >> c) | (w << (64 - c))
}

/*g is Blake2b's G function*/
fn g(a: u64, b: u64, c: u64, d: u64) -> [u64; 4] {
    let mut abcd: [u64; 4] = [a, b, c, d];
    abcd[0] = abcd[0].wrapping_add(abcd[1]);
    abcd[3] = rotr64(abcd[3] ^ abcd[0], 32);
    abcd[2] = abcd[2].wrapping_add(abcd[3]);
    abcd[1] = rotr64(abcd[1] ^ abcd[2], 24);
    abcd[0] = abcd[0].wrapping_add(abcd[1]);
    abcd[3] = rotr64(abcd[3] ^ abcd[0], 16);
    abcd[2] = abcd[2].wrapping_add(abcd[3]);
    abcd[1] = rotr64(abcd[1] ^ abcd[2], 63);
    abcd
}

/*round_lyra is One Round of the Blake2b's compression function*/
fn round_lyra(mut v: [u64; 16]) -> [u64; 16] {
    let mut abcd = g(v[0], v[4], v[8], v[12]);
    v[0] = abcd[0];
    v[4] = abcd[1];
    v[8] = abcd[2];
    v[12] = abcd[3];
    abcd = g(v[1], v[5], v[9], v[13]);
    v[1] = abcd[0];
    v[5] = abcd[1];
    v[9] = abcd[2];
    v[13] = abcd[3];
    abcd = g(v[2], v[6], v[10], v[14]);
    v[2] = abcd[0];
    v[6] = abcd[1];
    v[10] = abcd[2];
    v[14] = abcd[3];
    abcd = g(v[3], v[7], v[11], v[15]);
    v[3] = abcd[0];
    v[7] = abcd[1];
    v[11] = abcd[2];
    v[15] = abcd[3];
    abcd = g(v[0], v[5], v[10], v[15]);
    v[0] = abcd[0];
    v[5] = abcd[1];
    v[10] = abcd[2];
    v[15] = abcd[3];
    abcd = g(v[1], v[6], v[11], v[12]);
    v[1] = abcd[0];
    v[6] = abcd[1];
    v[11] = abcd[2];
    v[12] = abcd[3];
    abcd = g(v[2], v[7], v[8], v[13]);
    v[2] = abcd[0];
    v[7] = abcd[1];
    v[8] = abcd[2];
    v[13] = abcd[3];
    abcd = g(v[3], v[4], v[9], v[14]);
    v[3] = abcd[0];
    v[4] = abcd[1];
    v[9] = abcd[2];
    v[14] = abcd[3];
    v
}

/**
 * init_state Initializes the Sponge State. The first 512 bits are set to zeros and the remainder
 * receive Blake2b's IV as per Blake2b's specification. <b>Note:</b> Even though sponges
 * typically have their internal state initialized with zeros, Blake2b's G function
 * has a fixed point: if the internal state and message are both filled with zeros. the
 * resulting permutation will always be a block filled with zeros; this happens because
 * Blake2b does not use the constants originally employed in Blake2 inside its G function,
 * relying on the IV for avoiding possible fixed points.
 *
 * @param state         The 1024-bit array to be initialized
 */
fn init_state() -> [u64; 16] {
    let mut s: [u64; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    s[8] = BLAKE2BIV[0];
    s[9] = BLAKE2BIV[1];
    s[10] = BLAKE2BIV[2];
    s[11] = BLAKE2BIV[3];
    s[12] = BLAKE2BIV[4];
    s[13] = BLAKE2BIV[5];
    s[14] = BLAKE2BIV[6];
    s[15] = BLAKE2BIV[7];
    s
}

/**
 * blake2b_lyra Execute Blake2b's G function, with all 12 rounds.
 *
 * @param v     A 1024-bit (16 uint64_t) array to be processed by Blake2b's G function
 */
fn blake2b_lyra(mut v: [u64; 16]) -> [u64; 16] {
    v = round_lyra(v);
    v = round_lyra(v);
    v = round_lyra(v);
    v = round_lyra(v);
    v = round_lyra(v);
    v = round_lyra(v);
    v = round_lyra(v);
    v = round_lyra(v);
    v = round_lyra(v);
    v = round_lyra(v);
    v = round_lyra(v);
    v = round_lyra(v);
    v
}

/**
 * reduced_blake2b_lyra Executes a reduced version of Blake2b's G function with only one round
 * @param v     A 1024-bit (16 uint64_t) array to be processed by Blake2b's G function
 */
fn reduced_blake2b_lyra(mut v: [u64; 16]) -> [u64; 16] {
    v = round_lyra(v);
    v
}

/**
 * squeeze Performs a squeeze operation, using Blake2b's G function as the
 * internal permutation
 *
 * @param state      The current state of the sponge
 * @param out        Array that will receive the data squeezed
 * @param len        The number of bytes to be squeezed into the "out" array
 */
fn squeeze(mut state: [u64; 16], output_size: u64) -> Vec<u8> {
    let mut tmp = vec![];
    let mut out = vec![0; output_size as usize];
    let mut _j = 0;
    let jmax = output_size as i64 / BLOCKLENBYTES + 1;
    for _j in 0..jmax {
        for _i in 0..BLOCKLENINT64 {
            tmp.extend_from_slice(&state[_i as usize].to_le_bytes());
        }
        //be care in case of len(out[i:])<len(tmp)
        out[..output_size as usize].clone_from_slice(&tmp[..output_size as usize]);
        state = blake2b_lyra(state);
    }
    out
}

/**
 * absorb_block Performs an absorb operation for a single block (BLOCK_LEN_INT64 words
 * of type uint64_t), using Blake2b's G function as the internal permutation
 *
 * @param s    The current state of the sponge
 * @param w    The block to be absorbed (BLOCK_LEN_INT64 words)
 */
fn absorb_block(mut s: [u64; 16], w: Vec<u64>) -> [u64; 16] {
    //XORs the first BLOCK_LEN_INT64 words of "in" with the current state
    s[0] ^= w[0];
    s[1] ^= w[1];
    s[2] ^= w[2];
    s[3] ^= w[3];
    s[4] ^= w[4];
    s[5] ^= w[5];
    s[6] ^= w[6];
    s[7] ^= w[7];
    s[8] ^= w[8];
    s[9] ^= w[9];
    s[10] ^= w[10];
    s[11] ^= w[11];

    //Applies the transformation f to the sponge's state
    s = blake2b_lyra(s);
    s
}

/**
 * absorb_block_blake2_safe  Performs an absorb operation for a single block (BLOCK_LEN_BLAKE2_SAFE_INT64
 * words of type uint64_t), using Blake2b's G function as the internal permutation
 *
 * @param s    The current state of the sponge
 * @param w    The block to be absorbed (BLOCK_LEN_BLAKE2_SAFE_INT64 words)
 */
fn absorb_block_blake2_safe(mut s: [u64; 16], w: Vec<u64>) -> [u64; 16] {
    //XORs the first BLOCK_LEN_BLAKE2_SAFE_INT64 words of "in" with the current state
    s[0] ^= w[0];
    s[1] ^= w[1];
    s[2] ^= w[2];
    s[3] ^= w[3];
    s[4] ^= w[4];
    s[5] ^= w[5];
    s[6] ^= w[6];
    s[7] ^= w[7];
    //Applies the transformation f to the sponge's state
    s = blake2b_lyra(s);
    s
}

// lyra2 Executes Lyra2 based on the G function from Blake2b. This version supports salts and passwords
// whose combined length is smaller than the size of the memory matrix, (i.e., (n_rows x n_cols x b) bits,
// where "b" is the underlying sponge's bitrate). In this implementation, the "basil" is composed by all
// integer parameters (treated as type "unsigned int") in the order they are provided, plus the value
// of n_cols, (i.e., basil = kLen || pwdlen || saltlen || timeCost || n_rows || n_cols).
//
// @param K The derived key to be output size by the algorithm
// @param pwd User password
// @param salt Salt
// @param time_cost Parameter to determine the processing time (T)
// @param n_rows Number or rows of the memory matrix (R)
// @param n_cols Number of columns of the memory matrix (C)
/// Returns the calculation result of lyra2(advanced).
/// # Examples
///
/// ```
/// let base1 = "abc".as_bytes().to_vec();
/// let base2 = base1.clone();
/// let lyra2_result1 = lyra2::lyra2::lyra2(32, base1, base2, 1, 4, 4);
/// assert_eq!(
///     "8f63758bd178f014ea3fd4df09ff0a61646dc574a0b6bcf2890ec529a6a7360c",
///     lyra2_result1
///         .iter()
///         .map(|n| format!("{:02x}", n))
///         .collect::<String>()
/// );
/// ```
///
/// # Panics
///
/// `time_cost` < 1, `n_rows` < 3
///
pub fn lyra2(
    k: u64,
    pwd: Vec<u8>,
    salt: Vec<u8>,
    time_cost: u64,
    n_rows: u64,
    n_cols: u64,
) -> Vec<u8> {
    //============================= parameter check ============================//
    if time_cost < 1 {panic!()};
    if n_rows < 3 {panic!()};
    //==========================================================================/

    //============================= Basic variables ============================//
    let mut row: i64 = 2; //index of row to be processed
    let mut prev: i64 = 1; //index of prev (last row ever computed/modified)
    let mut rowa: i64 = 0; //index of row* (a previous row, deterministically picked during Setup and randomly picked while Wandering)
    let mut _tau: i64 = 1; //Time Loop iterator
    let mut step: i64 = 1; //Visitation step (used during Setup and Wandering phases)
    let mut window: i64 = 2; //Visitation window (used to define which rows can be revisited during Setup)
    let mut gap: i64 = 1; //Modifier to the step, assuming the values 1 or -1
    let mut _i: i64 = 0; //auxiliary iteration counter
    //==========================================================================/

    //========== Initializing the Memory Matrix and pointers to it =============//
    //Tries to allocate enough space for the whole memory matrix

    let row_len_int64: i64 = BLOCKLENINT64 * n_cols as i64;
    let mut _i: i64 = n_rows as i64 * row_len_int64;
    let mut whole_matrix: Vec<u64> = vec![0; _i as usize];
    whole_matrix.resize(_i as usize, 0);

    //==========================================================================/

    //============= Getting the password + salt + basil padded with 10*1 ===============//
    //OBS.:The memory matrix will temporarily hold the password: not for saving memory,
    //but this ensures that the password copied locally will be overwritten as soon as possible

    //First, we clean enough blocks for the password, salt, basil and padding
    let n_blocks_input: i64 =
        ((salt.len() + pwd.len() + 6 * 8) as i64 / BLOCKLENBLAKE2SAFEBYTES) + 1;
    let mut ptr_byte: usize = 0; // (byte*) whole_matrix;

    // Prepends the password
    for j in 0..pwd.len() / 8 {
        let start = 8 * j;
        let end = start + 8;
        whole_matrix[ptr_byte + j] = read_u64_le(&pwd[start..end]);
    }

    ptr_byte += (pwd.len() as u64 / 8) as usize;

    // Concatenates the salt
    for j in 0..salt.len() / 8 {
        let start = 8 * j;
        let end = start + 8;
        whole_matrix[ptr_byte + j] = read_u64_le(&salt[start..end]);
    }

    ptr_byte += (salt.len() as u64 / 8) as usize;

    //Concatenates the basil: every integer passed as parameter, in the order they are provided by the interface
    whole_matrix[ptr_byte] = k;
    ptr_byte += 1;
    whole_matrix[ptr_byte] = pwd.len() as u64;
    ptr_byte += 1;
    whole_matrix[ptr_byte] = salt.len() as u64;
    ptr_byte += 1;
    whole_matrix[ptr_byte] = time_cost;
    ptr_byte += 1;
    whole_matrix[ptr_byte] = n_rows;
    ptr_byte += 1;
    whole_matrix[ptr_byte] = n_cols;
    ptr_byte += 1;

    //Now comes the padding
    whole_matrix[ptr_byte] = 0x80; //first byte of padding: right after the password
                                   //resets the pointer to the start of the memory matrix
    ptr_byte = ((n_blocks_input * BLOCKLENBLAKE2SAFEBYTES) / 8 - 1) as usize; //sets the pointer to the correct position: end of incomplete block
    whole_matrix[ptr_byte] ^= 0x0100_0000_0000_0000; //last byte of padding: at the end of the last incomplete block00
    //==========================================================================/

    //======================= Initializing the Sponge State ====================//
    //Sponge state: 16 uint64_t, BLOCK_LEN_INT64 words of them for the bitrate (b) and the remainder for the capacity (c)
    let mut state = init_state();
    //==========================================================================/

    //================================ Setup Phase =============================//
    //Absorbing salt, password and basil: this is the only place in which the block length is hard-coded to 512 bits
    let mut ptr_word = 0;
    for _i in 0..n_blocks_input {
        let (_, mut _right) = &whole_matrix.split_at(ptr_word);
        state = absorb_block_blake2_safe(state, _right.to_vec()); //absorbs each block of pad(pwd || salt || basil)
        ptr_word += BLOCKLENBLAKE2SAFEINT64 as usize; //goes to next block of pad(pwd || salt || basil)
    }

    //Initializes M[0] and M[1]
    //reducedSqueezeRow0
    //The locally copied password is most likely overwritten here
    let mut ptr = (n_cols as i64 - 1) * BLOCKLENINT64;
    //M[row][C-1-col] = H.reduced_squeeze()
    for _i in 0..n_cols {
        //In Lyra2: pointer to M[0][C-1]
        whole_matrix[ptr as usize] = state[0];
        whole_matrix[(ptr + 1) as usize] = state[1];
        whole_matrix[(ptr + 2) as usize] = state[2];
        whole_matrix[(ptr + 3) as usize] = state[3];
        whole_matrix[(ptr + 4) as usize] = state[4];
        whole_matrix[(ptr + 5) as usize] = state[5];
        whole_matrix[(ptr + 6) as usize] = state[6];
        whole_matrix[(ptr + 7) as usize] = state[7];
        whole_matrix[(ptr + 8) as usize] = state[8];
        whole_matrix[(ptr + 9) as usize] = state[9];
        whole_matrix[(ptr + 10) as usize] = state[10];
        whole_matrix[(ptr + 11) as usize] = state[11];

        //Goes to next block (column) that will receive the squeezed data
        ptr = ptr.wrapping_sub(BLOCKLENINT64);

        //Applies the reduced-round transformation f to the sponge's state
        state = reduced_blake2b_lyra(state);
    }

    //reducedDuplexRow1
    for _i in 0..n_cols {
        //Absorbing "M[prev][col]"
        state[0] ^= whole_matrix[(_i as i64 * BLOCKLENINT64) as usize];
        state[1] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + 1) as usize];
        state[2] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + 2) as usize];
        state[3] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + 3) as usize];
        state[4] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + 4) as usize];
        state[5] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + 5) as usize];
        state[6] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + 6) as usize];
        state[7] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + 7) as usize];
        state[8] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + 8) as usize];
        state[9] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + 9) as usize];
        state[10] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + 10) as usize];
        state[11] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + 11) as usize];

        //Applies the reduced-round transformation f to the sponge's state
        state = reduced_blake2b_lyra(state);

        //M[row][C-1-col] = M[prev][col] XOR rand
        whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row_len_int64) as usize] =
            whole_matrix[(_i as i64 * BLOCKLENINT64) as usize] ^ state[0];
        whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row_len_int64 + 1) as usize] =
            whole_matrix[(_i as i64 * BLOCKLENINT64 + 1) as usize] ^ state[1];
        whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row_len_int64 + 2) as usize] =
            whole_matrix[(_i as i64 * BLOCKLENINT64 + 2) as usize] ^ state[2];
        whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row_len_int64 + 3) as usize] =
            whole_matrix[(_i as i64 * BLOCKLENINT64 + 3) as usize] ^ state[3];
        whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row_len_int64 + 4) as usize] =
            whole_matrix[(_i as i64 * BLOCKLENINT64 + 4) as usize] ^ state[4];
        whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row_len_int64 + 5) as usize] =
            whole_matrix[(_i as i64 * BLOCKLENINT64 + 5) as usize] ^ state[5];
        whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row_len_int64 + 6) as usize] =
            whole_matrix[(_i as i64 * BLOCKLENINT64 + 6) as usize] ^ state[6];
        whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row_len_int64 + 7) as usize] =
            whole_matrix[(_i as i64 * BLOCKLENINT64 + 7) as usize] ^ state[7];
        whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row_len_int64 + 8) as usize] =
            whole_matrix[(_i as i64 * BLOCKLENINT64 + 8) as usize] ^ state[8];
        whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row_len_int64 + 9) as usize] =
            whole_matrix[(_i as i64 * BLOCKLENINT64 + 9) as usize] ^ state[9];
        whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row_len_int64 + 10) as usize] =
            whole_matrix[(_i as i64 * BLOCKLENINT64 + 10) as usize] ^ state[10];
        whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row_len_int64 + 11) as usize] =
            whole_matrix[(_i as i64 * BLOCKLENINT64 + 11) as usize] ^ state[11];
    }

    let mut _x: i64 = row;
    for _x in _x..n_rows as i64 {
        //M[row] = rand; //M[row*] = M[row*] XOR rotW(rand)
        //reducedDuplexRowSetup
        for _i in 0..n_cols {
            //Absorbing "M[prev] [+] M[row*]"
            state[0] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64) as usize]
                .wrapping_add(
                    whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64) as usize],
                );
            state[1] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 1) as usize]
                .wrapping_add(
                    whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 1) as usize],
                );
            state[2] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 2) as usize]
                .wrapping_add(
                    whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 2) as usize],
                );
            state[3] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 3) as usize]
                .wrapping_add(
                    whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 3) as usize],
                );
            state[4] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 4) as usize]
                .wrapping_add(
                    whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 4) as usize],
                );
            state[5] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 5) as usize]
                .wrapping_add(
                    whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 5) as usize],
                );
            state[6] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 6) as usize]
                .wrapping_add(
                    whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 6) as usize],
                );
            state[7] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 7) as usize]
                .wrapping_add(
                    whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 7) as usize],
                );
            state[8] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 8) as usize]
                .wrapping_add(
                    whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 8) as usize],
                );
            state[9] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 9) as usize]
                .wrapping_add(
                    whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 9) as usize],
                );
            state[10] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 10) as usize]
                .wrapping_add(
                    whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 10) as usize],
                );
            state[11] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 11) as usize]
                .wrapping_add(
                    whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 11) as usize],
                );

            //Applies the reduced-round transformation f to the sponge's state
            state = reduced_blake2b_lyra(state);

            //M[row][col] = M[prev][col] XOR rand
            whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row * row_len_int64) as usize] =
                whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64) as usize] ^ state[0];
            whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row * row_len_int64 + 1) as usize] =
                whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 1) as usize] ^ state[1];
            whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row * row_len_int64 + 2) as usize] =
                whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 2) as usize] ^ state[2];
            whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row * row_len_int64 + 3) as usize] =
                whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 3) as usize] ^ state[3];
            whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row * row_len_int64 + 4) as usize] =
                whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 4) as usize] ^ state[4];
            whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row * row_len_int64 + 5) as usize] =
                whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 5) as usize] ^ state[5];
            whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row * row_len_int64 + 6) as usize] =
                whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 6) as usize] ^ state[6];
            whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row * row_len_int64 + 7) as usize] =
                whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 7) as usize] ^ state[7];
            whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row * row_len_int64 + 8) as usize] =
                whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 8) as usize] ^ state[8];
            whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row * row_len_int64 + 9) as usize] =
                whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 9) as usize] ^ state[9];
            whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row * row_len_int64 + 10) as usize] =
                whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 10) as usize] ^ state[10];
            whole_matrix[((n_cols - _i - 1) as i64 * BLOCKLENINT64 + row * row_len_int64 + 11) as usize] =
                whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 11) as usize] ^ state[11];

            //M[row*][col] = M[row*][col] XOR rotW(rand)
            whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64) as usize] ^= state[11];
            whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 1) as usize] ^= state[0];
            whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 2) as usize] ^= state[1];
            whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 3) as usize] ^= state[2];
            whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 4) as usize] ^= state[3];
            whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 5) as usize] ^= state[4];
            whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 6) as usize] ^= state[5];
            whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 7) as usize] ^= state[6];
            whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 8) as usize] ^= state[7];
            whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 9) as usize] ^= state[8];
            whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 10) as usize] ^= state[9];
            whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 11) as usize] ^= state[10];
        }

        //updates the value of row* (deterministically picked during Setup))
        rowa = (rowa + step) & (window - 1);
        //update prev: it now points to the last row ever computed
        prev = row;
        //updates row: goes to the next row to be computed
        row += 1;

        //Checks if all rows in the window where visited.
        if rowa == 0 {
            step = window + gap; //changes the step: approximately doubles its value
            window *= 2; //doubles the size of the re-visitation window
            gap = -gap; //inverts the modifier to the step
        }
    }
    //==========================================================================/

    //============================ Wandering Phase =============================//
    row = 0; //Resets the visitation to the first row of the memory matrix
    for _tau in 1..=time_cost {
        //Step is approximately half the number of all rows of the memory matrix for an odd _tau; otherwise, it is -1
        step = n_rows as i64 / 2 - 1;
        if _tau % 2 == 0 {
            step = -1;
        }

        loop {
            //Selects a pseudorandom index row*
            //------------------------------------------------------------------------------------------
            //rowa = ((unsigned int)state[0]) & (n_rows-1);	//(USE THIS IF n_rows IS A POWER OF 2)
            rowa = (state[0] & (n_rows - 1)) as i64; //(USE THIS FOR THE "GENERIC" CASE)
            //------------------------------------------------------------------------------------------

            //Performs a reduced-round duplexing operation over M[row*] XOR M[prev], updating both M[row*] and M[row]
            //reducedDuplexRow(state, memMatrix[prev], memMatrix[rowa], memMatrix[row], n_cols)
            for _i in 0..n_cols {
                state[0] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64) as usize]
                    .wrapping_add(
                        whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64) as usize],
                    );
                state[1] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 1) as usize]
                    .wrapping_add(
                        whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 1) as usize],
                    );
                state[2] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 2) as usize]
                    .wrapping_add(
                        whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 2) as usize],
                    );
                state[3] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 3) as usize]
                    .wrapping_add(
                        whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 3) as usize],
                    );
                state[4] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 4) as usize]
                    .wrapping_add(
                        whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 4) as usize],
                    );
                state[5] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 5) as usize]
                    .wrapping_add(
                        whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 5) as usize],
                    );
                state[6] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 6) as usize]
                    .wrapping_add(
                        whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 6) as usize],
                    );
                state[7] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 7) as usize]
                    .wrapping_add(
                        whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 7) as usize],
                    );
                state[8] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 8) as usize]
                    .wrapping_add(
                        whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 8) as usize],
                    );
                state[9] ^= whole_matrix[(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 9) as usize]
                    .wrapping_add(
                        whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 9) as usize],
                    );
                state[10] ^= whole_matrix
                    [(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 10) as usize]
                    .wrapping_add(
                        whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 10) as usize],
                    );
                state[11] ^= whole_matrix
                    [(_i as i64 * BLOCKLENINT64 + prev * row_len_int64 + 11) as usize]
                    .wrapping_add(
                        whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 11) as usize],
                    );

                //Applies the reduced-round transformation f to the sponge's state
                state = reduced_blake2b_lyra(state);

                //M[rowOut][col] = M[rowOut][col] XOR rand
                whole_matrix[(_i as i64 * BLOCKLENINT64 + row * row_len_int64) as usize] ^= state[0];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + row * row_len_int64 + 1) as usize] ^= state[1];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + row * row_len_int64 + 2) as usize] ^= state[2];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + row * row_len_int64 + 3) as usize] ^= state[3];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + row * row_len_int64 + 4) as usize] ^= state[4];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + row * row_len_int64 + 5) as usize] ^= state[5];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + row * row_len_int64 + 6) as usize] ^= state[6];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + row * row_len_int64 + 7) as usize] ^= state[7];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + row * row_len_int64 + 8) as usize] ^= state[8];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + row * row_len_int64 + 9) as usize] ^= state[9];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + row * row_len_int64 + 10) as usize] ^= state[10];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + row * row_len_int64 + 11) as usize] ^= state[11];

                //M[rowInOut][col] = M[rowInOut][col] XOR rotW(rand)
                whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64) as usize] ^= state[11];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 1) as usize] ^= state[0];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 2) as usize] ^= state[1];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 3) as usize] ^= state[2];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 4) as usize] ^= state[3];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 5) as usize] ^= state[4];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 6) as usize] ^= state[5];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 7) as usize] ^= state[6];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 8) as usize] ^= state[7];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 9) as usize] ^= state[8];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 10) as usize] ^= state[9];
                whole_matrix[(_i as i64 * BLOCKLENINT64 + rowa * row_len_int64 + 11) as usize] ^= state[10];
            }

            //update prev: it now points to the last row ever computed
            prev = row;

            //updates row: goes to the next row to be computed
            //------------------------------------------------------------------------------------------
            //row = (row + step) & (n_rows-1);	//(USE THIS IF n_rows IS A POWER OF 2)
            row = (row + step) & (n_rows as i64 -1); //(USE THIS FOR THE "GENERIC" CASE)
            //------------------------------------------------------------------------------------------
            if row == 0 {
                break;
            }
        }
    }
    //==========================================================================/

    //============================ Wrap-up Phase ===============================//
    //Absorbs the last block of the memory matrix
    let (_, mut _right) = &whole_matrix.split_at((rowa * row_len_int64) as usize);
    state = absorb_block(state, _right.to_vec());
    //Squeezes the key
    squeeze(state, k)
    //==========================================================================/
}

/// Returns the calculation result of lyra2.
/// # Examples
///
/// ```
/// let base1 = "abc".as_bytes().to_vec();
/// let lyra2_result1 = lyra2::lyra2::sum(base1);
/// assert_eq!(
///     "8f63758bd178f014ea3fd4df09ff0a61646dc574a0b6bcf2890ec529a6a7360c",
///     lyra2_result1
///         .iter()
///         .map(|n| format!("{:02x}", n))
///         .collect::<String>()
/// );
/// ```
pub fn sum(input: Vec<u8>) -> Vec<u8> {
    let input2 = input.clone();
    lyra2(32, input, input2, 1, 4, 4)
}

#[test]
fn lyra2_hash_cal() {
    let base1 = "abc".as_bytes().to_vec();
    let lyra2_result1 = sum(base1);
    assert_eq!(
        "8f63758bd178f014ea3fd4df09ff0a61646dc574a0b6bcf2890ec529a6a7360c",
        lyra2_result1
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );

    let base3 = "脇山珠美ちゃんかわいい！".as_bytes().to_vec();
    let base4 = base3.clone();
    let lyra2_result2 = lyra2(48, base3, base4, 1, 3, 4);
    assert_eq!("c937cfe0ee21a8e7c1d1871245ea717457edbee2de8bf544e50f807349a3460c52cb6bb10bd0b7328504bc2ad984e1f3", lyra2_result2.iter().map(|n| format!("{:02x}", n)).collect::<String>());

    let base5 = "😀😁😂".as_bytes().to_vec();
    let base6 = base5.clone();
    let lyra2_result3 = lyra2(16, base5, base6, 1, 4, 2);
    assert_eq!(
        "372557ef600c8c76bedd91ecd5a01f45",
        lyra2_result3
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );
}
