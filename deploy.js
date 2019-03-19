module.exports = async deployer => {
  return deployer.deploy("./pkg/hello_world_bg.wasm");
};
