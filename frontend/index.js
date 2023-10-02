// React
import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import { initContract } from './utils'

// Setup on page load
window.onload = async () => {
  const isSignedIn = await initContract()

    ReactDOM.render(
      <App isSignedIn={isSignedIn} contractId={CONTRACT_NAME} wallet={wallet} />,
      document.getElementById('root')
    );
  }