use blake3::Hasher;
use serde::{Deserialize, Serialize};

/// User's data to transfer from Worldcoin App to Orb.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    /// User's key stored in the app in the PEM public key format.
    pub user_public_key: String,
    /// User's biometric data policy.
    pub data_policy: DataPolicy,
}

/// User's biometric data policy. Part of [`UserData`].
#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Debug, Default)]
#[repr(u8)]
pub enum DataPolicy {
    /// No images should be transmitted from the Orb.
    #[default]
    OptOut,
    /// Research and remote custody.
    FullDataOptIn,
}

impl UserData {
    /// Returns `true` if `hash` is a BLAKE3 hash of this [`UserData`].
    ///
    /// This method calculates its own hash of the same length as the input
    /// `hash` and checks if both hashes are identical.
    pub fn verify(&self, hash: impl AsRef<[u8]>) -> bool {
        let external_hash = hash.as_ref();
        let internal_hash = self.hash(external_hash.len());
        external_hash == internal_hash
    }

    /// Calculates a BLAKE3 hash of the length `n`.
    pub fn hash(&self, n: usize) -> Vec<u8> {
        let mut hasher = Hasher::new();
        self.hasher_update(&mut hasher);
        let mut output = vec![0; n];
        hasher.finalize_xof().fill(&mut output);
        output
    }

    // This method must hash every field.
    fn hasher_update(&self, hasher: &mut Hasher) {
        let Self {
            user_public_key,
            data_policy,
        } = self;
        hasher.update(user_public_key.as_bytes());
        hasher.update(&[*data_policy as u8]);
    }
}

impl DataPolicy {
    /// Whether the policy is opt-in.
    #[must_use]
    pub fn is_opt_in(self) -> bool {
        match self {
            Self::OptOut => false,
            Self::FullDataOptIn => true,
        }
    }
}

impl ToString for DataPolicy {
    fn to_string(&self) -> String {
        match self {
            DataPolicy::FullDataOptIn => "full_data_opt_in".to_string(),
            DataPolicy::OptOut => "opt_out".to_string(),
        }
    }
}
