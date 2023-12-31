import React from 'react';
import boxImage1 from '../assets/img/connect-wallet.jpg';
import boxImage2 from '../assets/img/send-payment.jpg';
import boxImage3 from '../assets/img/receive-password.jpg';

const Services = () => {
  return (
    <section id="about-boxes" className="about-boxes">
      <div className="container" data-aos="fade-up">
        <div className="section-title">
          <h2>How It Works</h2>
        </div>

        <div className="row">
          <div
            className="col-lg-4 col-md-6 d-flex align-items-stretch"
            data-aos="fade-up"
            data-aos-delay="100"
          >
            <div className="card">
              <img
                src={boxImage1}
                className="card-img-top"
                alt="..."
              />
              <div className="card-icon">
                <i className="ri-brush-4-line"></i>
              </div>
              <div className="card-body">
                <h5 className="card-title"><a href="">Connect Your Wallet</a></h5>
                <p className="card-text">
                  Connect your preferred Near wallet by clicking the Connect
                  Wallet button at the top of this page.
                </p>
              </div>
            </div>
          </div>
          <div
            className="col-lg-4 col-md-6 d-flex align-items-stretch"
            data-aos="fade-up"
            data-aos-delay="200"
          >
            <div className="card">
              <img
                src={boxImage2}
                className="card-img-top"
                alt="..."
              />
              <div className="card-icon">
                <i className="ri-calendar-check-line"></i>
              </div>
              <div className="card-body">
                <h5 className="card-title"><a href="">Send payment</a></h5>
                <p className="card-text">
                  Browse all available locked Utilities in the Partage Marketplace, and send payment for your desired use time.
                </p>
              </div>
            </div>
          </div>
          <div
            className="col-lg-4 col-md-6 d-flex align-items-stretch"
            data-aos="fade-up"
            data-aos-delay="300"
          >
            <div className="card">
              <img
                src={boxImage3}
                className="card-img-top"
                alt="..."
              />
              <div className="card-icon">
                <i className="ri-movie-2-line"></i>
              </div>
              <div className="card-body">
                <h5 className="card-title"><a href="">Collect password</a></h5>
                <p className="card-text">
                  Collect your unique password and type it on the lock to access the utility of your choice.
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Services;
