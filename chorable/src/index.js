/*
 * index.js
 * Entrypoint for the `Chorable` web app
 *
 */ 


import React             from 'react';
import ReactDOM          from 'react-dom';
import { AppContainer }  from 'react-hot-loader';


import App from './App';


// import global css styles to render on #root
function renderApp() {
  ReactDOM.render(
    <AppContainer>
      <App />
    </AppContainer>,
    document.getElementById('root')
  );
};

if (module.hot) {
    module.hot.accept();
    renderApp();
} else {
    renderApp();
}

