# ipfs_eth_console
Upload a file to IPFS and then store the CID in a smart contract

<h2>Tech Stack</h2>

<ul>
  <li>Async (Tokio Runtime)</li>
  <li>Truffle</li>
  <li>Ganache</li>
  <li>Actix</li>
  <li>Solidity</li>
  <li>IPFS</li>
</ul>

Rust v1.53.0 (53cb7b09b 2021-06-17)<br/>
go-ipfs version: 0.10.0<br/>
Truffle v5.4.19<br/>
Solc 0.8.10<br/>



"npm install truffle -g"<br/>
"npm install"

to test the contracts enter<br/>
"truffle test"

Open powershell or terminal, then go where you installed the go-ipfs and initiate your node<br/>
"ipfs daemon"

Open the ganache UI and check if the server is running<br/>
Ganache UI <br/>
RPC Server: HTTP://127.0.0.1:7545<br/>
If your Ganache RPC Server is diferent, please update the "ganache" section in the truffle-config.js file.

To compile and deploy the contracts in the ganache blockchain, enter the following command<br/>
"truffle migrate --compile-all --reset --network ganache"

Now you should have in the ganache UI a set of wallets addresses and a contract address.<br/>
Use a wallet and the contract address in the config.json 

ex:<br/>
{<br/>
    "web3_transport": "http://127.0.0.1:7545",<br/>
    "contract_address":"d5f9b7c42d683Dd33A8E4E3318f599A678303ba3",<br/>
    "wallet_address":"56ddE94C59FE43dc889dF94E841441364ab66b9B"<br/>
}<br/>

Now type in the terminal (app directory) run the app (cargo run)

------Please insert a file path--------<br/>
Insert a file path<br/>
"C:\Users\User\test.txt"<br/>

If it fails to find the file, returns to the same msg. If you want to quit just escape the terminal<br/>

After that, the file will be uploaded to the IFPS, the next msg will be shown<br/>
File uploaded successfully to IPFS, cid is: "Qmxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"<br/>

Then The app checks in the smart contract, if this cid is already there, this step is to save gas,<br/>
if it already exists, will show the following msgs.

This cid is already stored in the smart contract as string, don't waste gas!!!<br/>
This cid is already stored in the smart contract as multihash struct, don't waste gas!!!<br/>

------Please Insert--------<br/>
------1 to store as a string--------<br/>
------2 to store as a multihash struct--------<br/>
------q to exit--------<br/>

You can save the cid as a string or as a struct, in the truffle tests the struct saved more gas.

If the cid is successfully saved you will see the following:

Transaction hash: 0x9a52e8169b6b6413a0f2c51e16513600fdd9e61a7eda8de834fcade4550029f2<br/>
Gas used: 26914<br/>
Account Balance: 99972343960000000000<br/>
Quiting
