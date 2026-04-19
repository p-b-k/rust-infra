////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Basic JS Functionality
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

function processGetRequest (url, callback) {
  fetch(url).then(response => { response.json().then(callback); });
}

function load_sidebar(sidebarDiv) {
  let pagesCallback = function (obj) {
    console.log("pages = " + obj);

    for (i = 0; i < obj.length; i++) {
      var e = obj[i];
      var t = e[0];
      var p = e[1];
      console.log('processing page ' + p.name);

      console.log('image source = ' + p.icon);

      let a = document.createElement('a');
      a.href = '/page/' + t;

      let img = document.createElement('img');
      img.src = '/static/svg/pages/beige/' + p.icon;
      img.width = 28;
      img.height = 28;
      img.title = p.name

      a.appendChild(img);
      sidebarDiv.appendChild(a);
    }
  }

  console.log('Sidebar load called on sidebar div ' + sidebarDiv);

  processGetRequest('/pages', pagesCallback);
}
