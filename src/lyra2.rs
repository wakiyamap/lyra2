use byteorder::{ByteOrder, LittleEndian};
//use bytes::{Bytes, BufMut};

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
//fn squeeze(state: [u64; 16], out: Vec<u8>) {
//	tmp := make([]byte, blockLenBytes)
//	let mut _j = 0;
//	let mut jmax = out.len()/blockLenBytes+1;
//	for _j in 0..imax {
//		let mut _i = 0;
//		for _i in 0..BLOCKLENINT64 {
//			binary.LittleEndian.PutUint64(tmp[i*8:], state[i])
//		}
//		copy(out[j*blockLenBytes:], tmp) //be care in case of len(out[i:])<len(tmp)
//		blake2bLyra(state)
//	}
//}

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

/**
 * reducedSqueezeRow0 erforms a reduced squeeze operation for a single row, from the highest to
 * the lowest index, using the reduced-round Blake2b's G function as the
 * internal permutation
 *
 * @param state     The current state of the sponge
 * @param rowOut    Row to receive the data squeezed
 */
fn reducedSqueezeRow0(mut state: [u64; 16], rowOut: Vec<u64>, nCols: u64) {
	let mut ptr = (nCols - 1) * BLOCKLENINT64;
	//M[row][C-1-col] = H.reduced_squeeze()
	for _i in 0..nCols {
		//let (_, mut _right) = &rowOut.split_at(ptr);
		//ptrWord = _right as usize; //In Lyra2: pointer to M[0][C-1]
//		ptrWord[0] = state[0];
//		ptrWord[1] = state[1];
//		ptrWord[2] = state[2];
//		ptrWord[3] = state[3];
//		ptrWord[4] = state[4];
//		ptrWord[5] = state[5];
//		ptrWord[6] = state[6];
//		ptrWord[7] = state[7];
//		ptrWord[8] = state[8];
//		ptrWord[9] = state[9];
//		ptrWord[10] = state[10];
//		ptrWord[11] = state[11];

		//Goes to next block (column) that will receive the squeezed data
		//ptr -= BLOCKLENINT64;

		//Applies the reduced-round transformation f to the sponge's state
		state = reducedBlake2bLyra(state);
	}
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
fn lyra2(k: Vec<u8>, pwd: Vec<u8>, salt: Vec<u8>, timeCost: u64, nRows: u64, nCols: u64) {

	//============================= Basic variables ============================//
	let mut row: i32 = 2;              //index of row to be processed
	let mut prev: i32 = 1;             //index of prev (last row ever computed/modified)
	let mut rowa: u64;       //index of row* (a previous row, deterministically picked during Setup and randomly picked while Wandering)
	let mut tau: u64;        //Time Loop iterator
	let mut step: i32 = 1;             //Visitation step (used during Setup and Wandering phases)
	let mut window: u64 = 2; //Visitation window (used to define which rows can be revisited during Setup)
	let mut gap: u64 = 1;    //Modifier to the step, assuming the values 1 or -1
	let mut i: i32;             //auxiliary iteration counter
	//==========================================================================/

	//========== Initializing the Memory Matrix and pointers to it =============//
	//Tries to allocate enough space for the whole memory matrix

	let mut rowLenInt64: u64 = BLOCKLENINT64 * nCols;
	let mut i : u64 = nRows * rowLenInt64;
	let mut wholeMatrix: Vec<u64> = Vec::new();
	wholeMatrix.resize(i as usize, 0);
	let mut wholeMatrixLen : usize = i as usize;

	let mut memMatrix: Vec<Vec<u64>> = Vec::new();
	memMatrix.resize(nRows as usize, Vec::new());
	let mut _i : usize = 0;
	let mut ptrWord = 0;
	let mut _nRows : usize = nRows as usize;
	for _i in 0.._nRows {
		memMatrix[_i].resize(wholeMatrixLen - ptrWord, 0);
		ptrWord += rowLenInt64 as usize;
	}

//	let mut rowLenInt64: u64 = BLOCKLENINT64 * nCols;
//	//rowLenBytes := rowLenInt64 * 8
//
//	let mut i : u64 = nRows * rowLenInt64;
//	let mut wholeMatrix: Vec<u64> = Vec::new();
//	wholeMatrix.resize(i as usize, 0);
//	//Allocates pointers to each row of the matrix
//	let mut memMatrix: Vec<Vec<u64>> = Vec::new();
//	memMatrix.resize(nRows as usize, &Vec::new());
//
//	//Places the pointers in the correct positions
//	let mut ptrWord = 0;
//	let mut _i : usize = 0;
//	for _i in 0..nRows as usize {
//		memMatrix[_i] = &wholeMatrix[..ptrWord];
//		ptrWord += rowLenInt64 as usize;
//		println!("s0: {:?}", &wholeMatrix[..ptrWord]);
//	}
	println!("s1: {:?}", memMatrix);
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
	println!("s1: {:?}", memMatrix);
	ptrByte += pwd.len() as u64 / 8;

	//Concatenates the salt
	let mut _right = &salt;
	for _j in 0..salt.len()/8 {
		let (_, mut _right) = &pwd.split_at(8*_j);
		wholeMatrix[ptrByte as usize +_j] = LittleEndian::read_u64(_right);
	}
	println!("s1: {:?}", memMatrix);
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
	//println!("result: {:?}", memMatrix[0]);
	//memMatrix[0] = wholeMatrix;
	let mut memM = memMatrix[0].clone();
	println!("s1: {:?}", state);
	println!("m1: {:?}", memM[0]);
	println!("m3: {:?}", %memMatrix[0]);
	reducedSqueezeRow0(state, memM, nCols); //The locally copied password is most likely overwritten here
	//reducedDuplexRow1(state, memMatrix[0], memMatrix[1], nCols)
	println!("result: {:?}", state);
}

fn main() {
	let resultcube1: Vec<u8> = "1833a9fa7cf4086bd5fda73da32e5a1d".as_bytes().to_vec();
	let resultcube2: Vec<u8> = "1833a9fa7cf4086bd5fda73da32e5a1d".as_bytes().to_vec();
	let mut lyra2result: Vec<u8> = "00000000000000000000000000000000".as_bytes().to_vec();
	lyra2(lyra2result, resultcube1, resultcube2, 1, 4, 4);
	//let result = cubehash256(data);
	//println!("result: {:x}", result);
	//let data: Vec<u8> = "1833a9fa7cf4086bd5fda73da32e5a1d".as_bytes().to_vec();
	//let out = Lyra2(data);
	//let result = Bytes::from(out);
	//println!("result: {:x}", result);
}
