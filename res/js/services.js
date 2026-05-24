//////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Services page support
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

var sp_data = {};

function showServicePanel(elem, key) {
  console.log("showServicePanel(" + elem + ', ' + key + ")");
  let current_key = data[]
  let panel = document.getElementById("service-panel");
  if (panel.style.visibility == 'hidden') {
    panel.style.visibility = 'visible';
    // populateServicePanel (key);
  } else if (panel.style.visibility == 'visible') {
    // if (current_key == key) {
      panel.style.visibility = 'hidden';
    // } else {
    //   populateServicePanel (key);
    // }
  } else {
    console.log('panel.style.visibility = ' + panel.style.visibility);
  }
}

function populateServicePanel (key) {
  let headUrl = '/pass/page/services/service-panel/service/' + key

  let serviceCallback = function (obj) {
    sp_data[key] = obj;    

    rePopulateervicePanelData(obj);
  }

  processGetRequest(headUrl, serviceCallback);
}

function  rePopulateervicePanelData (obj) {
  let spId = document.getElementById('service-panel-id');
  let spName = document.getElementById('service-panel-id');
  let spVersion = document.getElementById('service-panel-id');

  spId.innerText = obj.svc_id;
  spName.innerText = obj.svc_name;
  spVersion.innerText = obj.version;
}
