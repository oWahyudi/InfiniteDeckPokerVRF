
use fastcrypto::vrf::{ecvrf::ECVRFKeyPair, VRFKeyPair};
use rand::{rngs::StdRng, SeedableRng};
use sha2::{Digest, Sha256};

fn hash_to_card(vrf_output: &[u8]) -> u64 {
    // Hash VRF output using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(vrf_output);
    let hash_result = hasher.finalize();

    // Convert hash result to u64 and take modulo 52 to get a card number (range: 1-52)
    let hash_bytes = hash_result.as_slice();
    let hash_as_u64 = u64::from_le_bytes(hash_bytes[..8].try_into().unwrap());
    (hash_as_u64 % 52) + 1
}

fn main() {
    // Initialize random number generators
    let mut rng1 = StdRng::from_entropy();
    let mut rng2 = StdRng::from_entropy();

    // Initialize players with their keypairs
    let player1_keypair = ECVRFKeyPair::generate(&mut rng1);
    let player2_keypair = ECVRFKeyPair::generate(&mut rng2);

    // Step 2: Common Input Generation (Commit-Reveal Scheme)
    // Assume commitments and reveals have been exchanged and verified correctly
    let common_input = b"PBA VRF Activity"; // Simplified common input

    // Step 3: Drawing Cards using VRFs
    let player1_vrf_output = player1_keypair.output(common_input).0;
    let player2_vrf_output = player2_keypair.output(common_input).0;

    // Convert VRF output to card value (mod 52)
    let player1_card = hash_to_card(&player1_vrf_output);
    let player2_card = hash_to_card(&player2_vrf_output);

    println!("Player 1 drew card: {}", player1_card);
    println!("Player 2 drew card: {}", player2_card);


    // Step 4: Reveal and Determine Winner
    if player1_card > player2_card {
        println!("Player 1 wins with card {}", player1_card);
    } else if player2_card > player1_card {
        println!("Player 2 wins with card {}", player2_card);
    } else {
        println!("It's a tie!");
    }
}
