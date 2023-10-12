import React, { useState } from 'react';

function PinDisplay({ pin }) {
  
  return (
    <section id="about-boxes" className="about-boxes">

      <br></br>
      <div className="container calenderly" data-aos="fade-up">
      <br></br>
      <div className="calendar-container">
      <h1 id='calender-title'>Your Partage Pin</h1>
      <div className="pin-display">
      <h5>Save this Pin or take a screenshot of it as your asset maybe lost when you lost your pin</h5>
      <p>{pin}</p>
    </div>
      
    
    </div>
      </div>
    </section>
    
    
    
  );
}

export default PinDisplay;
