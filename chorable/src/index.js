/*
 * index.js
 * Entrypoint for the `Chorable` web app
 *
 */ 


import React             from 'react';
import ReactDOM          from 'react-dom';
import { AppContainer }  from 'react-hot-loader';


import { App }           from './App';


// import global css styles to render on #root
async function renderApp() {
  ReactDOM.render(
    <App Container>
      <App/>
    </App Container>,
    document.querySelector('#root');
  );
};


if (module.hot) {
    module.hot.accept();
    renderApp();
} else {
    renderApp();
}

