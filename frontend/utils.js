import { connect, Contract, keyStores } from 'near-api-js'
import getConfig from './config'

// NEAR
import { Wallet } from './near-wallet';

const CONTRACT_NAME = process.env.CONTRACT_NAME
const nearConfig = getConfig(process.env.NODE_ENV || 'development')

// When creating the wallet you can optionally ask to create an access key
// Having the key enables to call non-payable methods without interrupting the user to sign
const wallet = new Wallet({ createAccessKeyFor: CONTRACT_NAME })

// Initialize contract & set global variables
export async function initContract() {
  // Initialize connection to the NEAR testnet
  const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig))

  // Getting the Account ID. If still unauthorized, it's just empty string
  wallet.accountId = window.wallet.getAccountId()

  // Initializing our contract APIs by contract name and configuration
  window.contract = await new Contract(window.wallet.startUp(), nearConfig.contractName, {
    // View methods are read only. They don't modify the state, but usually return some value.
    viewMethods: ["event_count"],
    // Change methods can modify the state. But you don't receive the returned value when called.
    changeMethods: ['add_event', 'add_vote', "list_events"],
    // add more functions above when developing the smart contract.
  })
}

export function logout() {
  window.wallet.signOut()
  // reload page
  window.location.replace(window.location.origin + window.location.pathname)
}

export function login() {
  // Allow the current app to make calls to the specified contract on the
  // user's behalf.
  // This works by creating a new access key for the user's account and storing
  // the private key in localStorage.
  window.wallet.requestSignIn(nearConfig.contractName)
}