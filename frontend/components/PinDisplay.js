import React, { useState } from "react";

// function PinDisplay({ pin }) {
function PinDisplay() {
  return (
    <div className="container calenderly" data-aos="fade-up">
      <br></br>
      <div className="calendar-container">
        <h1 id="calender-title">Your Partage Pin</h1>
        <h5>
          Save this Pin or take a screenshot of it as your asset maybe lost when
          you lost your pin
        </h5>
        <div className="pin-display">
          {/* <p>{pin}</p> */}
          <p>1234</p>
        </div>
      </div>
    </div>
  );
}

export default PinDisplay;
