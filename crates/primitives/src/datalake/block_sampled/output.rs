//! This module defines the types used in the block sampled datalake.

use serde::{Deserialize, Serialize};

use crate::datalake::output::{
    hex_to_8_byte_chunks_little_endian, split_little_endian_hex_into_parts,
    CairoFormattedChunkResult, MPTProof, MPTProofFormatted, Uint256,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Account {
    pub address: String,
    // U256 type
    pub account_key: String,
    pub proofs: Vec<MPTProof>,
}

impl Account {
    pub(crate) fn to_cairo_format(&self) -> AccountFormatted {
        let address_chunk_result = hex_to_8_byte_chunks_little_endian(&self.address);
        let account_key = split_little_endian_hex_into_parts(&self.account_key);
        let proofs = self
            .proofs
            .iter()
            .map(|proof| {
                let proof_chunk_result: Vec<CairoFormattedChunkResult> = proof
                    .proof
                    .iter()
                    .map(|proof| hex_to_8_byte_chunks_little_endian(proof))
                    .collect();

                let proof_bytes_len = proof_chunk_result.iter().map(|x| x.chunks_len).collect();
                let proof_result: Vec<Vec<String>> = proof_chunk_result
                    .iter()
                    .map(|x| x.chunks.clone())
                    .collect();

                MPTProofFormatted {
                    block_number: proof.block_number,
                    proof_bytes_len,
                    proof: proof_result,
                }
            })
            .collect();
        AccountFormatted {
            address: address_chunk_result.chunks,
            account_key,
            proofs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub(crate) struct AccountFormatted {
    pub address: Vec<String>,
    pub account_key: Uint256,
    pub proofs: Vec<MPTProofFormatted>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Storage {
    pub address: String,
    pub slot: String,
    // U256 type
    pub storage_key: String,
    pub proofs: Vec<MPTProof>,
}

impl Storage {
    pub(crate) fn to_cairo_format(&self) -> StorageFormatted {
        let address_chunk_result = hex_to_8_byte_chunks_little_endian(&self.address);
        let slot_chunk_result = hex_to_8_byte_chunks_little_endian(&self.slot);
        let storage_key = split_little_endian_hex_into_parts(&self.storage_key);
        let proofs = self
            .proofs
            .iter()
            .map(|proof| {
                // process storage proof
                let storage_proof_chunk_result: Vec<CairoFormattedChunkResult> = proof
                    .proof
                    .iter()
                    .map(|proof| hex_to_8_byte_chunks_little_endian(proof))
                    .collect();
                let storage_proof_bytes_len = storage_proof_chunk_result
                    .iter()
                    .map(|x| x.chunks_len)
                    .collect();
                let storage_proof_result: Vec<Vec<String>> = storage_proof_chunk_result
                    .iter()
                    .map(|x| x.chunks.clone())
                    .collect();

                MPTProofFormatted {
                    block_number: proof.block_number,
                    proof_bytes_len: storage_proof_bytes_len,
                    proof: storage_proof_result,
                }
            })
            .collect();
        StorageFormatted {
            address: address_chunk_result.chunks,
            slot: slot_chunk_result.chunks,
            storage_key,
            proofs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub(crate) struct StorageFormatted {
    // chunked address
    pub address: Vec<String>,
    // chunked storage slot
    pub slot: Vec<String>,
    // keccak(slot) as uint256
    pub storage_key: Uint256,
    pub proofs: Vec<MPTProofFormatted>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datalake::output::*;

    #[test]
    fn cairo_format_header() {
        let original_header = Header{
            rlp: "f90226a018a6770e7e502f9209082c676922bbf1ad4f984924a17743d3044e6b3ffd8f19a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347947cbd790123255d9467d22baa806c9f059e558dc1a0156be497b45c06194d49508c8dca1ecef038ab4d3bd6060de6cfa2c9a4c3591ca0dcf5dc08c6e2720af2576fad9b9cccc66c0b50e53ebdd946bf0529ea750acb27a0d365f953867eadc22b2b2ded7cd620d92214e06671fd95e4f4d0b4747a4d2906b901000040020a0900000206083c210411006001d1080000040000001800a48100083040001000e00102090013424000844400000004004800020030144004a0600820448001000821811080002108880408100000404001140a1000004c004080020a280280280a108000025800044a044903800914004080000000c04015980109800022000002018804242400200a004a00000000201208804808001000c652088103080400100000060c00000000001000100022800a18000a2034a200040200010000013e000030000510000020020401004001100088000052008e0345802b0828b0005000a0011201022002808420402401000020001000820022400840081080834b90248401c9c380838ef3b5846588daac856c696e7578a03310d07ba1b9123c44429746f84d32df7e725178ae2c66404a3afad502c0a402880000000000000000849ac020c3a01e922a1e8e795414af0458d9af8d1fa08f5365cb4efb05273c3004b882cd3c84".to_string(),
            proof: HeaderProof{
                leaf_idx: 56993,
                mmr_path: vec!["0x4f582f7c3e936d25c2979f6c473278c17fb4c1cc02b5dc27b8226d41135fc9c".to_string()]
            }
        };

        let formatted_header = original_header.to_cairo_format();

        assert_eq!(
            formatted_header.rlp,
            vec![
                "0xe77a618a02602f9",
                "0x672c0809922f507e",
                "0x49984fadf1bb2269",
                "0x6b4e04d34377a124",
                "0x4dcc1da0198ffd3f",
                "0xb585ab7a5dc7dee8",
                "0x4512d31ad4ccb667",
                "0x42a1f013748a941b",
                "0xbd7c944793d440fd",
                "0xd267945d25230179",
                "0x559e059f6c80aa2b",
                "0xb497e46b15a0c18d",
                "0x8d8c50494d19065c",
                "0x3b4dab38f0ce1eca",
                "0xa4c9a2cfe60d06d6",
                "0x8dcf5dca01c59c3",
                "0xad6f57f20a72e2c6",
                "0xe5500b6cc6cc9c9b",
                "0xea2905bf46d9bd3e",
                "0xf965d3a027cb0a75",
                "0x2d2b2bc2ad7e8653",
                "0xe01422d920d67ced",
                "0xb4d0f4e495fd7166",
                "0x1b906294d7a74",
                "0x20000090a024000",
                "0x60001104213c0806",
                "0x4000008d101",
                "0x30080081a4001800",
                "0x90201e000100040",
                "0x44840040421300",
                "0x2004800040000",
                "0x200860a004401430",
                "0x1081210800018044",
                "0x1008048808210080",
                "0x100a140140400000",
                "0xa028040004c0000",
                "0x80100a28800228",
                "0x349044a04005802",
                "0x804000140980",
                "0x800901981540c000",
                "0x488010200002200",
                "0x4a000a20002424",
                "0x4880081220000000",
                "0x810852c600100008",
                "0x600001000040803",
                "0x1000000000000c",
                "0xa00180a80220010",
                "0x100020400a23420",
                "0x3000003e010000",
                "0x104022000001005",
                "0x880010014000",
                "0x82b8045038e0052",
                "0x1201a0005000b028",
                "0x4020848002200201",
                "0x10002000000124",
                "0x1008400840220082",
                "0xc9018424904b8380",
                "0x6584b5f38e8380c3",
                "0x756e696c85acda88",
                "0xb9a17bd01033a078",
                "0x4df8469742443c12",
                "0x2cae7851727edf32",
                "0xc002d5fa3a4a4066",
                "0x8802a4",
                "0xc320c09a84000000",
                "0x54798e1e2a921ea0",
                "0x1f8dafd95804af14",
                "0x5fb4ecb65538fa0",
                "0x3ccd82b804303c27",
                "0x84"
            ]
        );

        assert_eq!(formatted_header.rlp_bytes_len, 553)
    }

    #[test]
    fn cairo_format_account() {
        let original_account = Account {
            address: "0x7f2c6f930306d3aa736b3a6c6a98f512f74036d4".to_string(),
            account_key: "0x4ee516ed41ff168cfccb34c4efa2db7e4f369c363cf9480dc12886f2b6fb82a5".to_string(),
            proofs: vec![
                MPTProof {
                    block_number: 4952100,
                    proof: vec![
                      "0xf90211a0b260487078406bef4549faf0cf3b8689f38e132759b79f1b38f921bb0770725ba0f07c30281fed0e6c948d671b34a5c17926f19abaf816de01df4df13b2f21da0fa0841f208d25bb1a776e3c2171b6480a0c724d3dfda547b2640a44cc588e070b0ea06d0c3d0c26e0d6445ecb5b6a33fcf689223e7730ea633c10b3da596b95e0f1b2a00bf1fd1401f66489c80505a599832504e7ac4b21aa9c9d112773bcfc27092457a0954a14a3720833725342d37906ae2deea600f972e3be67bdfb9b2d9edb6c1591a041272f02efe0185e5efff8a544069f9e4a95ea84529bc19f1e4c2eeb1eac570aa060ca91f51e2cf585b712149c5a445bfac9538702f0d3426bee60aefa4c87ecbfa0cdba627f2918168968ec9b36b4de19bd296d5ac150c1a9b0f9c5fe16d2124cdda02d3691c76b294a895c7d87f1d6fdc9277f2535aadc78490e0d079a3e35659f1ea0bda5248f65638b83926e03c0d51a48620f41350e586285f895bbe1b22dfed40ea0758672f5a0bf7dcc32d1f39bacf1cc3e91a1152d3b8fefe0196b0c84e951a3ada0270bab342f3d7067df0a85afd721081abe2f9ba381e11c4d7696e3ea8374e781a0ebe3b5f3fce25e553b5d58f6fe8e1ca2bb755c36bb676fadcd2cafad96bcb33da047b0214744d7948e4006726fd5d457b1f161c39009214f65eb8294b334e6892ca01220abfad74e06651e36c1cc9f8c4f969bc6405f427808b0b59a8a536e95861780".to_string(),
                      "0xf90211a0a954661b9074f2aaa86f30dd72090c661033ea23bd53d5f4f81868f52fa36244a01796443bb911da9cf09b45140ced3c3b29a7ec3a718d064048309e080c181794a0c985eb0a0e53c4b2263b82e139bb99ab8734afe19f83a7c97d5fb5b5fc06a99ba0d68c13a7b9e4b19c3ef8c9c1ff28e79e6aa010f8c6d35732fb4ce94e48f414e9a02f4562fc142d982ff694d94cb140f320bee079f647061a89d867678cdca6dc9fa08c5cf6bec1e86b386077b2a601054b68c0e16da54f30ec4f997ff867072ff8d2a063f36f3b4d020c5662868715f4a794d12b9e35f2fe0839b0f53b5ca8b34026cea0958229737a3c3b26d0f7724a5f8b9c3f4aa198b8a5df96cd145eed5a1731f406a086f0a5107a6285864856d06c9fe42a852c9820ca3ecd114215a339ed12b9a97fa085670a6d72ee17acfc1a19550001596930bcfd8c5c52f18960bda455c39feef1a0cf03b39cf61db940a99e655a2ce1c9d8658e42d0f7340d414a8cdcadf7abb1bea07e61ec23442705596ce9886a93ed7d827478189654982650619160630fa68119a002eab370c4f7a0075bc748d614d2642402672b47b441a402dc0d941ebc962d30a06c9bc270052090f0ede819cd4858012659e2b12bb76400f7f501afa19ca38f25a0abb7bda7ae949eb2d0a8aca07f02edcb45cade684078b75ada26f76e1d574c73a045ba9374b98c8f1336f76608f91f4e984dab978dac5447d3fdbc0d1fb43c694e80".to_string(),
                      "0xf90211a0310581eee06b354207627fa84f8b7446bea7a72d57cace6c728d1e858ccfe63aa0a7a547cafa9c44fdfb89cc146d8c7eb24426186563caebef1f5221a57174f00ca0253fa6bbcebbbd65a7f42d7df002e806116f357a883a138e50d500421c3ec949a01cbaee38a8329511ea697563a730a82c18d25f2fd8084a3bfae6bd7104260c8da01b5a60992ab62791cae894a6f7a49c08199ee555de60e005f7d1f75fe4d6de6ba0a6ab5ded2187ff4c89752775bbadb7a053c5bcf3dc979f447bc0290961729aeaa0de9b27b305f034f73f618f572e67a05acc8bddc5fa94c07157d20fa988129704a0d546a721d807a2d15da202a60b3abaa40952a9da4f64aa09b27bc28dae814afba05d7cc04107956e1b0da8bfec0addcbe0ddecbb24f53b58a0fe4b009a3bae0b3fa0833db77f8fe4133aae2f0239cb6e3da14cd882f412caf4c70f1ef32e9a2d3232a0dab5929f9adabf64c7e37c545c1bb85c8d4d8ddee3a8357c698312abce225581a0d19b61dcbee08cef44ea3561e4338c725f8a10c105010a4ec5339565277251a9a077c2f9761ff69043b057da256b511a480d36ddd9114daa50d58ea0eb545307c9a0006475af74e7e0f09f64d153f07e942778d4ef0b1f0782733f5bee2a3fb6cd20a0aa1929dd092358f3579f527dd23b1b8920b2275c5a7dfefe53ae4f1fc7c175f5a03d4d663b89b086103333453e892cfdfd53213c5f68b2be25ff6fcf601cbb953180".to_string(),
                      "0xf90211a01313d4d949f1c6d765b380c85167c36ddd4a50a1a3a5632206807c5fbb84b8eaa0017174652da6eeb6b1ced2348401322a4ae8a64958e0985f464d1f6cec54ac25a0b7afe535d15307c31d59e46ab82a46d088da1fb28c6b8fbd401ad0f00ba2b10aa0294d97b0f709fcd8217b194674c3bb1e72f1ac8a72157a9ddee31c3acc0d6c32a0653cd435f791aa09ed6d90792a44b3a8204be0ff7af542e5982a784a6c6dcd67a0eb0e5f713b28a0b7dab330813d618d171a2fbea462b8eb48988f9988b0622b26a04c560f056b670c4369f09d48cfe5b64abdd71ec834270859e7d6919833ec0461a02cf718dd9f2f1e81b14d486fa0f2aa147b53a0c221179a2337a4f417ae8d63bda0987e4ef0bc43132f4fc8656df6d3a46c8f455c4001bbce8aa95b76fbf60eb388a061fa7b1646e130b0834e828c17097e4fa8795688ebc9e72bb557bc2a2854d2eaa03dc26feadef3e60cb1b32ae3ca20985d2fa7fda872157083287ff47c9d9a5438a0f0d76fc7d874c2ed155faa3f1f4b752cd24461e4eb17fe47d050acc107d9f317a0092abbbc3bdc3a9bb5c5d8e748cfb5054b72c03af1604e30caf7e97950dde889a0ddf1f5166efd8afda29144efe533bd454180247277067acac325501a454fa3e8a0dc601283681eb2a758b9271efa8a63eaae1df82613d45759cf425cfc54c840b6a0b6984c5746a144a9560b1df1723fb8aba697c17804b1ef01583cb51fc88dc30c80".to_string(),
                      "0xf90211a0e270c90cb49f72d776b8b15fbc4c1af480f1dd535d77896f624af8c2988b3607a0e2352b3689f57a39b887fb660e044159234423e620a59be04a55149e0c66b1cda04ff631d4515ec8577655836e399d596cfffa3bf5b2dcf75c96cf547853369d8ca0ede6659948cdb9a039c37a5ffb9a61c19635a6f668773cb51742d68bb46ddeffa05fac9e7f4107ab42660f707eec0880374ff8c1b050d7703c117a1cfe7decdff8a01f7c71dea2af699e513ca602e4663a8637637eedbf705b4e70e1763145f3c297a0fa0a75e221c480ad81a80efdf7f5861c4bf0b83421c50ce4d700192ebef6fb77a0c12ff9a99d8fcd3a7c69623a0cde48200024f13fea08b18e4b6d9fba8646f2c7a02e92386cbf8d5f7b2650836ba619505bf8bf501f99b9e64a103ff44b5f142e01a00de2306019f2caf07ae25444e6654a8354cc7247ef75abfac7701e1275e83e6ea0f25b7cd356d3da6922bc3f761e78118f9b19e0e610fc00381715695e5b757275a09fb473b88b52d4f9c0f705faa6ba416c0b42d60a0a20a248157febf1adf59f7fa028ee2e741ba400aae27205e3232e26137940f5d63f51a145745cae1b14aa3508a0c5843b9d96bca482fed2314387363c03025943deb30397335e88fe585b3d6c65a089968fa3749452a13d5e4b1b4c92330f0f21ac68123c8704e06675c03e21e3a5a031c1f51dc4d740a7d25e55e660f7063fb2f0583c74df7413f2240d4b02c3958f80".to_string(),
                      "0xf901918080a05ad8f58c5eeb611212582513105c3967f2be0c437c163e0a500f3e99376a1d0d8080a07039ebe73cd8e4a725451e6a9353314ec4b7a0899b79c192404bae1f13d8b8f9a0423c1e5c930fa044f5cfb134d6beffab09fb41d294a297394799ae06574b72c8a020a06be73b79a2293abc6e08b110a2dfe44804f705ff5c1220c170c927515b19a079fbc4003101e9c9b417f7cc4ca45621de54380d7302cf4ce1f2030cfacdf6eaa0cf60aebe174d15925f304a8226f6162b32a4f817f3153c9db02b359f95496bd6a0ce698f9fed05871cd65245e01ec8a4d1e50361b0e1b51468a93f12f19ce8ba6da01ec3512523b23d000cfd0ff5eaf77787223de5fc55dd3cda8d3792634a037977a08bc0a97065188cd5310ebceb9a17200651e6c767c47f047735404284c09b8c58a0864ff5e9ad3b6a76602e5575c52f9ec123679f7509451fa98cb2e152081a21b5a0a6d5f12320ebfce67e1d97ff2ff6cd21b8e83a97fb39460496828dfbfaa5361ea0045d68895435f0e949bced85f319c36da7e6a2ea3fd5ff45632284f26ddfe79a80".to_string(),
                      "0xf851808080808080808080a07b3c71cc818328815c79bcd344c717789bde929b23bf30bfe28a36ca3cad72cd80808080a0b0e04f694ef56d458c3c2fa191aa95862a20a393419a2f1fbe009e75864c662d8080".to_string(),
                      "0xf8719d3d41ff168cfccb34c4efa2db7e4f369c363cf9480dc12886f2b6fb82a5b851f84f821a78890242aa8ffb4eba0bc4a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470".to_string()
                    ]
                  }]
        };

        let formatted_account = original_account.to_cairo_format();

        assert_eq!(
            formatted_account.address,
            vec!["0xaad30603936f2c7f", "0x12f5986a6c3a6b73", "0xd43640f7"]
        );

        assert_eq!(
            formatted_account.proofs[0].proof,
            vec![
                vec![
                    "0x704860b2a01102f9",
                    "0xf0fa4945ef6b4078",
                    "0x27138ef389863bcf",
                    "0xbb21f9381b9fb759",
                    "0x307cf0a05b727007",
                    "0x678d946c0eed1f28",
                    "0x9af12679c1a5341b",
                    "0xf14ddf01de16f8ba",
                    "0x1f84a00fda212f3b",
                    "0x3c6e771abb258d20",
                    "0x4d720c0a48b67121",
                    "0x440a64b247a5fd3d",
                    "0x6da00e0b078e58cc",
                    "0x5e44d6e0260c3d0c",
                    "0x2289f6fc336a5bcb",
                    "0xb3103c63ea30773e",
                    "0xa0b2f1e0956b59da",
                    "0x8964f60114fdf10b",
                    "0x4258399a50505c8",
                    "0x119d9caa214bace7",
                    "0x57240927fcbc7327",
                    "0x330872a3144a95a0",
                    "0x2dae0679d3425372",
                    "0x67bee372f900a6ee",
                    "0x156cdb9e2d9bfbbd",
                    "0xe0ef022f2741a091",
                    "0x644a5f8ff5e5e18",
                    "0x9b5284ea954a9e9f",
                    "0xac1eeb2e4c1e9fc1",
                    "0x1ef591ca60a00a57",
                    "0x5a9c1412b785f52c",
                    "0xf0028753c9fa5b44",
                    "0x4cfaae60ee6b42d3",
                    "0x7f62bacda0bfec87",
                    "0x369bec6889161829",
                    "0xc15a6d29bd19deb4",
                    "0x16fec5f9b0a9c150",
                    "0x91362da0dd4c12d2",
                    "0x877d5c894a296bc7",
                    "0x35257f27c9fdd6f1",
                    "0x9a070d0e4978dcaa",
                    "0xa5bda01e9f65353e",
                    "0x6e92838b63658f24",
                    "0x410f62481ad5c003",
                    "0xbb95f88562580e35",
                    "0x75a00ed4fe2db2e1",
                    "0x32cc7dbfa0f57286",
                    "0x913eccf1ac9bf3d1",
                    "0x19e0ef8f3b2d15a1",
                    "0xa0ada351e9840c6b",
                    "0x67703d2f34ab0b27",
                    "0x1a0821d7af850adf",
                    "0x4d1ce181a39b2fbe",
                    "0x81e77483eae39676",
                    "0x5ee2fcf3b5e3eba0",
                    "0x1c8efef6585d3b55",
                    "0x6f67bb365c75bba2",
                    "0xb3bc96adaf2ccdad",
                    "0xd7444721b047a03d",
                    "0xd4d56f7206408e94",
                    "0x210990c361f1b157",
                    "0xe634b39482eb654f",
                    "0xd7faab2012a02c89",
                    "0x9fccc1361e65064e",
                    "0x425f40c69b964f8c",
                    "0x6e538a9ab5b00878",
                    "0x80178695"
                ],
                vec![
                    "0x1b6654a9a01102f9",
                    "0xdd306fa8aaf27490",
                    "0x23ea3310660c0972",
                    "0xf56818f8f4d553bd",
                    "0x449617a04462a32f",
                    "0x459bf09cda11b93b",
                    "0xeca7293b3ced0c14",
                    "0x9e304840068d713a",
                    "0x85c9a09417180c08",
                    "0x3b26b2c4530e0aeb",
                    "0x3487ab99bb39e182",
                    "0x5f7dc9a7839fe1af",
                    "0xd6a09ba906fcb5b5",
                    "0x3e9cb1e4b9a7138c",
                    "0x6a9ee728ffc1c9f8",
                    "0xfb3257d3c6f810a0",
                    "0xa0e914f4484ee94c",
                    "0x2f982d14fc62452f",
                    "0x20f340b14cd994f6",
                    "0x891a0647f679e0be",
                    "0x9fdca6dc8c6767d8",
                    "0x6be8c1bef65c8ca0",
                    "0x4b0501a6b2776038",
                    "0xec304fa56de1c068",
                    "0xf82f0767f87f994f",
                    "0x24d3b6ff363a0d2",
                    "0xa7f415878662560c",
                    "0x8fef2359e2bd194",
                    "0x40b3a85c3bf5b039",
                    "0x7a73298295a0ce26",
                    "0x5f4a72f7d0263b3c",
                    "0xa5b898a14a3f9c8b",
                    "0x175aed5e14cd96df",
                    "0x10a5f086a006f431",
                    "0x6cd056488685627a",
                    "0xca20982c852ae49f",
                    "0xed39a3154211cd3e",
                    "0xa6785a07fa9b912",
                    "0x191afcac17ee726d",
                    "0xfdbc306959010055",
                    "0xa4bd6089f1525c8c",
                    "0x3cfa0f1ee9fc355",
                    "0x9ea940b91df69cb3",
                    "0x8e65d8c9e12c5a65",
                    "0x8c4a410d34f7d042",
                    "0x7ea0beb1abf7addc",
                    "0x6c5905274423ec61",
                    "0x74827ded936a88e9",
                    "0x6150269854961878",
                    "0xa01981a60f636091",
                    "0x7a0f7c470b3ea02",
                    "0x2464d214d648c75b",
                    "0x2a441b4472b6702",
                    "0x302d96bc1e940ddc",
                    "0x90200570c29b6ca0",
                    "0x15848cd19e8edf0",
                    "0x64b72bb1e25926",
                    "0x8fa39ca1af01f5f7",
                    "0x94aea7bdb7aba025",
                    "0x27fa0aca8d0b29e",
                    "0x784068deca45cbed",
                    "0x571d6ef726da5ab7",
                    "0xb97493ba45a0734c",
                    "0xf90866f736138f8c",
                    "0xac8d97ab4d984e1f",
                    "0xb41f0dbcfdd34754",
                    "0x804e693c"
                ],
                vec![
                    "0xee810531a01102f9",
                    "0xa87f620742356be0",
                    "0x2da7a7be46748b4f",
                    "0x851e8d726cceca57",
                    "0x47a5a7a03ae6cf8c",
                    "0xcc89fbfd449cfaca",
                    "0x182644b27e8c6d14",
                    "0x21521fefebca6365",
                    "0x3f25a00cf07471a5",
                    "0xf4a765bdbbcebba6",
                    "0x6f1106e802f07d2d",
                    "0xd5508e133a887a35",
                    "0x1ca049c93e1c4200",
                    "0xea119532a838eeba",
                    "0x182ca830a7637569",
                    "0xfa3b4a08d82f5fd2",
                    "0xa08d0c260471bde6",
                    "0x9127b62a99605a1b",
                    "0x89ca4f7a694e8ca",
                    "0x5e060de55e59e19",
                    "0x6bded6e45ff7d1f7",
                    "0xff8721ed5daba6a0",
                    "0xb7adbb752775894c",
                    "0x9f97dcf3bcc553a0",
                    "0x9a72610929c07b44",
                    "0xf005b3279bdea0ea",
                    "0x672e578f613ff734",
                    "0x94fac5dd8bcc5aa0",
                    "0x1288a90fd25771c0",
                    "0xd821a746d5a00497",
                    "0xba602a25dd1a207",
                    "0x4fdaa95209a4ba3a",
                    "0xae8dc27bb209aa64",
                    "0x41c07c5da0fb4a81",
                    "0xecbfa80d1b6e9507",
                    "0x24bbecdde0cbdd0a",
                    "0x9a004bfea0583bf5",
                    "0xb73d83a03f0bae3b",
                    "0x22fae3a13e48f7f",
                    "0x82d84ca13d6ecb39",
                    "0xf31e0fc7f4ca12f4",
                    "0xb5daa032322d9a2e",
                    "0xe3c764bfda9a9f92",
                    "0x4d8d5cb81b5c547c",
                    "0x83697c35a8e3de8d",
                    "0xd1a0815522ceab12",
                    "0x44ef8ce0bedc619b",
                    "0x5f728c33e46135ea",
                    "0xc54e0a0105c1108a",
                    "0xa0a9517227659533",
                    "0x4390f61f76f9c277",
                    "0x481a516b25da57b0",
                    "0x50aa4d11d9dd360d",
                    "0xc9075354eba08ed5",
                    "0xe0e774af756400a0",
                    "0x947ef053d1649ff0",
                    "0x82071f0befd47827",
                    "0xcdb63f2aee5b3f73",
                    "0x2309dd2919aaa020",
                    "0x3bd27d529f57f358",
                    "0x7d5a5c27b220891b",
                    "0xc1c71f4fae53fefe",
                    "0x893b664d3da0f575",
                    "0x893e4533331086b0",
                    "0x685f3c2153fdfd2c",
                    "0x1c60cf6fff25beb2",
                    "0x803195bb"
                ],
                vec![
                    "0xd9d41313a01102f9",
                    "0xc880b365d7c6f149",
                    "0xa1504add6dc36751",
                    "0x5f7c80062263a5a3",
                    "0x747101a0eab884bb",
                    "0xd2ceb1b6eea62d65",
                    "0xa6e84a2a32018434",
                    "0x1f4d465f98e05849",
                    "0xafb7a025ac54ec6c",
                    "0x591dc30753d135e5",
                    "0xda88d0462ab86ae4",
                    "0x1a40bd8f6b8cb21f",
                    "0x29a00ab1a20bf0d0",
                    "0x21d8fc09f7b0974d",
                    "0x721ebbc37446197b",
                    "0xde9d7a15728aacf1",
                    "0xa0326c0dcc3a1ce3",
                    "0x9aa91f735d43c65",
                    "0xa8b3442a79906ded",
                    "0xe542f57affe04b20",
                    "0x67cd6d6c4a782a98",
                    "0xa0283b715f0eeba0",
                    "0x8d613d8130b3dab7",
                    "0xebb862a4be2f1a17",
                    "0x2b62b088998f9848",
                    "0x676b050f564ca026",
                    "0xe5cf489df069430c",
                    "0x2734c81ed7bd4ab6",
                    "0xec339891d6e75908",
                    "0x9fdd18f72ca06104",
                    "0xa06f484db1811e2f",
                    "0x21c2a0537b14aaf2",
                    "0xae17f4a437239a17",
                    "0xf04e7e98a0bd638d",
                    "0x6d65c84f2f1343bc",
                    "0x405c458f6ca4d3f6",
                    "0xfb765ba98acebb01",
                    "0x7bfa61a088b30ef6",
                    "0x824e83b030e14616",
                    "0x5679a84f7e09178c",
                    "0xbc57b52be7c9eb88",
                    "0xc23da0ead254282a",
                    "0xb3b10ce6f3deea6f",
                    "0xa72f5d9820cae32a",
                    "0x7f2883701572a8fd",
                    "0xf0a038549a9d7cf4",
                    "0x15edc274d8c76fd7",
                    "0xd22c754b1f3faa5f",
                    "0xd047fe17ebe46144",
                    "0xa017f3d907c1ac50",
                    "0x9b3adc3bbcbb2a09",
                    "0x5b5cf48e7d8c5b5",
                    "0x304e60f13ac0724b",
                    "0x89e8dd5079e9f7ca",
                    "0x8afd6e16f5f1dda0",
                    "0xbd33e5ef4491a2fd",
                    "0x7a06777224804145",
                    "0xa34f451a5025c3ca",
                    "0x1e68831260dca0e8",
                    "0x8afa1e27b958a7b2",
                    "0xd41326f81daeea63",
                    "0xc854fc5c42cf5957",
                    "0x46574c98b6a0b640",
                    "0x72f11d0b56a944a1",
                    "0x478c197a6abb83f",
                    "0xc81fb53c5801efb1",
                    "0x800cc38d"
                ],
                vec![
                    "0xcc970e2a01102f9",
                    "0x5fb1b876d7729fb4",
                    "0x53ddf180f41a4cbc",
                    "0xc2f84a626f89775d",
                    "0x2b35e2a007368b98",
                    "0xfb87b8397af58936",
                    "0x2344235941040e66",
                    "0x14554ae09ba520e6",
                    "0xf64fa0cdb1660c9e",
                    "0x557657c85e51d431",
                    "0xfaff6c599d396e83",
                    "0xcf965cf7dcb2f53b",
                    "0xeda08c9d36537854",
                    "0x39a0b9cd489965e6",
                    "0x96c1619afb5f7ac3",
                    "0x17b53c7768f6a635",
                    "0xa0ffde6db48bd642",
                    "0x42ab07417f9eac5f",
                    "0x378008ec7e700f66",
                    "0x3c70d750b0c1f84f",
                    "0xf8dfec7dfe1c7a11",
                    "0x69afa2de717c1fa0",
                    "0x3a66e402a63c519e",
                    "0x5b70bfed7e633786",
                    "0xc2f3453176e1704e",
                    "0xc421e2750afaa097",
                    "0xf5f7fd0ea881ad80",
                    "0xc52134b8f04b1c86",
                    "0xf6be2e1900d7e40c",
                    "0x9da9f92fc1a077fb",
                    "0xc3a62697c3acd8f",
                    "0xea3ff124002048de",
                    "0x86ba9f6d4b8eb108",
                    "0x6c38922ea0c7f246",
                    "0x6b8350267b5f8dbf",
                    "0x1f50bff85b5019a6",
                    "0x4bf43f104ae6b999",
                    "0x30e20da0012e145f",
                    "0x54e27af0caf21960",
                    "0x72cc54834a65e644",
                    "0x1e70c7faab75ef47",
                    "0x5bf2a06e3ee87512",
                    "0xbc2269dad356d37c",
                    "0x199b8f11781e763f",
                    "0x15173800fc10e6e0",
                    "0x9fa07572755b5e69",
                    "0xc0f9d4528bb873b4",
                    "0xb6c41baa6fa05f7",
                    "0x1548a2200a0ad642",
                    "0xa07f9ff5adf1eb7f",
                    "0xaa00a41b742eee28",
                    "0x13262e23e30572e2",
                    "0x45a1513fd6f54079",
                    "0x835aa141bae5c74",
                    "0xa4bc969d3b84c5a0",
                    "0x3c36874331d2fe82",
                    "0x9703b3de43590203",
                    "0x6c3d5b58fe885e33",
                    "0x9474a38f9689a065",
                    "0x924c1b4b5e3da152",
                    "0x3c1268ac210f0f33",
                    "0x213ec07566e00487",
                    "0xc41df5c131a0a5e3",
                    "0x60e6555ed2a740d7",
                    "0x743c58f0b23f06f7",
                    "0x24b0d24f21374df",
                    "0x808f95c3"
                ],
                vec![
                    "0xd85aa080809101f9",
                    "0x58121261eb5e8cf5",
                    "0xbef267395c101325",
                    "0xf500a3e167c430c",
                    "0x80800d1d6a37993e",
                    "0xe4d83ce7eb3970a0",
                    "0x3153936a1e4525a7",
                    "0xc1799b89a0b7c44e",
                    "0xb8d8131fae4b4092",
                    "0xf935c1e3c42a0f9",
                    "0xbed634b1cff544a0",
                    "0xa294d241fb09abff",
                    "0x4b5706ae99473997",
                    "0x3be76ba020a0c872",
                    "0xb1086ebc3a29a279",
                    "0x5f70448e4dfa210",
                    "0x27c970c120125cff",
                    "0xc4fb79a0195b51",
                    "0xccf717b4c9e90131",
                    "0xd3854de2156a44c",
                    "0xc03f2e14ccf0273",
                    "0xae60cfa0eaf6cdfa",
                    "0x4a305f92154d17be",
                    "0xf8a4322b16f62682",
                    "0x352bb09d3c15f317",
                    "0x69cea0d66b49959f",
                    "0x52d61c8705ed9f8f",
                    "0x3e5d1a4c81ee045",
                    "0x3fa96814b5e1b061",
                    "0x1ea06dbae89cf112",
                    "0xc003db2232551c3",
                    "0x228777f7eaf50ffd",
                    "0x8dda3cdd55fce53d",
                    "0xa07779034a639237",
                    "0xd58c186570a9c08b",
                    "0x620179aebbc0e31",
                    "0x77047fc467c7e651",
                    "0x588c9bc084424035",
                    "0x6a3bade9f54f86a0",
                    "0x9e2fc575552e6076",
                    "0x1f4509759f6723c1",
                    "0x211a0852e1b28ca9",
                    "0xeb2023f1d5a6a0b5",
                    "0xf62fff971d7ee6fc",
                    "0x39fb973ae8b821cd",
                    "0xa5fafb8d82960446",
                    "0x5489685d04a01e36",
                    "0xf385edbc49e9f035",
                    "0x3feaa2e6a76dc319",
                    "0x6df284226345ffd5",
                    "0x809ae7df"
                ],
                vec![
                    "0x80808080808051f8",
                    "0xcc713c7ba0808080",
                    "0xd3bc795c81288381",
                    "0x9b92de9b7817c744",
                    "0xca368ae2bf30bf23",
                    "0x80808080cd72ad3c",
                    "0x6df54e694fe0b0a0",
                    "0x95aa91a12f3c8c45",
                    "0x2f9a4193a3202a86",
                    "0x664c86759e00be1f",
                    "0x80802d"
                ],
                vec![
                    "0x8c16ff413d9d71f8",
                    "0x7edba2efc434cbfc",
                    "0xd48f93c369c364f",
                    "0xa582fbb6f28628c1",
                    "0x89781a824ff851b8",
                    "0xbba4efb8faa4202",
                    "0xcc1b171fe856a0c4",
                    "0xc092e64583ffa655",
                    "0x6c991be0485b6ef8",
                    "0x63e3b52f6201c0ad",
                    "0x860146d2c5a021b4",
                    "0xdcb27d7e923c23f7",
                    "0xca53b600e5c003c7",
                    "0x5d04d8fa7b3b2782",
                    "0x70a485"
                ]
            ]
        );

        assert_eq!(
            formatted_account.account_key,
            Uint256 {
                low: "0x7edba2efc434cbfc8c16ff41ed16e54e".to_string(),
                high: "0xa582fbb6f28628c10d48f93c369c364f".to_string()
            }
        );
    }

    #[test]
    fn test_split128_mpt_proof_key() {
        let account_key_result = split_little_endian_hex_into_parts(
            "0x4ee516ed41ff168cfccb34c4efa2db7e4f369c363cf9480dc12886f2b6fb82a5",
        );
        assert_eq!(
            account_key_result,
            Uint256 {
                low: "0x7edba2efc434cbfc8c16ff41ed16e54e".to_string(),
                high: "0xa582fbb6f28628c10d48f93c369c364f".to_string()
            }
        );
    }

    #[test]
    fn test_cairo_format_storage() {
        let original_storage = Storage {
            address: "0x75cec1db9dceb703200eaa6595f66885c962b920".to_string(),
            slot: "0x0000000000000000000000000000000000000000000000000000000000000003".to_string(),
            storage_key: "0xc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b"
                .to_string(),
            proofs: vec![MPTProof {
                block_number: 4952100,
                proof: vec![],
            }],
        };

        let formatted_storage = original_storage.to_cairo_format();
        assert_eq!(
            formatted_storage.address,
            vec![
                "0x3b7ce9ddbc1ce75".to_string(),
                "0x8568f69565aa0e20".to_string(),
                "0x20b962c9".to_string()
            ]
        );

        assert_eq!(
            formatted_storage.slot,
            vec![
                "0x0".to_string(),
                "0x0".to_string(),
                "0x0".to_string(),
                "0x300000000000000".to_string()
            ]
        );

        assert_eq!(
            formatted_storage.storage_key,
            Uint256 {
                low: "0x28db122fc9f859f9003c599e0e5a57c2".to_string(),
                high: "0x5bf8716f4416255ed002053b5a39c369".to_string()
            }
        );
    }

    #[test]
    fn test_split128_root() {
        // cross checking with solidity :
        // uint256 taskMerkleRoot = uint256(
        //     bytes32(
        //         0x730f1037780b3b53cfaecdb95fc648ce719479a58afd4325a62b0c5e09e83090
        //     )
        // );
        // (uint256 taskRootLow, uint256 taskRootHigh) = Uint256Splitter.split128(
        //     taskMerkleRoot
        // );
        // uint128 scheduledTasksBatchMerkleRootLow = 0x719479a58afd4325a62b0c5e09e83090;
        // uint128 scheduledTasksBatchMerkleRootHigh = 0x730f1037780b3b53cfaecdb95fc648ce;
        // assertEq(scheduledTasksBatchMerkleRootLow, taskRootLow);
        // assertEq(scheduledTasksBatchMerkleRootHigh, taskRootHigh);
        let task_root = split_big_endian_hex_into_parts(
            "0x730f1037780b3b53cfaecdb95fc648ce719479a58afd4325a62b0c5e09e83090",
        );
        assert_eq!(
            task_root,
            Uint256 {
                low: "0x719479a58afd4325a62b0c5e09e83090".to_string(),
                high: "0x730f1037780b3b53cfaecdb95fc648ce".to_string()
            }
        );
    }

    #[test]
    fn cairo_format_tasks() {
        let original_task = Task{
            encoded_task: "0x23c69fe8ceb11087e27f0b0a89d8dc0cda85ab933464d49bd21d623526acf8c073756d000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000000".to_string(),
            task_commitment: "0x46296bc9cb11408bfa46c5c31a542f12242db2412ee2217b4e8add2bc1927d0b".to_string(),
            compiled_result: "6776".to_string(),
            result_commitment: "0x40b44b4fe85644ce1d6f8f5035a5f7e91861d064ed0b4189ebb2eb6ce8985f4d".to_string(),
            task_proof: vec![],
            result_proof: vec![],
            encoded_datalake: "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004b902400000000000000000000000000000000000000000000000000000000004b9024000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000016027f2c6f930306d3aa736b3a6c6a98f512f74036d40000000000000000000000".to_string(),
            datalake_type: 0,
            property_type: 2
        };

        let formatted_task = original_task.to_cairo_format();

        assert_eq!(
            formatted_task.encoded_task,
            vec![
                "0x8710b1cee89fc623",
                "0xcdcd8890a0b7fe2",
                "0x9bd4643493ab85da",
                "0xc0f8ac2635621dd2",
                "0x6d7573",
                "0x0",
                "0x0",
                "0x0",
                "0x0",
                "0x0",
                "0x0",
                "0x6000000000000000",
                "0x0",
                "0x0",
                "0x0",
                "0x0"
            ]
        );

        assert_eq!(formatted_task.task_bytes_len, 128);

        assert_eq!(
            formatted_task.encoded_datalake,
            vec![
                "0x0",
                "0x0",
                "0x0",
                "0x0",
                "0x0",
                "0x0",
                "0x0",
                "0x24904b0000000000",
                "0x0",
                "0x0",
                "0x0",
                "0x24904b0000000000",
                "0x0",
                "0x0",
                "0x0",
                "0x100000000000000",
                "0x0",
                "0x0",
                "0x0",
                "0xa000000000000000",
                "0x0",
                "0x0",
                "0x0",
                "0x1600000000000000",
                "0xd30603936f2c7f02",
                "0xf5986a6c3a6b73aa",
                "0xd43640f712",
                "0x0"
            ]
        );

        assert_eq!(formatted_task.datalake_bytes_len, 224);
    }
}
