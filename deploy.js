module.exports = async deployer => {
  return deployer.deploy("./pkg/erc20.wasm");
};
