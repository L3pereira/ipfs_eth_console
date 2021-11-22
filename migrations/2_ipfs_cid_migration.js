const IpfsCid = artifacts.require("IpfsCid");

module.exports = function (deployer) {
  deployer.deploy(IpfsCid);
};
