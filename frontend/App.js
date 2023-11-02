import "regenerator-runtime/runtime";
import React, { useState, useEffect } from "react";
import { useHistory } from "react-router-dom";

import About from "./components/About"
import Calendar from "./components/Calendar";
import Contact from "./components/contact";
import Footer from "./components/footer";
import Header from "./components/header";
import Services from "./components/Services";
import Connectedhead from "./components/Connectedhead";
import GLightbox from "glightbox";
import Swiper from "swiper";
import Isotope from "isotope-layout";
import AOS from "aos";

import "./assets/css/style.css";
import "./assets/vendor/aos/aos.css";
import "./assets/vendor/bootstrap/css/bootstrap.min.css";
import "./assets/vendor/bootstrap-icons/bootstrap-icons.css";
import "./assets/vendor/boxicons/css/boxicons.min.css";
import "./assets/vendor/glightbox/css/glightbox.min.css";
import "./assets/vendor/remixicon/remixicon.css";
import "./assets/vendor/swiper/swiper-bundle.min.css";


const App = ({ isSignedIn, lockCalendar, wallet }) => {
  

  AOS.init({
    duration: 1000,
    easing: "ease-in-out",
    once: true,
    mirror: false,
  });
  const glightbox = GLightbox({
    selector: ".glightbox",
  });

  const [bookings, setBookings] = useState([]);

  useEffect(() => {
    lockCalendar.getBookings().then(setBookings);
  }, []);

  // add booking function
  async function addBooking (e) {
    e.preventDefault();
    const { name, numberOfDays, totalPrice, description, pin } = e.target.elements;

    // use the wallet to send the booking to the contract
    await lockCalendar.addBooking(
        name.value,
        numberOfDays.value,
        totalPrice.value,
        description.value,
        pin.value
        )
    const bookings = await lockCalendar.getBookings()

    setBookings(bookings);
    name.value = '';
    numberOfDays.value = '';
    totalPrice.value = '0';
    description.value = '';
    pin.value = '';
    name.focus();
    numberOfDays.focus();
    totalPrice.focus();
    description.focus();
    pin.focus();

  };

  // If user not signed-in show landingpage
  if (!isSignedIn) {
    // Sign-in flow will reload the page later
    return (
      <main>
        <Header onClick={() => wallet.signIn()} />
        <About />
        <Services />
        <Contact />
        <Footer />
      </main>
    );
  }

  // If user signed-in show booking calendar
  return (
    <main>
      <Connectedhead onClick={() => wallet.signOut()} />
      <Calendar onSubmit={addBooking} currentAccountId={wallet.accountId} />
      { !!bookings.length && <Bookings bookings={bookings}/> }
      <Footer />
    </main>
  );
}

export default App;
