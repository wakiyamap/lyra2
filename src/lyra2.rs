use byteorder::{ByteOrder, LittleEndian};
use bytes::{Bytes, BufMut};

const BLAKE2BIV: [u64; 8] = [
	0x6a09e667f3bcc908, 0xbb67ae8584caa73b,
	0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
	0x510e527fade682d1, 0x9b05688c2b3e6c1f,
	0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
];

const BLOCKLENINT64: u64 = 12;                //Block length: 768 bits (=96 bytes, =12 uint64_t)
const BLOCKLENBYTES: u64 = BLOCKLENINT64 * 8; //Block length, in bytes
const BLOCKLENBLAKE2SAFEINT64: u64 = 8;                             //512 bits (=64 bytes, =8 uint64_t)
const BLOCKLENBLAKE2SAFEBYTES: u64 = (BLOCKLENBLAKE2SAFEINT64 * 8); //same as above, in bytes

/*Blake2b's rotation*/
fn rotr64(w: u64, c: u8) -> u64{
	return (w >> c) | (w << (64 - c));
}

/*g is Blake2b's G function*/
fn g(mut a: u64, mut b: u64, mut c: u64, mut d: u64) -> [u64; 4]{
	let mut abcd: [u64; 4] = [a, b, c, d];
	abcd[0] = abcd[0].wrapping_add(abcd[1]);
	abcd[3] = rotr64(abcd[3]^abcd[0], 32);
	abcd[2] = abcd[2].wrapping_add(abcd[3]);
	abcd[1] = rotr64(abcd[1]^abcd[2], 24);
	abcd[0] = abcd[0].wrapping_add(abcd[1]);
	abcd[3] = rotr64(abcd[3]^abcd[0], 16);
	abcd[2] = abcd[2].wrapping_add(abcd[3]);
	abcd[1] = rotr64(abcd[1]^abcd[2], 63);
	return abcd;
}

/*roundLyra is One Round of the Blake2b's compression function*/
fn roundLyra(mut v: [u64; 16]) -> [u64; 16]{
	let mut abcd = g(v[0], v[4], v[8], v[12]);
	v[0] = abcd[0];
	v[4] = abcd[1];
	v[8] = abcd[2];
	v[12] = abcd[3];
	let mut abcd = g(v[1], v[5], v[9], v[13]);
	v[1] = abcd[0];
	v[5] = abcd[1];
	v[9] = abcd[2];
	v[13] = abcd[3];
	let mut abcd = g(v[2], v[6], v[10], v[14]);
	v[2] = abcd[0];
	v[6] = abcd[1];
	v[10] = abcd[2];
	v[14] = abcd[3];
	let mut abcd = g(v[3], v[7], v[11], v[15]);
	v[3] = abcd[0];
	v[7] = abcd[1];
	v[11] = abcd[2];
	v[15] = abcd[3];
	let mut abcd = g(v[0], v[5], v[10], v[15]);
	v[0] = abcd[0];
	v[5] = abcd[1];
	v[10] = abcd[2];
	v[15] = abcd[3];
	let mut abcd = g(v[1], v[6], v[11], v[12]);
	v[1] = abcd[0];
	v[6] = abcd[1];
	v[11] = abcd[2];
	v[12] = abcd[3];
	let mut abcd = g(v[2], v[7], v[8], v[13]);
	v[2] = abcd[0];
	v[7] = abcd[1];
	v[8] = abcd[2];
	v[13] = abcd[3];
	let mut abcd = g(v[3], v[4], v[9], v[14]);
	v[3] = abcd[0];
	v[4] = abcd[1];
	v[9] = abcd[2];
	v[14] = abcd[3];
	return v;
}

/**
 * initState Initializes the Sponge State. The first 512 bits are set to zeros and the remainder
 * receive Blake2b's IV as per Blake2b's specification. <b>Note:</b> Even though sponges
 * typically have their internal state initialized with zeros, Blake2b's G function
 * has a fixed point: if the internal state and message are both filled with zeros. the
 * resulting permutation will always be a block filled with zeros; this happens because
 * Blake2b does not use the constants originally employed in Blake2 inside its G function,
 * relying on the IV for avoiding possible fixed points.
 *
 * @param state         The 1024-bit array to be initialized
 */
fn initState() -> [u64; 16] {
	let mut s: [u64; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
	s[8] = BLAKE2BIV[0];
	s[9] = BLAKE2BIV[1];
	s[10] = BLAKE2BIV[2];
	s[11] = BLAKE2BIV[3];
	s[12] = BLAKE2BIV[4];
	s[13] = BLAKE2BIV[5];
	s[14] = BLAKE2BIV[6];
	s[15] = BLAKE2BIV[7];
	return s;
}

/**
 * Eblake2bLyraxecute Blake2b's G function, with all 12 rounds.
 *
 * @param v     A 1024-bit (16 uint64_t) array to be processed by Blake2b's G function
 */
fn blake2bLyra(mut v: [u64; 16]) -> [u64; 16]{
	let mut v = roundLyra(v);
	let mut v = roundLyra(v);
	let mut v = roundLyra(v);
	let mut v = roundLyra(v);
	let mut v = roundLyra(v);
	let mut v = roundLyra(v);
	let mut v = roundLyra(v);
	let mut v = roundLyra(v);
	let mut v = roundLyra(v);
	let mut v = roundLyra(v);
	let mut v = roundLyra(v);
	let mut v = roundLyra(v);
	return v;
}

/**
 * reducedBlake2bLyra Executes a reduced version of Blake2b's G function with only one round
 * @param v     A 1024-bit (16 uint64_t) array to be processed by Blake2b's G function
 */
fn reducedBlake2bLyra(mut v: [u64; 16]) -> [u64; 16]{
	let mut v = roundLyra(v);
	return v;
}

/**
 * squeeze Performs a squeeze operation, using Blake2b's G function as the
 * internal permutation
 *
 * @param state      The current state of the sponge
 * @param out        Array that will receive the data squeezed
 * @param len        The number of bytes to be squeezed into the "out" array
 */
fn squeeze(state: [u64; 16], out: Vec<u8>) -> Vec<u8>{
	let mut k = vec![];
	let mut _j = 0;
	let jmax = out.len()/BLOCKLENBYTES as usize +1;
	for _j in 0..jmax {
		let mut _i = 0;
		for _i in 0..BLOCKLENINT64 as usize {
			k.put_u64_le(state[_i as usize]);
		}
		//copy(out[j*BLOCKLENBYTES:], tmp) //be care in case of len(out[i:])<len(tmp)
		let mut state = blake2bLyra(state);
	}
	return k;
}

/**
 * absorbBlock Performs an absorb operation for a single block (BLOCK_LEN_INT64 words
 * of type uint64_t), using Blake2b's G function as the internal permutation
 *
 * @param state The current state of the sponge
 * @param in    The block to be absorbed (BLOCK_LEN_INT64 words)
 */
fn absorbBlock(mut s: [u64; 16], inWholeMatrix: Vec<u64>) -> [u64; 16]{
	//XORs the first BLOCK_LEN_INT64 words of "in" with the current state
	s[0] ^= inWholeMatrix[0];
	s[1] ^= inWholeMatrix[1];
	s[2] ^= inWholeMatrix[2];
	s[3] ^= inWholeMatrix[3];
	s[4] ^= inWholeMatrix[4];
	s[5] ^= inWholeMatrix[5];
	s[6] ^= inWholeMatrix[6];
	s[7] ^= inWholeMatrix[7];
	s[8] ^= inWholeMatrix[8];
	s[9] ^= inWholeMatrix[9];
	s[10] ^= inWholeMatrix[10];
	s[11] ^= inWholeMatrix[11];

	//Applies the transformation f to the sponge's state
	let mut s = blake2bLyra(s);
	return s;
}

/**
 * absorbBlockBlake2Safe  Performs an absorb operation for a single block (BLOCK_LEN_BLAKE2_SAFE_INT64
 * words of type uint64_t), using Blake2b's G function as the internal permutation
 *
 * @param state            The current state of the sponge
 * @param inWholeMatrix    The block to be absorbed (BLOCK_LEN_BLAKE2_SAFE_INT64 words)
 */
fn absorbBlockBlake2Safe(mut s: [u64; 16], inWholeMatrix: Vec<u64>) -> [u64; 16]{
	//XORs the first BLOCK_LEN_BLAKE2_SAFE_INT64 words of "in" with the current state
	s[0] ^= inWholeMatrix[0];
	s[1] ^= inWholeMatrix[1];
	s[2] ^= inWholeMatrix[2];
	s[3] ^= inWholeMatrix[3];
	s[4] ^= inWholeMatrix[4];
	s[5] ^= inWholeMatrix[5];
	s[6] ^= inWholeMatrix[6];
	s[7] ^= inWholeMatrix[7];
	//Applies the transformation f to the sponge's state
	let mut s = blake2bLyra(s);
	return s;
}

// lyra2 Executes Lyra2 based on the G function from Blake2b. This version supports salts and passwords
// whose combined length is smaller than the size of the memory matrix, (i.e., (nRows x nCols x b) bits,
// where "b" is the underlying sponge's bitrate). In this implementation, the "basil" is composed by all
// integer parameters (treated as type "unsigned int") in the order they are provided, plus the value
// of nCols, (i.e., basil = kLen || pwdlen || saltlen || timeCost || nRows || nCols).
//
// @param K The derived key to be output by the algorithm
// @param kLen Desired key length
// @param pwd User password
// @param pwdlen Password length
// @param salt Salt
// @param saltlen Salt length
// @param timeCost Parameter to determine the processing time (T)
// @param nRows Number or rows of the memory matrix (R)
// @param nCols Number of columns of the memory matrix (C)
//
// @return 0 if the key is generated correctly; -1 if there is an error (usually due to lack of memory for allocation)
fn lyra2(k: Vec<u8>, pwd: Vec<u8>, salt: Vec<u8>, timeCost: u64, nRows: u64, nCols: u64) -> Vec<u8>{

	//============================= Basic variables ============================//
	let mut row: u64 = 2;              //index of row to be processed
	let mut prev: u64 = 1;             //index of prev (last row ever computed/modified)
	let mut rowa: u64 = 0;       //index of row* (a previous row, deterministically picked during Setup and randomly picked while Wandering)
	let mut tau: u64 = 1;        //Time Loop iterator
	let mut step: i32 = 1;             //Visitation step (used during Setup and Wandering phases)
	let mut window: u64 = 2; //Visitation window (used to define which rows can be revisited during Setup)
	let mut gap: i32 = 1;    //Modifier to the step, assuming the values 1 or -1
	let mut _i: u64 = 0;             //auxiliary iteration counter
	//==========================================================================/

	//========== Initializing the Memory Matrix and pointers to it =============//
	//Tries to allocate enough space for the whole memory matrix

	let mut rowLenInt64: u64 = BLOCKLENINT64 * nCols;
	let mut i : u64 = nRows * rowLenInt64;
	let mut wholeMatrix: Vec<u64> = Vec::new();
	wholeMatrix.resize(i as usize, 0);
	let mut wholeMatrixLen : usize = i as usize;

	//==========================================================================/

	//============= Getting the password + salt + basil padded with 10*1 ===============//
	//OBS.:The memory matrix will temporarily hold the password: not for saving memory,
	//but this ensures that the password copied locally will be overwritten as soon as possible

	//First, we clean enough blocks for the password, salt, basil and padding
	let mut nBlocksInput:u64 = ((salt.len() as u64 + pwd.len() as u64 + 6*8) / BLOCKLENBLAKE2SAFEBYTES) + 1;
	let mut ptrByte: u64 = 0; // (byte*) wholeMatrix;

	//Prepends the password
	let mut _right = &pwd;
	for _j in 0..pwd.len()/8 {
		let (_, mut _right) = &pwd.split_at(8*_j);
		wholeMatrix[ptrByte as usize +_j] = LittleEndian::read_u64(_right);
	}

	ptrByte += pwd.len() as u64 / 8;

	//Concatenates the salt
	let mut _right = &salt;
	for _j in 0..salt.len()/8 {
		let (_, mut _right) = &pwd.split_at(8*_j);
		wholeMatrix[ptrByte as usize +_j] = LittleEndian::read_u64(_right);
	}

	ptrByte += salt.len() as u64 / 8;

	//Concatenates the basil: every integer passed as parameter, in the order they are provided by the interface
	wholeMatrix[ptrByte as usize] = k.len() as u64;
	ptrByte += 1;
	wholeMatrix[ptrByte as usize] = pwd.len() as u64;
	ptrByte += 1;
	wholeMatrix[ptrByte as usize] = salt.len() as u64;
	ptrByte += 1;
	wholeMatrix[ptrByte as usize] = timeCost;
	ptrByte += 1;
	wholeMatrix[ptrByte as usize] = nRows;
	ptrByte += 1;
	wholeMatrix[ptrByte as usize] = nCols;
	ptrByte += 1;

	//Now comes the padding
	wholeMatrix[ptrByte as usize] = 0x80; //first byte of padding: right after the password
	//resets the pointer to the start of the memory matrix
	ptrByte = (nBlocksInput*BLOCKLENBLAKE2SAFEBYTES)/8 - 1; //sets the pointer to the correct position: end of incomplete block
	wholeMatrix[ptrByte as usize] ^= 0x0100000000000000;    //last byte of padding: at the end of the last incomplete block00
	//==========================================================================/

	//======================= Initializing the Sponge State ====================//
	//Sponge state: 16 uint64_t, BLOCK_LEN_INT64 words of them for the bitrate (b) and the remainder for the capacity (c)
	let mut state = initState();
	//==========================================================================/

	//================================ Setup Phase =============================//
	//Absorbing salt, password and basil: this is the only place in which the block length is hard-coded to 512 bits
	let mut ptrWord = 0;
	for _i in 0..nBlocksInput {
		let (_, mut _right) = &wholeMatrix.split_at(ptrWord);
		state = absorbBlockBlake2Safe(state, _right.to_vec());     //absorbs each block of pad(pwd || salt || basil)
		ptrWord += BLOCKLENBLAKE2SAFEINT64 as usize;   //goes to next block of pad(pwd || salt || basil)
	}

	//Initializes M[0] and M[1]
	//reducedSqueezeRow0
	//The locally copied password is most likely overwritten here
	let mut ptr = (nCols - 1) * BLOCKLENINT64;
	let mut ptrWord = wholeMatrix;
	//M[row][C-1-col] = H.reduced_squeeze()
	for _i in 0..nCols {
		//In Lyra2: pointer to M[0][C-1]
		ptrWord[ptr as usize] = state[0];
		ptrWord[(ptr+1) as usize] = state[1];
		ptrWord[(ptr+2) as usize] = state[2];
		ptrWord[(ptr+3) as usize] = state[3];
		ptrWord[(ptr+4) as usize] = state[4];
		ptrWord[(ptr+5) as usize] = state[5];
		ptrWord[(ptr+6) as usize] = state[6];
		ptrWord[(ptr+7) as usize] = state[7];
		ptrWord[(ptr+8) as usize] = state[8];
		ptrWord[(ptr+9) as usize] = state[9];
		ptrWord[(ptr+10) as usize] = state[10];
		ptrWord[(ptr+11) as usize] = state[11];

		//Goes to next block (column) that will receive the squeezed data
		ptr = ptr.wrapping_sub(BLOCKLENINT64);

		//Applies the reduced-round transformation f to the sponge's state
		state = reducedBlake2bLyra(state);
	}
	let mut wholeMatrix = ptrWord;

	//reducedDuplexRow1
	for _i in 0..nCols {
		//Absorbing "M[prev][col]"
		state[0] ^= wholeMatrix[(_i*BLOCKLENINT64 +0) as usize];
		state[1] ^= wholeMatrix[(_i*BLOCKLENINT64 +1) as usize];
		state[2] ^= wholeMatrix[(_i*BLOCKLENINT64 +2) as usize];
		state[3] ^= wholeMatrix[(_i*BLOCKLENINT64 +3) as usize];
		state[4] ^= wholeMatrix[(_i*BLOCKLENINT64 +4) as usize];
		state[5] ^= wholeMatrix[(_i*BLOCKLENINT64 +5) as usize];
		state[6] ^= wholeMatrix[(_i*BLOCKLENINT64 +6) as usize];
		state[7] ^= wholeMatrix[(_i*BLOCKLENINT64 +7) as usize];
		state[8] ^= wholeMatrix[(_i*BLOCKLENINT64 +8) as usize];
		state[9] ^= wholeMatrix[(_i*BLOCKLENINT64 +9) as usize];
		state[10] ^= wholeMatrix[(_i*BLOCKLENINT64 +10) as usize];
		state[11] ^= wholeMatrix[(_i*BLOCKLENINT64 +11) as usize];

		//Applies the reduced-round transformation f to the sponge's state
		state = reducedBlake2bLyra(state);

		//M[row][C-1-col] = M[prev][col] XOR rand
		wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + rowLenInt64 +0) as usize] = wholeMatrix[(_i*BLOCKLENINT64 +0) as usize] ^ state[0];
		wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + rowLenInt64 +1) as usize] = wholeMatrix[(_i*BLOCKLENINT64 +1) as usize] ^ state[1];
		wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + rowLenInt64 +2) as usize] = wholeMatrix[(_i*BLOCKLENINT64 +2) as usize] ^ state[2];
		wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + rowLenInt64 +3) as usize] = wholeMatrix[(_i*BLOCKLENINT64 +3) as usize] ^ state[3];
		wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + rowLenInt64 +4) as usize] = wholeMatrix[(_i*BLOCKLENINT64 +4) as usize] ^ state[4];
		wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + rowLenInt64 +5) as usize] = wholeMatrix[(_i*BLOCKLENINT64 +5) as usize] ^ state[5];
		wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + rowLenInt64 +6) as usize] = wholeMatrix[(_i*BLOCKLENINT64 +6) as usize] ^ state[6];
		wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + rowLenInt64 +7) as usize] = wholeMatrix[(_i*BLOCKLENINT64 +7) as usize] ^ state[7];
		wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + rowLenInt64 +8) as usize] = wholeMatrix[(_i*BLOCKLENINT64 +8) as usize] ^ state[8];
		wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + rowLenInt64 +9) as usize] = wholeMatrix[(_i*BLOCKLENINT64 +9) as usize] ^ state[9];
		wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + rowLenInt64 +10) as usize] = wholeMatrix[(_i*BLOCKLENINT64 +10) as usize] ^ state[10];
		wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + rowLenInt64 +11) as usize] = wholeMatrix[(_i*BLOCKLENINT64 +11) as usize] ^ state[11];
	}

	let mut _x = row.clone();
	for _x in _x..nRows {
		//M[row] = rand; //M[row*] = M[row*] XOR rotW(rand)
		//reducedDuplexRowSetup
		for _i in 0..nCols {

			//Absorbing "M[prev] [+] M[row*]"
			state[0] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +0) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa*rowLenInt64 +0) as usize]);
			state[1] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +1) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa*rowLenInt64 +1) as usize]);
			state[2] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +2) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa*rowLenInt64 +2) as usize]);
			state[3] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +3) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa*rowLenInt64 +3) as usize]);
			state[4] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +4) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa*rowLenInt64 +4) as usize]);
			state[5] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +5) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa*rowLenInt64 +5) as usize]);
			state[6] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +6) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa*rowLenInt64 +6) as usize]);
			state[7] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +7) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa*rowLenInt64 +7) as usize]);
			state[8] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +8) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa*rowLenInt64 +8) as usize]);
			state[9] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +9) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa*rowLenInt64 +9) as usize]);
			state[10] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +10) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa*rowLenInt64 +10) as usize]);
			state[11] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +11) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa*rowLenInt64 +11) as usize]);

			//Applies the reduced-round transformation f to the sponge's state
			state = reducedBlake2bLyra(state);

			//M[row][col] = M[prev][col] XOR rand
			wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + row*rowLenInt64 +0) as usize] = wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +0) as usize] ^ state[0];
			wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + row*rowLenInt64 +1) as usize] = wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +1) as usize] ^ state[1];
			wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + row*rowLenInt64 +2) as usize] = wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +2) as usize] ^ state[2];
			wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + row*rowLenInt64 +3) as usize] = wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +3) as usize] ^ state[3];
			wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + row*rowLenInt64 +4) as usize] = wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +4) as usize] ^ state[4];
			wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + row*rowLenInt64 +5) as usize] = wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +5) as usize] ^ state[5];
			wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + row*rowLenInt64 +6) as usize] = wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +6) as usize] ^ state[6];
			wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + row*rowLenInt64 +7) as usize] = wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +7) as usize] ^ state[7];
			wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + row*rowLenInt64 +8) as usize] = wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +8) as usize] ^ state[8];
			wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + row*rowLenInt64 +9) as usize] = wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +9) as usize] ^ state[9];
			wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + row*rowLenInt64 +10) as usize] = wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +10) as usize] ^ state[10];
			wholeMatrix[((nCols - _i - 1)*BLOCKLENINT64 + row*rowLenInt64 +11) as usize] = wholeMatrix[(_i*BLOCKLENINT64 + prev*rowLenInt64 +11) as usize] ^ state[11];

			//M[row*][col] = M[row*][col] XOR rotW(rand)
			wholeMatrix[(_i*BLOCKLENINT64 + rowa*rowLenInt64 +0) as usize] ^= state[11];
			wholeMatrix[(_i*BLOCKLENINT64 + rowa*rowLenInt64 +1) as usize] ^= state[0];
			wholeMatrix[(_i*BLOCKLENINT64 + rowa*rowLenInt64 +2) as usize] ^= state[1];
			wholeMatrix[(_i*BLOCKLENINT64 + rowa*rowLenInt64 +3) as usize] ^= state[2];
			wholeMatrix[(_i*BLOCKLENINT64 + rowa*rowLenInt64 +4) as usize] ^= state[3];
			wholeMatrix[(_i*BLOCKLENINT64 + rowa*rowLenInt64 +5) as usize] ^= state[4];
			wholeMatrix[(_i*BLOCKLENINT64 + rowa*rowLenInt64 +6) as usize] ^= state[5];
			wholeMatrix[(_i*BLOCKLENINT64 + rowa*rowLenInt64 +7) as usize] ^= state[6];
			wholeMatrix[(_i*BLOCKLENINT64 + rowa*rowLenInt64 +8) as usize] ^= state[7];
			wholeMatrix[(_i*BLOCKLENINT64 + rowa*rowLenInt64 +9) as usize] ^= state[8];
			wholeMatrix[(_i*BLOCKLENINT64 + rowa*rowLenInt64 +10) as usize] ^= state[9];
			wholeMatrix[(_i*BLOCKLENINT64 + rowa*rowLenInt64 +11) as usize] ^= state[10];

		}

		//updates the value of row* (deterministically picked during Setup))
		rowa = (rowa + step as u64) & (window - 1);
		//update prev: it now points to the last row ever computed
		prev = row;
		//updates row: goes to the next row to be computed
		row = row + 1;

		//Checks if all rows in the window where visited.
		if rowa == 0 {
			step = (window as i32 + gap) as i32; //changes the step: approximately doubles its value
			window *= 2;              //doubles the size of the re-visitation window
			gap = -gap;          //inverts the modifier to the step
		}
	}
	//==========================================================================/

	//============================ Wandering Phase =============================//
	row = 0; //Resets the visitation to the first row of the memory matrix
	for tau in 1..timeCost + 1 {
		//Step is approximately half the number of all rows of the memory matrix for an odd tau; otherwise, it is -1
		step = nRows as i32 /2 - 1;
		if tau%2 == 0 {
			step = -1;
		}

		let mut row0: bool = false;
		while !row0 {
			//Selects a pseudorandom index row*
			//------------------------------------------------------------------------------------------
			//rowa = ((unsigned int)state[0]) & (nRows-1);	//(USE THIS IF nRows IS A POWER OF 2)
			rowa = state[0] % nRows as u64; //(USE THIS FOR THE "GENERIC" CASE)
			//------------------------------------------------------------------------------------------

			//Performs a reduced-round duplexing operation over M[row*] XOR M[prev], updating both M[row*] and M[row]
			//reducedDuplexRow(state, memMatrix[prev], memMatrix[rowa], memMatrix[row], nCols)
			for _i in 0..nCols {
				state[0] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev * rowLenInt64 +0) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa * rowLenInt64 +0) as usize]);
				state[1] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev * rowLenInt64 +1) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa * rowLenInt64 +1) as usize]);
				state[2] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev * rowLenInt64 +2) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa * rowLenInt64 +2) as usize]);
				state[3] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev * rowLenInt64 +3) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa * rowLenInt64 +3) as usize]);
				state[4] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev * rowLenInt64 +4) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa * rowLenInt64 +4) as usize]);
				state[5] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev * rowLenInt64 +5) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa * rowLenInt64 +5) as usize]);
				state[6] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev * rowLenInt64 +6) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa * rowLenInt64 +6) as usize]);
				state[7] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev * rowLenInt64 +7) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa * rowLenInt64 +7) as usize]);
				state[8] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev * rowLenInt64 +8) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa * rowLenInt64 +8) as usize]);
				state[9] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev * rowLenInt64 +9) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa * rowLenInt64 +9) as usize]);
				state[10] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev * rowLenInt64 +10) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa * rowLenInt64 +10) as usize]);
				state[11] ^= wholeMatrix[(_i*BLOCKLENINT64 + prev * rowLenInt64 +11) as usize].wrapping_add(wholeMatrix[(_i*BLOCKLENINT64+ rowa * rowLenInt64 +11) as usize]);

				//Applies the reduced-round transformation f to the sponge's state
				state = reducedBlake2bLyra(state);

				//M[rowOut][col] = M[rowOut][col] XOR rand
				wholeMatrix[(_i*BLOCKLENINT64 + row * rowLenInt64 +0) as usize] ^= state[0];
				wholeMatrix[(_i*BLOCKLENINT64 + row * rowLenInt64 +1) as usize] ^= state[1];
				wholeMatrix[(_i*BLOCKLENINT64 + row * rowLenInt64 +2) as usize] ^= state[2];
				wholeMatrix[(_i*BLOCKLENINT64 + row * rowLenInt64 +3) as usize] ^= state[3];
				wholeMatrix[(_i*BLOCKLENINT64 + row * rowLenInt64 +4) as usize] ^= state[4];
				wholeMatrix[(_i*BLOCKLENINT64 + row * rowLenInt64 +5) as usize] ^= state[5];
				wholeMatrix[(_i*BLOCKLENINT64 + row * rowLenInt64 +6) as usize] ^= state[6];
				wholeMatrix[(_i*BLOCKLENINT64 + row * rowLenInt64 +7) as usize] ^= state[7];
				wholeMatrix[(_i*BLOCKLENINT64 + row * rowLenInt64 +8) as usize] ^= state[8];
				wholeMatrix[(_i*BLOCKLENINT64 + row * rowLenInt64 +9) as usize] ^= state[9];
				wholeMatrix[(_i*BLOCKLENINT64 + row * rowLenInt64 +10) as usize] ^= state[10];
				wholeMatrix[(_i*BLOCKLENINT64 + row * rowLenInt64 +11) as usize] ^= state[11];

				//M[rowInOut][col] = M[rowInOut][col] XOR rotW(rand)
				wholeMatrix[(_i*BLOCKLENINT64 + rowa * rowLenInt64 +0) as usize] ^= state[11];
				wholeMatrix[(_i*BLOCKLENINT64 + rowa * rowLenInt64 +1) as usize] ^= state[0];
				wholeMatrix[(_i*BLOCKLENINT64 + rowa * rowLenInt64 +2) as usize] ^= state[1];
				wholeMatrix[(_i*BLOCKLENINT64 + rowa * rowLenInt64 +3) as usize] ^= state[2];
				wholeMatrix[(_i*BLOCKLENINT64 + rowa * rowLenInt64 +4) as usize] ^= state[3];
				wholeMatrix[(_i*BLOCKLENINT64 + rowa * rowLenInt64 +5) as usize] ^= state[4];
				wholeMatrix[(_i*BLOCKLENINT64 + rowa * rowLenInt64 +6) as usize] ^= state[5];
				wholeMatrix[(_i*BLOCKLENINT64 + rowa * rowLenInt64 +7) as usize] ^= state[6];
				wholeMatrix[(_i*BLOCKLENINT64 + rowa * rowLenInt64 +8) as usize] ^= state[7];
				wholeMatrix[(_i*BLOCKLENINT64 + rowa * rowLenInt64 +9) as usize] ^= state[8];
				wholeMatrix[(_i*BLOCKLENINT64 + rowa * rowLenInt64 +10) as usize] ^= state[9];
				wholeMatrix[(_i*BLOCKLENINT64 + rowa * rowLenInt64 +11) as usize] ^= state[10];
			}

			//update prev: it now points to the last row ever computed
			prev = row;

			//updates row: goes to the next row to be computed
			//------------------------------------------------------------------------------------------
			//row = (row + step) & (nRows-1);	//(USE THIS IF nRows IS A POWER OF 2)
			row = (row + step as u64) % nRows; //(USE THIS FOR THE "GENERIC" CASE)
			//------------------------------------------------------------------------------------------
			if row == 0 {
				row0 = true;
			}
		}
	}
	//==========================================================================/

	//============================ Wrap-up Phase ===============================//
	//Absorbs the last block of the memory matrix
	let (_, mut _right) = &wholeMatrix.split_at((rowa * rowLenInt64) as usize);
	state = absorbBlock(state, _right.to_vec());
	println!("state: {:?}", state);
	//Squeezes the key
	let mut k = squeeze(state, k);
	return k;
	//==========================================================================/

}

fn main() {
	let resultcube1: Vec<u8> = "1833a9fa7cf4086bd5fda73da32e5a1d".as_bytes().to_vec();
	let resultcube2: Vec<u8> = "1833a9fa7cf4086bd5fda73da32e5a1d".as_bytes().to_vec();
	let mut lyra2result: Vec<u8> = "00000000000000000000000000000000".as_bytes().to_vec();
	let out = lyra2(lyra2result, resultcube1, resultcube2, 1, 4, 4);
	//let result = cubehash256(data);
	let mut result = Bytes::from(out);
	println!("result: {:x}", result);
	//let data: Vec<u8> = "1833a9fa7cf4086bd5fda73da32e5a1d".as_bytes().to_vec();
	//let out = Lyra2(data);
	//let result = Bytes::from(out);
	//println!("result: {:x}", result);
}
