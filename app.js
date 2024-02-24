htmx.config.historyCacheSize = 30;
htmx.config.globalViewTransitions = true;

document.addEventListener('htmx:responseError', function(evt) {
  if (evt.detail.xhr.status === 404) {
    const html = `<div class="toast toast-top toast-end"><div class="alert alert-error"><span>${evt.detail.xhr.status}: ${evt.detail.xhr.statusText}</span></div>`;
    document.getElementById('site-htmx-preserve-alert').innerHTML = html;
    setTimeout(() => {
      document.getElementById('site-htmx-preserve-alert').innerHTML = ``;
    }, 5000);
  }
});

htmx.onLoad(function() {
  console.log('-> Loaded htmx');
});

window.app = {};

window.app.setThemeButtonState = (theme) => {
  document.getElementById('theme-name-dark').classList.remove('swap-off');
  document.getElementById('theme-name-dark').classList.remove('swap-on');
  document.getElementById('theme-name-light').classList.remove('swap-off');
  document.getElementById('theme-name-light').classList.remove('swap-on');
  if (theme === 'dark') {
    document.getElementById('theme-name-light').classList.add('swap-on');
  } else {
    document.getElementById('theme-name-dark').classList.add('swap-on');
  }
}

window.app.setCurrentTheme = (theme) => {
  document.documentElement.setAttribute('data-theme', theme);
  localStorage.setItem('color-theme', theme);
  window.app.setThemeButtonState(theme);
}

window.app.toggleTheme = function() {
  if (localStorage.getItem('color-theme')) {
    if (localStorage.getItem('color-theme') === 'light') {
      window.app.setCurrentTheme('dark');
    } else {
      window.app.setCurrentTheme('light');
    }
  } else {
    if (document.documentElement.getAttribute('data-theme') === 'dark') {
      window.app.setCurrentTheme('light');
    } else {
      window.app.setCurrentTheme('dark');
    }
  }
}

console.log('-> Loaded app.js');

