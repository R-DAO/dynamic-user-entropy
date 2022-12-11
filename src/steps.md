ElGamal asymetric encryption: 

The properties of the keys are as follows: 

Step 1: Let p be a 1024-bit prime and
Step 2: q be a 160-bit prime 

Step 2a: such that q | (p − 1). 

Step 3: Let g ∈ Zp be our generator. 

Step 4: For any private key x ∈ Z∗q the public key can be obtained as follows: 

Step 5: y = gx mod p. 

Step 6: To encrypt a message m, pick a random entropy coefficient e ∈ Z∗q and let E(m) = (ge, mye). 

Step 7: To decrypt a ciphertext (a, b), compute bax = m. 

Step 8: From this, it is trivial to also show that if 
(a1, b1) and (a2, b2) and the ciphertexts of m1 and m2, E(m1m2) = (a1a2, b1b2), 

meaning that ElGamal is homorphic, allowing us
to perform mathematical operations on data without revealing it.
