use std::path::Path;

use anyhow::{ensure, Result};

use filecoin_proofs_v1::types::{EmptySectorUpdateProof, MerkleTreeTrait};
use filecoin_proofs_v1::{with_shape, TreeRHasher};

use crate::{Commitment, PieceInfo, RegisteredUpdateProof};

pub fn empty_sector_update_encode_into_inner<
    Tree: 'static + MerkleTreeTrait<Hasher = TreeRHasher>,
>(
    registered_proof: RegisteredUpdateProof,
    new_replica_path: &Path,
    new_cache_path: &Path,
    sector_key_path: &Path,
    sector_key_cache_path: &Path,
    staged_data_path: &Path,
    piece_infos: &[PieceInfo],
) -> Result<(Commitment, Commitment, Commitment)> {
    ensure!(
        registered_proof.major_version() == 1,
        "unusupported version"
    );

    let config = registered_proof.as_v1_config();

    filecoin_proofs_v1::encode_into::<Tree>(
        config,
        new_replica_path,
        new_cache_path,
        sector_key_path,
        sector_key_cache_path,
        staged_data_path,
        piece_infos,
    )
}

/// Encodes data into an existing replica.
/// Returns tuple of (comm_r_new, comm_r_last_new, comm_d_new)
pub fn empty_sector_update_encode_into<R, S, T, U, V>(
    registered_proof: RegisteredUpdateProof,
    new_replica_path: R,
    new_cache_path: S,
    sector_key_path: T,
    sector_key_cache_path: U,
    staged_data_path: V,
    piece_infos: &[PieceInfo],
) -> Result<(Commitment, Commitment, Commitment)>
where
    R: AsRef<Path>,
    S: AsRef<Path>,
    T: AsRef<Path>,
    U: AsRef<Path>,
    V: AsRef<Path>,
{
    ensure!(
        registered_proof.major_version() == 1,
        "unusupported version"
    );

    with_shape!(
        u64::from(registered_proof.sector_size()),
        empty_sector_update_encode_into_inner,
        registered_proof,
        new_replica_path.as_ref(),
        new_cache_path.as_ref(),
        sector_key_path.as_ref(),
        sector_key_cache_path.as_ref(),
        staged_data_path.as_ref(),
        piece_infos,
    )
}

pub fn empty_sector_update_decode_from_inner<
    Tree: 'static + MerkleTreeTrait<Hasher = TreeRHasher>,
>(
    registered_proof: RegisteredUpdateProof,
    out_data_path: &Path,
    replica_path: &Path,
    sector_key_path: &Path,
    sector_key_cache_path: &Path,
    comm_d_new: Commitment,
) -> Result<()> {
    ensure!(
        registered_proof.major_version() == 1,
        "unusupported version"
    );

    let config = registered_proof.as_v1_config();

    filecoin_proofs_v1::decode_from::<Tree>(
        config,
        out_data_path,
        replica_path,
        sector_key_path,
        sector_key_cache_path,
        comm_d_new,
    )
}

/// Reverses the encoding process and outputs the data into out_data_path.
pub fn empty_sector_update_decode_from<R, S, T, U>(
    registered_proof: RegisteredUpdateProof,
    out_data_path: R,
    replica_path: S,
    sector_key_path: T,
    sector_key_cache_path: U,
    comm_d_new: Commitment,
) -> Result<()>
where
    R: AsRef<Path>,
    S: AsRef<Path>,
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    ensure!(
        registered_proof.major_version() == 1,
        "unusupported version"
    );

    with_shape!(
        u64::from(registered_proof.sector_size()),
        empty_sector_update_decode_from_inner,
        registered_proof,
        out_data_path.as_ref(),
        replica_path.as_ref(),
        sector_key_path.as_ref(),
        sector_key_cache_path.as_ref(),
        comm_d_new,
    )
}

pub fn empty_sector_update_remove_encoded_data_inner<
    Tree: 'static + MerkleTreeTrait<Hasher = TreeRHasher>,
>(
    registered_proof: RegisteredUpdateProof,
    sector_key_path: &Path,
    sector_key_cache_path: &Path,
    replica_path: &Path,
    replica_cache_path: &Path,
    data_path: &Path,
    comm_d_new: Commitment,
) -> Result<()> {
    ensure!(
        registered_proof.major_version() == 1,
        "unusupported version"
    );

    let config = registered_proof.as_v1_config();

    filecoin_proofs_v1::remove_encoded_data::<Tree>(
        config,
        sector_key_path,
        sector_key_cache_path,
        replica_path,
        replica_cache_path,
        data_path,
        comm_d_new,
    )
}

/// Removes encoded data and outputs the sector key.
pub fn empty_sector_update_remove_encoded_data<R, S, T, U, V>(
    registered_proof: RegisteredUpdateProof,
    sector_key_path: R,
    sector_key_cache_path: S,
    replica_path: T,
    replica_cache_path: U,
    data_path: V,
    comm_d_new: Commitment,
) -> Result<()>
where
    R: AsRef<Path>,
    S: AsRef<Path>,
    T: AsRef<Path>,
    U: AsRef<Path>,
    V: AsRef<Path>,
{
    ensure!(
        registered_proof.major_version() == 1,
        "unusupported version"
    );

    with_shape!(
        u64::from(registered_proof.sector_size()),
        empty_sector_update_remove_encoded_data_inner,
        registered_proof,
        sector_key_path.as_ref(),
        sector_key_cache_path.as_ref(),
        replica_path.as_ref(),
        replica_cache_path.as_ref(),
        data_path.as_ref(),
        comm_d_new,
    )
}

pub fn generate_empty_sector_update_proof_inner<
    Tree: 'static + MerkleTreeTrait<Hasher = TreeRHasher>,
>(
    registered_proof: RegisteredUpdateProof,
    comm_r_old: Commitment,
    comm_r_new: Commitment,
    comm_d_new: Commitment,
    sector_key_path: &Path,
    sector_key_cache_path: &Path,
    replica_path: &Path,
    replica_cache_path: &Path,
) -> Result<EmptySectorUpdateProof> {
    ensure!(
        registered_proof.major_version() == 1,
        "unusupported version"
    );

    let config = registered_proof.as_v1_config();

    filecoin_proofs_v1::generate_empty_sector_update_proof::<Tree>(
        config,
        comm_r_old,
        comm_r_new,
        comm_d_new,
        sector_key_path,
        sector_key_cache_path,
        replica_path,
        replica_cache_path,
    )
}

pub fn generate_empty_sector_update_proof<R, S, T, U>(
    registered_proof: RegisteredUpdateProof,
    comm_r_old: Commitment,
    comm_r_new: Commitment,
    comm_d_new: Commitment,
    sector_key_path: R,
    sector_key_cache_path: S,
    replica_path: T,
    replica_cache_path: U,
) -> Result<EmptySectorUpdateProof>
where
    R: AsRef<Path>,
    S: AsRef<Path>,
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    ensure!(
        registered_proof.major_version() == 1,
        "unusupported version"
    );

    with_shape!(
        u64::from(registered_proof.sector_size()),
        generate_empty_sector_update_proof_inner,
        registered_proof,
        comm_r_old,
        comm_r_new,
        comm_d_new,
        sector_key_path.as_ref(),
        sector_key_cache_path.as_ref(),
        replica_path.as_ref(),
        replica_cache_path.as_ref(),
    )
}

pub fn verify_empty_sector_update_proof_inner<
    Tree: 'static + MerkleTreeTrait<Hasher = TreeRHasher>,
>(
    registered_proof: RegisteredUpdateProof,
    proof: &[u8],
    comm_r_old: Commitment,
    comm_r_new: Commitment,
    comm_d_new: Commitment,
    sector_key_cache_path: &Path,
) -> Result<bool> {
    ensure!(
        registered_proof.major_version() == 1,
        "unusupported version"
    );

    let config = registered_proof.as_v1_config();

    filecoin_proofs_v1::verify_empty_sector_update_proof::<Tree>(
        config,
        proof,
        comm_r_old,
        comm_r_new,
        comm_d_new,
        sector_key_cache_path,
    )
}

pub fn verify_empty_sector_update_proof<R>(
    registered_proof: RegisteredUpdateProof,
    proof: &[u8],
    comm_r_old: Commitment,
    comm_r_new: Commitment,
    comm_d_new: Commitment,
    sector_key_cache_path: R,
) -> Result<bool>
where
    R: AsRef<Path>,
{
    ensure!(
        registered_proof.major_version() == 1,
        "unusupported version"
    );

    with_shape!(
        u64::from(registered_proof.sector_size()),
        verify_empty_sector_update_proof_inner,
        registered_proof,
        proof,
        comm_r_old,
        comm_r_new,
        comm_d_new,
        sector_key_cache_path.as_ref(),
    )
}
