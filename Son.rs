object SuiNFT has store {
  nfts: object_table<TokenID, NFT>
}

struct NFT {
  owner: address
  metadata: Option<Vec<u8>>
}

fun init(): SuiNFT {
  return SuiNFT { nfts: object_table::empty() }
}

fun mint(owner: address, metadata: Option<Vec<u8>>): TokenID {
  let token_id = object_table::next_available_key(SuiNFT.nfts);
  let nft = NFT { owner, metadata };
  SuiNFT.nfts.insert(token_id, nft);
  emit MintEvent(token_id);
  return token_id;
}

fun transfer(from: address, to: address, token_id: TokenID) {
  let mut nft = SuiNFT.nfts.get(token_id).unwrap();
  assert(nft.owner == from);
  nft.owner = to;
  SuiNFT.nfts.update(token_id, nft);
  emit TransferEvent(from, to, token_id);
}

event MintEvent(token_id: TokenID)
event TransferEvent(from: address, to: address, token_id: TokenID)
