const Web3 = require('web3');
const Migrations = artifacts.require("Migrations");
const TruffleConfig = require('../truffle-config.js');

module.exports = function(deployer, network, accounts) {
    const config = TruffleConfig.networks[network];
    const opts = { gas: 6e6, from: accounts[0] };
    
    if (process.env.ACCOUNT_PASSWORD) {
        const web3 = new Web3(new Web3.providers.HttpProvider('http://' + config.host + ':' + config.port));
        console.log('Unlocking account: ' + accounts[0]);
        web3.personal.unlockAccount(accounts[0], process.env.ACCOUNT_PASSWORD, 36000);
    } else {
        throw new Error('provide ACCOUNT_PASSWORD ev var');
    }
    

    deployer.deploy(Migrations, opts);
};
