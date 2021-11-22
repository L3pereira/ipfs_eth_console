pragma solidity >=0.4.22 <0.9.0;

contract IpfsCid {
  struct Multihash {
    bytes32 hash;
    bytes2 hash_function;
    uint8 size;
  }

  string cid;

  Multihash multihashCid;

  function storeCIDAsStruct(bytes32 _hash, bytes2 _hash_function, uint8 _size) public {
    multihashCid.hash = _hash;
    multihashCid.hash_function = _hash_function;
    multihashCid.size = _size;
  }

  function storeCIDAsString(string memory _cid) public {
    cid = _cid;
  }
  //View functions ensure that they will not modify the state
  function getCIDAsString() public view returns (string memory) {
    return cid;
  }
  //View functions ensure that they will not modify the state
  function getCIDAsStruct() public view returns (Multihash memory) {
    return multihashCid;
  }

}