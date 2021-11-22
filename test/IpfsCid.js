const IpfsCid = artifacts.require("IpfsCid");
const multihashes = require('multihashes');
const toHexString = arr => Array.from(arr, i => i.toString(16).padStart(2, "0")).join("");
const fromHexString = hexString => new Uint8Array(hexString.match(/.{1,2}/g).map(byte => parseInt(byte, 16)));
const cid_str = "QmTJd6JnxTGrLgJqLfnhMHXaytrSaBHvos4ECeVTvqwHdi";

contract("IpfsCid", accounts => {

    it('Store the IPFS CID as a string', async () => {
        const ipfs_cid = await IpfsCid.new();
        let txReceipt = await ipfs_cid.storeCIDAsString(cid_str);
        let gasUsed = txReceipt.receipt.gasUsed;
        console.log("gasUsed: " + gasUsed + " units");
        const cid = await ipfs_cid.getCIDAsString();
        assert(cid.toString() === cid_str);
    });



    it('Store the IPFS CID as a struct', async () => {

        let mh = multihashes.fromB58String(Buffer.from(cid_str));
        console.log(mh);

        let args = {
          hashFunction: '0x' + toHexString(mh.slice(0, 2)),
          hash: '0x' + toHexString(mh.slice(2)),
          size: mh.length - 2
        }
        console.log(args);
        const ipfs_cid = await IpfsCid.new();
        let txReceipt = await ipfs_cid.storeCIDAsStruct(args.hash, args.hashFunction, args.size);
   
        let gasUsed = txReceipt.receipt.gasUsed;
        const cid = await ipfs_cid.getCIDAsStruct();
        //convert cid struct to multihash uint8array and then to B58 string
        let hashFunctionBytes = fromHexString(cid.hash_function.toString(16)).slice(1);        
        let hashBytes = fromHexString(cid.hash.toString(16)).slice(1);

        var mergedArray = new Uint8Array(hashFunctionBytes.length + hashBytes.length);
        mergedArray.set(hashFunctionBytes);
        mergedArray.set(hashBytes, hashFunctionBytes.length);
 
        assert(multihashes.toB58String(mergedArray) === cid_str);
        console.log("gasUsed: " + gasUsed + " units");

    });

});
