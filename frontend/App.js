import 'regenerator-runtime/runtime'
import { useEffect, useState } from 'react'
import ListEvents from './components/ListEvents.js'
import CreateEvent from './components/CreateEvents.js'
import React from 'react'
import { login, logout } from './utils'
import './assets/global.css'

import getConfig from './config'

const { networkId } = getConfig(process.env.NODE_ENV || 'development')

export default function App() {

  const [events, setEvents] = useState([])
  const [toggleModal, setToggleModal] = useState(false)


  function addProject() {
    setToggleModal(!toggleModal)
  }

  
  useEffect(
    () => {

      if (window.walletConnection.isSignedIn()) {

        window.contract.list_events().then((eventprojects) => {
          const eventList = [...eventprojects]
          setEvents(eventList)
        })
      }
    },

    [],
  )

  if (!window.walletConnection.isSignedIn()) {
    return (
      <main className='signin'>
        <h1>Welcome to Awesome Events</h1>
        <p style={{ textAlign: 'center' }}>
          Click the button below to sign in:
        </p>
        <p style={{ textAlign: 'center', marginTop: '2.5em' }}>
          <button onClick={login}>Sign in</button>
        </p>
      </main>
    )
  }

  return (

    <>
      <header>
        <div className="logo"></div>
        <button className="link" style={{ float: 'right' }} onClick={logout}>
          Sign out <span className="id">{window.accountId}</span>
        </button>
      </header>
      <button onClick={addProject}>Add an event</button>
      <main>
        <CreateEvent toggleModal={toggleModal} />
        <section className='events'>
          {events.map((project, id) => {
            return (
              <div key={id}>
                <ListEvents project={project} />
              </div>
            )
          })}
        </section>
      </main>
    </>
  )
}